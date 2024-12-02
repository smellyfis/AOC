#![warn(clippy::all, clippy::pedantic)]

use error_stack::{Report, Result, ResultExt};
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
        fn internal_check(levels: &[u32]) -> Safety {
            let mut dir = Ordering::Equal;
            for i in 1..levels.len() {
                if !(1_u32..=3).contains(&(levels[i - 1].abs_diff(levels[i]))) {
                    return Safety::UnSafe;
                }
                let new_dir = levels[i - 1].cmp(&levels[i]);
                if dir != Ordering::Equal && dir != new_dir {
                    return Safety::UnSafe;
                }
                dir = new_dir;
            }
            Safety::Safe
        }
        let ret = internal_check(&self.levels);
        if ret == Safety::Safe {
            return ret;
        }
        for i in 0..self.levels.len() {
            let mut attempt = self.levels.clone();
            let _ = attempt.remove(i);
            let ret = internal_check(&attempt);
            if ret == Safety::Safe {
                return ret;
            }
        }
        Safety::UnSafe
    }
}

// day-2
#[derive(Debug, Error)]
pub enum Day2Part2Error {
    #[error("Problem parsing Day 2")]
    ParseError,
}

/// Day-2 Part 2 for 2024 advent of code
/// Problem can be found here: <https://adventofcode.com/2024/day/2#part2>
///
/// # Errors
/// - `ParseError` there was an issue with the parser
pub fn part2(input: &str) -> Result<String, Day2Part2Error> {
    let (_, reports) = parse_input(input)
        .map_err(|x| Report::from(x.to_owned()))
        .change_context(Day2Part2Error::ParseError)?;
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
    #[case("1 3 2 4 5", Safety::Safe)]
    #[case("8 6 4 4 1", Safety::Safe)]
    #[case("1 3 6 7 9", Safety::Safe)]
    #[case("5 3 6 7 9", Safety::Safe)]
    #[case("5 3 6 7 6", Safety::UnSafe)]
    #[case("8 6 4 6 4", Safety::UnSafe)]
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
    fn part2_works() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, "4".to_string());
    }
}

