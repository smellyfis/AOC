#![warn(clippy::all, clippy::pedantic)]

use day_5::part1;
use day_5::part2;

fn main() {
    let input = include_str!("./input.txt");
    let part1_result = part1(input);
    println!("part 1: {part1_result}");
    let part2_result = part2(input);
    println!("part 2: {part2_result}");
}
