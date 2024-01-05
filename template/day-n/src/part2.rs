#![warn(clippy::all, clippy::pedantic)]

use error_stack::Result;
use thiserror::Error;

// {{project-name}}
#[derive(Debug, Error)]
pub enum {{ project-name | upper_camel_case }}Part2Error{
    #[error("Problem parsing {{project-name | title_case }}")]
    ParseError,
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

