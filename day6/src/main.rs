#![warn(clippy::all, clippy::pedantic)]

use std::fs;

use itertools::Itertools;

fn work(input: &str, count: usize) -> String {
    let array = input.as_bytes();
    for x in count..array.len() {
        if array
            .iter()
            .enumerate()
            .filter_map(|(pos, val)| {
                if pos >= x - count && pos < x {
                    Some(val)
                } else {
                    None
                }
            })
            .unique()
            .count()
            == count
        {
            return x.to_string();
        }
    }
    panic!("stuff should have been gotten {input}")
}

fn part1(input: &str) -> String {
    work(input, 4)
}

fn part2(input: &str) -> String {
    work(input, 14)
}

fn main() -> () {
    //Read in file
    let file = fs::read_to_string("input").unwrap();

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[(&str, &str, &str)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7", "19"),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", "5", "23"),
        ("nppdvjthqldpwncqszvftbrmjlhg", "6", "23"),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10", "29"),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11", "26"),
    ];

    #[test]
    fn part1_works() {
        INPUT
            .iter()
            .for_each(|(test, ans1, _)| assert_eq!(part1(*test), *ans1))
    }

    #[test]
    fn part2_works() {
        INPUT
            .iter()
            .for_each(|(test, _, ans2)| assert_eq!(part2(*test), *ans2))
    }
}
