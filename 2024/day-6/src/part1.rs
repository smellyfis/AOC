#![warn(clippy::all, clippy::pedantic)]

use error_stack::Result;
use thiserror::Error;

// day-6
#[derive(Debug, Error)]
pub enum Day6Part1Error{
    #[error("Problem parsing Day 6")]
    ParseError,
}

/// Day-6 Part 2 for 2024 advent of code
/// Problem can be found here: <https://adventofcode.com/2024/day/6>
///
/// # Errors
/// - `ParseError` there was an issue with the parser
pub fn part1 (_input: &str) -> Result<String, Day6Part1Error> {
    Ok("Not Finished".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part1_works() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, "Not Finished".to_string());
    }
}

