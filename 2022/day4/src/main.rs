#![warn(clippy::all, clippy::pedantic)]

use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list0,
    sequence::separated_pair,
};

fn parse_range(input: &str) -> nom::IResult<&str, (i32, i32)> {
    separated_pair(complete::i32, tag("-"), complete::i32)(input)
}
fn parse_line(input: &str) -> nom::IResult<&str, ((i32, i32), (i32, i32))> {
    separated_pair(parse_range, tag(","), parse_range)(input)
}
fn process_input(input: &str) -> nom::IResult<&str, Vec<((i32, i32), (i32, i32))>> {
    separated_list0(newline, parse_line)(input)
}

fn part1(input: &str) -> String {
    do_part(input, |(a, b)| {
        i32::from((a.0 <= b.0 && a.1 >= b.1) || (a.0 >= b.0 && a.1 <= b.1))
    })
}
fn part2(input: &str) -> String {
    do_part(input, |(a, b)| {
        i32::from(
            (a.0 >= b.0 && a.0 <= b.1)
                || (a.1 >= b.0 && a.1 <= b.1)
                || (b.0 >= a.0 && b.0 <= a.1)
                || (b.1 >= a.0 && b.1 <= a.1),
        )
    })
}

fn do_part(input: &str, f: impl Fn((&(i32, i32), &(i32, i32))) -> i32) -> String {
    let (_, ranges) = process_input(input).unwrap();
    ranges
        .iter()
        .map(|(a, b)| f((a, b)))
        .fold(0, |mut acc: i32, x: i32| {
            acc += x;
            acc
        })
        .to_string()
}
fn main() -> std::io::Result<()> {
    //Read in file
    let file = fs::read_to_string("input")?;

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), "2")
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), "4")
    }
}
