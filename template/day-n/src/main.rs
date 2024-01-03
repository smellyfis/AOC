#![warn(clippy::all, clippy::pedantic)]

use {{crate_name}}::part1;
use {{crate_name}}::part2;

fn main() {
    let input = include_str!("./input.txt");
    let part1_result = part1(input).expect("part 1 should have no error");
    println!("part 1: {part1_result}");
    let part2_result = part2(input).expect("part 2 should have no error");
    println!("part 2: {part2_result}");
}
