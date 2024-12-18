#![warn(clippy::all, clippy::pedantic)]

use day_22::part1;
use day_22::part2;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("./input.txt");
    let part1_result = part1(input);
    println!("part 1: {part1_result}");
    let part2_result = part2(input);
    println!("part 2: {part2_result}");
}
