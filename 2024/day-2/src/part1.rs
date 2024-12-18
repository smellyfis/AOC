#![warn(clippy::all, clippy::pedantic)]

use error_stack::{Report, Result, ResultExt};
use itertools::Itertools;
use nom::{character::complete, multi::separated_list1, IResult};
use std::cmp::Ordering;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
enum Safety {
    Safe,
    UnSafe,
}

#[derive(Debug, PartialEq, Eq)]
struct XmasReport {
    levels: Vec<u32>,
}

impl XmasReport {
    pub fn is_safe(&self) -> Safety {
        let mut dir = Ordering::Equal;
        for (a, b) in self.levels.iter().tuple_windows(){
            if !(1_u32..=3).contains(&(a.abs_diff(*b))) {
                return Safety::UnSafe;
            }
            let new_dir = a.cmp(b);
            if dir != Ordering::Equal && dir != new_dir {
                return Safety::UnSafe;
            }
            dir = new_dir;
        }
        Safety::Safe
    }
}

// day-2
#[derive(Debug, Error)]
pub enum Day2Part1Error {
    #[error("Problem parsing Day 2")]
    ParseError,
}

/// Day-3 Part 1 for 2024 advent of code
/// Problem can be found here: <https://adventofcode.com/2024/day/3>
///
/// # Errors
/// - `ParseError` there was an issue with the parser
pub fn part1(input: &str) -> Result<String, Day2Part1Error> {
    let (_, reports) = parse_input(input)
        .map_err(|x| Report::from(x.to_owned()))
        .change_context(Day2Part1Error::ParseError)?;
    Ok(reports
        .iter()
        .filter(|x| x.is_safe() == Safety::Safe)
        .count()
        .to_string())
}

fn parse_level(input: &str) -> IResult<&str, XmasReport> {
    let (input, v) = separated_list1(complete::space1, complete::u32)(input)?;
    Ok((input, XmasReport { levels: v }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<XmasReport>> {
    separated_list1(complete::line_ending, parse_level)(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("7 6 4 2 1", Safety::Safe)]
    #[case("1 2 7 8 9", Safety::UnSafe)]
    #[case("9 7 6 2 1", Safety::UnSafe)]
    #[case("1 3 2 4 5", Safety::UnSafe)]
    #[case("8 6 4 4 1", Safety::UnSafe)]
    #[case("1 3 6 7 9", Safety::Safe)]
    fn part1_report_safety(#[case] input: &str, #[case] expected: Safety) {
        let (_, tester) = parse_level(input).expect("should be valid input");
        assert_eq!(tester.is_safe(), expected);
    }

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part1_works() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, "2".to_string());
    }
}

