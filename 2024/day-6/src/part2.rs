#![warn(clippy::all, clippy::pedantic)]

use error_stack::Result;
use thiserror::Error;

// day-6
#[derive(Debug, Error)]
pub enum Day6Part2Error{
    #[error("Problem parsing Day 6")]
    ParseError,
}

/// Day-6 Part 2 for 2024 advent of code
/// Problem can be found here: <https://adventofcode.com/2024/day/6#part2>
///
/// # Errors
/// - `ParseError` there was an issue with the parser
pub fn part2 (_input: &str) -> Result<String, Day6Part2Error> {
    Ok("Not Finished".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part2_works() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, "Not Finished".to_string());
    }
}

