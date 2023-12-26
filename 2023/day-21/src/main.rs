#![warn(clippy::all, clippy::pedantic)]

use day_21::part1;
use day_21::part2;

fn main() {
    let input = include_str!("./input.txt");
    let part1_result = part1(input, 64);
    println!("part 1: {part1_result}");
    let part2_result = part2(input, 26_501_365);
    println!("part 2: {part2_result}");
}
