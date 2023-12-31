#![warn(clippy::all, clippy::pedantic)]

use std::{fmt::Display, ops::Not};

use error_stack::{Result, Context, Report};
use log::trace;

#[derive(Debug)]
pub struct Day1Part2Error;

impl Context for Day1Part2Error{}

impl Display for Day1Part2Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "day 1 part 2 error")
    }
}

/// Day 1 Part 2 of AOC2023
///
/// # Arguments
/// - puzzle input
///
/// # Errors
/// this panics if there is no numbers in a line
pub fn part2(input: &str) -> Result<String, Day1Part2Error> {
    let values = input.lines().map(parse_line).collect::<Result<Vec<Vec<u32>>,_>>()?;
    trace!("{values:?}");
    values
        .iter()
        .map(|v| {
            v.first().and_then(|first| if let Some(last) = v.last() { Some(*first *10 + *last) } else {None}).ok_or(Day1Part2Error)
        })
        .fold(Ok(0_u32), | sum, number |{
            let Ok(sum) = sum else {return Err(Report::from(Day1Part2Error))};
            let Ok(number) = number else { return Err(Report::from(Day1Part2Error))};
            Ok(sum + number)
        })
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
                reduced_line
                    .chars()
                    .next()
                    .and_then(|x| x.to_digit(10))
            };

            result
        })
        .collect();
    numbers.is_empty().not().then_some(numbers).ok_or(Report::from(Day1Part2Error))
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
