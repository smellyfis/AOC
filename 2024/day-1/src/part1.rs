#![warn(clippy::all, clippy::pedantic)]

use error_stack::Result;
use nom::{character::complete, multi::separated_list1, sequence::separated_pair, IResult};
use thiserror::Error;

// day-1
#[derive(Debug, Error)]
pub enum Day1Part1Error {
    #[error("Problem parsing Day 1")]
    ParseError,
}

pub fn part1(input: &str) -> Result<u64, Day1Part1Error> {
    let (_, (mut col1, mut col2)) =
        parse_input(input).expect("there should always be valid input from aoc");
    col1.sort();
    col2.sort();

    Ok(col1
        .into_iter()
        .zip(col2.iter())
        .map(|(a, b)| u64::max(a, *b) - u64::min(a, *b))
        .sum())
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, combo) = separated_list1(
        complete::line_ending,
        separated_pair(complete::u64, complete::space1, complete::u64),
    )(input)?;
    Ok((input, combo.into_iter().unzip()))

}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3 ";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part1_works() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, 11);
    }
}

