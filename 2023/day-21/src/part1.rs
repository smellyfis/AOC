#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::terminated,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

fn next_step(loc: IVec2, boulders: &HashSet<IVec2>) -> Vec<IVec2> {
    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        .iter()
        .map(|dir| loc + *dir)
        .filter(|loc| boulders.get(loc).is_none())
        .collect()
}

/// day 21 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part1(input: &str, steps: u32) -> String {
    let (_, (start, boulders)) = parse_input(Span::from(input)).expect("AOC input should be valid");
    let mut current = [start].into_iter().collect::<HashSet<_>>();
    for _i in 0..steps {
        current = current
            .iter()
            .flat_map(|loc| next_step(*loc, &boulders))
            .unique()
            .collect::<HashSet<_>>();
    }
    current.len().to_string()
}

fn with_xy(span: Span) -> SpanIVec2 {
    let x = i32::try_from(span.get_column()).expect("overflow") - 1;
    let y = i32::try_from(span.location_line()).expect("wrap around") - 1;
    span.map_extra(|()| IVec2::new(x, y))
}

fn parse_input(input: Span) -> IResult<Span, (IVec2, HashSet<IVec2>)> {
    fold_many1(
        terminated(
            many1(alt((tag("S"), tag("."), tag("#"))).map(with_xy)),
            alt((complete::line_ending, eof)),
        ),
        || (IVec2::splat(0), HashSet::new()),
        |(mut start, mut set), row| {
            for spot in row {
                if spot.fragment() == &"S" {
                    start = spot.extra;
                }
                if spot.fragment() == &"#" {
                    set.insert(spot.extra);
                }
            }
            (start, set)
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    #[rstest]
    #[case(6, "16")]
    fn part1_works(#[case] steps: u32, #[case] expected: &str) {
        let result = part1(INPUT, steps);
        assert_eq!(result, expected.to_string());
    }
}
