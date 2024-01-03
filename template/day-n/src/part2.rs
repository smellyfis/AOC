#![warn(clippy::all, clippy::pedantic)]

use std::fmt::Display;

use error_stack::{Context, Result};

#[derive(Debug)]
pub struct {{ project-name | upper_camel_case }}Part2Error;

impl Context for {{ project-name | upper_camel_case }}Part2Error {}

impl Display for {{ project-name | upper_camel_case }}Part2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "day 1 part 1 error")
    }
}

pub fn part2 (_input: &str) -> Result<String, {{ project-name | upper_camel_case }}Part2Error> {
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

