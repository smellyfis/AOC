#![warn(clippy::all, clippy::pedantic)]

use day_5::part1;
use day_5::part2;

use error_stack::{Result, ResultExt};
use thiserror::Error;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Debug, Error)]
enum Day5Error {
    #[error("Part 1 failed")]
    Part1Error,
    #[error("Part 2 failed")]
    Part2Error,
}

fn main() -> Result<(), Day5Error> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("./input.txt");
    let part1_result = part1(input).change_context(Day5Error::Part1Error)?;
    println!("part 1: {part1_result}");
    let part2_result = part2(input).change_context(Day5Error::Part2Error)?;
    println!("part 2: {part2_result}");
    Ok(())
}