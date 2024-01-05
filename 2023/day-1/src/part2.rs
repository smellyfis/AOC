#![warn(clippy::all, clippy::pedantic)]

use std::ops::Not;

use error_stack::{report, Result};
use log::trace;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day1Part2Error {
    #[error("Problem parsing Day-1 Part2")]
    ParseError,
    #[error("Day 1 Input parsed to Empty")]
    EmptyInput,
}

/// Day 1 Part 2 of AOC2023
///
/// # Arguments
/// - puzzle input
///
/// # Errors
/// this panics if there is no numbers in a line
pub fn part2(input: &str) -> Result<String, Day1Part2Error> {
    let values = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<Vec<u32>>, _>>()?;
    trace!("{values:?}");
    values
        .iter()
        .map(|v| {
            v.first()
                .and_then(|first| v.last().map(|last| *first * 10 + *last))
                .ok_or(report!(Day1Part2Error::EmptyInput))
        })
        .try_fold(0_u32, |sum, number| Ok(sum + number?))
        .map(|x| x.to_string())
}

fn parse_line(line: &str) -> Result<Vec<u32>, Day1Part2Error> {
    let numbers: Vec<u32> = (0..line.len())
        .filter_map(|index| {
            let reduced_line = &line[index..];
            let result = if reduced_line.starts_with("one") {
                Some(1)
            } else if reduced_line.starts_with("two") {
                Some(2)
            } else if reduced_line.starts_with("three") {
                Some(3)
            } else if reduced_line.starts_with("four") {
                Some(4)
            } else if reduced_line.starts_with("five") {
                Some(5)
            } else if reduced_line.starts_with("six") {
                Some(6)
            } else if reduced_line.starts_with("seven") {
                Some(7)
            } else if reduced_line.starts_with("eight") {
                Some(8)
            } else if reduced_line.starts_with("nine") {
                Some(9)
            } else if reduced_line.starts_with("zero") {
                Some(0)
            } else {
                reduced_line.chars().next().and_then(|x| x.to_digit(10))
            };

            result
        })
        .collect();
    numbers
        .is_empty()
        .not()
        .then_some(numbers)
        .ok_or(report!(Day1Part2Error::ParseError))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part2_works() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, "281".to_string());
    }
}
