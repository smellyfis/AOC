#![warn(clippy::all, clippy::pedantic)]

use std::fs;

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (comp1, comp2) = line.split_at(line.len() / 2);
            let duplicate = comp2.chars().find(|c| comp1.contains(*c)).unwrap();
            match duplicate {
                n @ 'a'..='z' => (n as i32) - ('a' as i32) + 1_i32,
                n @ 'A'..='Z' => (n as i32) - ('A' as i32) + 27_i32,
                _ => 0,
            }
        })
        .sum::<i32>()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .fold(Vec::new(), |mut acc: Vec<Vec<String>>, line| {
            if acc.is_empty() || acc.last().unwrap().len() == 3 {
                acc.push(Vec::new());
            }
            acc.last_mut().unwrap().push(line.to_owned());
            acc
        })
        .iter()
        .map(|group| {
            let [g1, g2, g3] = group.as_slice() else {
                panic!("not get here")
            };
            match g1
                .chars()
                .fold(Vec::new(), |mut combo: Vec<char>, ch| {
                    if g2.contains(ch) {
                        combo.push(ch);
                    }
                    combo
                })
                .iter()
                .find(|c| g3.contains(**c))
                .unwrap()
            {
                n @ 'a'..='z' => (*n as i32) - ('a' as i32) + 1_i32,
                n @ 'A'..='Z' => (*n as i32) - ('A' as i32) + 27_i32,
                _ => 0,
            }
        })
        .sum::<i32>()
        .to_string()
}

fn main() -> std::io::Result<()> {
    //Read in file
    let file = fs::read_to_string("input")?;

    // fold the lines into groups of three
    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
    // find common letter in the groups
    //   find common letters in the first 2 then find the common in the third
    // sum the common letters
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        assert_eq!(part1(&INPUT), "157");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&INPUT), "70");
    }
}
