#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashSet, ops::Not};

use glam::IVec2;
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

fn next_step(
    loc: IVec2,
    size: IVec2,
    boulders: &HashSet<IVec2>,
) -> impl Iterator<Item = IVec2> + '_ {
    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        .iter()
        .map(move |dir| loc + *dir)
        .filter(move |loc| boulders.contains(&(loc.rem_euclid(size))).not())
}

/// day 21 part 2 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part2(input: &str, steps: usize) -> String {
    let (_, (start, size, boulders)) =
        parse_input(Span::from(input)).expect("AOC input should be valid");
    let sq_size = usize::try_from(size.x).unwrap();
    let base = steps % sq_size;
    let reps = steps / sq_size;
    let mut current = [start].into_iter().collect::<HashSet<_>>();
    let mut coef = Vec::new();
    for i in 0..=(base + sq_size * 2 + 1) {
        current = current
            .iter()
            .flat_map(|loc| next_step(*loc, size, &boulders))
            .collect::<HashSet<_>>();

        if i >= base - 1 && (i - base + 1) % sq_size == 0 {
            //println!("{i} - {} - {}", (i - base) / sq_size, current.len());
            coef.push(current.len());
        }
    }

    //TODO assuming this is fit with a quadratic
    let a = (coef[2] - 2 * coef[1] + coef[0]) / 2;
    let b = coef[1] - coef[0] - a;
    let c = coef[0];

    let total = a * reps.pow(2) + b * reps + c;

    total.to_string()
    // TODO this doesn't work for general case
}

fn with_xy(span: Span) -> SpanIVec2 {
    let x = i32::try_from(span.get_column()).expect("overflow") - 1;
    let y = i32::try_from(span.location_line()).expect("wrap around") - 1;
    span.map_extra(|()| IVec2::new(x, y))
}

fn parse_input(input: Span) -> IResult<Span, (IVec2, IVec2, HashSet<IVec2>)> {
    fold_many1(
        terminated(
            many1(alt((tag("S"), tag("."), tag("#"))).map(with_xy)),
            alt((complete::line_ending, eof)),
        ),
        || (IVec2::splat(0), IVec2::splat(0), HashSet::new()),
        |(mut start, mut size, mut set), row| {
            for spot in row {
                if spot.fragment() == &"S" {
                    start = spot.extra;
                }
                if spot.fragment() == &"#" {
                    set.insert(spot.extra);
                }
                size = size.max(spot.extra + 1);
            }
            (start, size, set)
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
    #[case(10, "50")]
    #[case(50, "1594")]
    #[case(100, "6536")]
    #[case(500, "167004")]
    #[case(1000, "668697")]
    #[case(5000, "16733044")]
    fn part2_works(#[case] steps: usize, #[case] expected: &str) {
        let result = part2(INPUT, steps);
        assert_eq!(result, expected.to_string());
    }
}
