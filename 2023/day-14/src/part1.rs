#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;
use nom::{bytes::complete::is_a, character::complete, multi::separated_list1, IResult};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Boulder {
    Round,
    Static,
}
impl From<char> for Boulder {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Round,
            '#' => Self::Static,
            x => unimplemented!("there is no boulder type for this charachter {x}"),
        }
    }
}

/// day 14 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
#[allow(clippy::cast_sign_loss)]
pub fn part1(input: &str) -> String {
    let (_, (maxes, map)) = parse_input(input).expect("stuff");

    (0..maxes.x)
        .map(|col| {
            map.iter()
                .filter(|(key, _)| key.x == col)
                .sorted_by(|(a, _), (b, _)| a.y.cmp(&b.y))
                .fold((0, 0), |(score, last), (pos, boulde)| match boulde {
                    Boulder::Static => (score, pos.y + 1),
                    Boulder::Round => (score + maxes.y - last, last + 1),
                })
                .0 as usize
        })
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, (IVec2, HashMap<IVec2, Boulder>)> {
    let (input, rows) = separated_list1(complete::line_ending, is_a(".O#"))(input)?;
    let max_rows = i32::try_from(rows.len()).expect("stuff and things");
    let max_cols = i32::try_from(rows[0].len()).expect("things and stuff?");
    let maxs = IVec2::from((max_cols, max_rows));
    let hash = rows
        .iter()
        .enumerate()
        .flat_map(|(line_no, chars)| {
            chars
                .chars()
                .enumerate()
                .filter_map(move |(col_no, c)| {
                    (c != '.').then_some((
                        IVec2::from((
                            i32::try_from(col_no).expect("hopefully not to small"),
                            i32::try_from(line_no).expect("this shouldn't be too big"),
                        )),
                        c,
                    ))
                })
                .map(|(pos, c)| (pos, Boulder::from(c)))
        })
        .collect();
    Ok((input, (maxs, hash)))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "136".to_string());
    }
}
