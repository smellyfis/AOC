#![warn(clippy::all, clippy::pedantic)]

use error_stack::Result;
use thiserror::Error;
use regex::Regex;

// day-3
#[derive(Debug, Error)]
pub enum Day3Part1Error{
    #[error("Problem parsing Day 3")]
    ParseError,
}

pub fn part1 (input: &str) -> Result<String, Day3Part1Error> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Ok(re.captures_iter(input).map(|x| &x[1].parse::<i64>().unwrap() * &x[2].parse::<i64>().unwrap()).sum::<i64>().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part1_works() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, "161".to_string());
    }
}

