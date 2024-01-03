#![warn(clippy::all, clippy::pedantic)]

use std::fmt::Display;

use error_stack::{Context, Result};

// {{project-name}}
#[derive(Debug)]
pub struct {{project-name|upper_camel_case}}Part1Error;

impl Context for {{project-name|upper_camel_case}}Part1Error {}

impl Display for {{project-name|upper_camel_case}}Part1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "day 1 part 1 error")
    }
}

pub fn part1 (_input: &str) -> Result<String, {{project-name|upper_camel_case}}Part1Error> {
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

