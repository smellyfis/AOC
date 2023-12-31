#![warn(clippy::all, clippy::pedantic)]

use std::fmt::Display;

use log::trace;
use nom::{
    self,
    character::complete::{alphanumeric1, newline},
    multi::separated_list1,
};

use error_stack::{Context, Report, Result, ResultExt};

#[derive(Debug)]
pub struct Day1Part1Error;

impl Context for Day1Part1Error {}

impl Display for Day1Part1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "day 1 part 1 error")
    }
}

/// Day-1 part 1 of AC2023
///
/// # Arguments
/// - input the input for day1 as a string
///
/// # Errors
/// errors when can't parse the input
pub fn part1(input: &str) -> Result<String, Day1Part1Error> {
    let (_input, values) = parse_input(input)
        .map_err(|x| Report::from(x.to_owned()))
        .change_context(Day1Part1Error)?;
    trace!("{values:?}");
    values
        .iter()
        .map(|v| {
            v.first()
                .and_then(|first| v.last().map(|last| *first * 10 + *last))
                .ok_or(Day1Part1Error)
        })
        .try_fold(0_u32, |sum, number| {
            let Ok(number) = number else {
                return Err(Report::from(Day1Part1Error));
            };
            Ok(sum + number)
        })
        .map(|x| x.to_string())
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Vec<u32>>> {
    let (i, j) = separated_list1(newline, alphanumeric1)(input)?;
    let res = j
        .iter()
        .map(|v| {
            v.chars()
                .filter_map(|x| x.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    Ok((i, res))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part1_works() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, "142".to_string());
    }
}
