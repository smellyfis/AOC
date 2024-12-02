#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashSet;

use glam::UVec2;
use nom::{
    bytes::complete::is_a, character::complete, multi::separated_list1, sequence::tuple, IResult,
};

struct Drawing {
    pub size: UVec2,
    pub mounds: HashSet<UVec2>,
}

impl Drawing {
    fn process(&self) -> u32 {
        let (max_col, max_row) = self.size.into();
        let col_score = (1..max_col)
            .filter(|reflect_col| {
                let reflect_col = *reflect_col;
                let span = reflect_col.min(max_col - reflect_col);
                self.mounds
                    .iter()
                    .filter(|mound| mound.x + span >= reflect_col && mound.x < reflect_col)
                    .map(|mound| (2 * reflect_col - mound.x - 1, mound.y).into())
                    .all(|mound_reflect| self.mounds.contains(&mound_reflect))
                    && self
                        .mounds
                        .iter()
                        .filter(|mound| mound.x < reflect_col + span && mound.x >= reflect_col)
                        .map(|mound| (2 * reflect_col - mound.x - 1, mound.y).into())
                        .all(|mound_reflect| self.mounds.contains(&mound_reflect))
            })
            .sum::<u32>();
        let row_score = (1..max_row)
            .filter(|reflect_row| {
                let reflect_row = *reflect_row;
                let span = reflect_row.min(max_row - reflect_row);
                self.mounds
                    .iter()
                    .filter(|mound| mound.y + span >= reflect_row && mound.y < reflect_row)
                    .map(|mound| (mound.x, 2 * reflect_row - mound.y - 1).into())
                    .all(|mound_reflect| self.mounds.contains(&mound_reflect))
                    && self
                        .mounds
                        .iter()
                        .filter(|mound| mound.y < reflect_row + span && mound.y >= reflect_row)
                        .map(|mound| (mound.x, 2 * reflect_row - mound.y - 1).into())
                        .all(|mound_reflect| self.mounds.contains(&mound_reflect))
            })
            .sum::<u32>()
            * 100;
        col_score + row_score
    }
}

/// day 13 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part1(input: &str) -> String {
    let (_, drawings) = parse_input(input).expect("aoc always valid");
    drawings
        .iter()
        .map(Drawing::process)
        .sum::<u32>()
        .to_string()
}

fn parse_drawing(input: &str) -> IResult<&str, Drawing> {
    let (input, rows) = separated_list1(complete::line_ending, is_a(".#"))(input)?;
    let max_rows = u32::try_from(rows.len()).expect("shouldn't be that big");
    let max_cols = u32::try_from(rows[0].len()).expect("shouldn't be that big");
    let size = UVec2::from((max_cols, max_rows));
    let mounds = rows
        .iter()
        .enumerate()
        .flat_map(|y| {
            y.1.chars().enumerate().map(move |x| {
                (
                    u32::try_from(x.0).expect("should be 32"),
                    u32::try_from(y.0).expect("should not fail"),
                    x.1,
                )
            })
        })
        .filter_map(|(x, y, mound)| (mound == '#').then_some(UVec2::from((x, y))))
        .collect();
    Ok((input, Drawing { size, mounds }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Drawing>> {
    separated_list1(
        tuple((complete::line_ending, complete::line_ending)),
        parse_drawing,
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        5
    )]
    #[case(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        400
    )]
    fn board_test(#[case] input: &str, #[case] expected: u32) {
        let (_, drawing) = parse_drawing(input).expect("Parsing should work");
        assert_eq!(drawing.process(), expected);
    }

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "405".to_string());
    }
}
