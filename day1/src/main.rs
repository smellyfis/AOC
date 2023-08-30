#![warn(clippy::all, clippy::pedantic)]

use std::fs;

use itertools::Itertools;
fn part1(input: &[u64]) -> String {
    input.iter().max().unwrap().to_string()
}

fn part2(input: &[u64]) -> String {
    input
        .iter()
        //order the elves since we don't care about position anymore
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .copied()
        .sum::<u64>()
        .to_string()
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().fold(vec![0_u64], |mut acc, line| {
        //empty lines mean new elf
        if line.is_empty() {
            acc.push(0_u64);
        } else {
            // the first time through is an edge case preventing an else here
            let last = acc.last_mut().unwrap();
            *last += line.parse::<u64>().unwrap();
        }
        acc
    })
}
fn main() -> std::io::Result<()> {
    let file = fs::read_to_string("input")?;

    let elves = parse_input(&file);

    //part 1 is get the max
    println!("Part 1: {}", part1(&elves));

    //Part 2 is get the sum of the largest 3
    println!("Part 2: {}", part2(&elves));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_works() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), "24000")
    }

    #[test]
    fn part2_works() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), "45000")
    }
}
