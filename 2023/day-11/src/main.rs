#![warn(clippy::all, clippy::pedantic)]

use day_11::part1;
use day_11::part2;

fn main() {
    let input = include_str!("./input.txt");
    let part1_result = part1(input);
    println!("part 1: {part1_result}");
    let part2_result = part2(input, 1_000_000);
    println!("part 2: {part2_result}");
}
