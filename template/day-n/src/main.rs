#![warn(clippy::all, clippy::pedantic)]

use {{ crate_name }}::part1;
use {{ crate_name }}::part2;

use error_stack::{Result, ResultExt};
use thiserror::Error;

#[derive(Debug, Error)]
enum {{ project-name | upper_camel_case }}Error {
    #[error("Part 1 failed")]
    Part1Error,
    #[error("Part 2 failed")]
    Part2Error,
}

fn main() -> Result<(), {{ project-name | upper_camel_case }}Error> {
    let input = include_str!("./input.txt");
    let part1_result = part1(input).change_context({{ project-name | upper_camel_case }}Error::Part1Error)?;
    println!("part 1: {part1_result}");
    let part2_result = part2(input).change_context({{ project-name | upper_camel_case }}Error::Part2Error)?;
    println!("part 2: {part2_result}");
    Ok(())
}
