#![warn(clippy::all, clippy::pedantic)]

use error_stack::Result;
use thiserror::Error;

// day-3
#[derive(Debug, Error)]
pub enum Day3Part2Error{
    #[error("Problem parsing Day 3")]
    ParseError,
}

pub fn part2 (_input: &str) -> Result<String, Day3Part2Error> {
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

