#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use error_stack::{Report, Result, ResultExt};
use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use thiserror::Error;

// day-5
#[derive(Debug, Error)]
pub enum Day5Part1Error {
    #[error("Problem parsing Day 5")]
    ParseError,
}

type Orderings = HashMap<u32, Vec<u32>>;

/// Day-5 Part 1 for 2024 advent of code
/// Problem can be found here: <https://adventofcode.com/2024/day/5>
///
/// # Errors
/// - `ParseError` there was an issue with the parser
pub fn part1(input: &str) -> Result<String, Day5Part1Error> {
    //parse into "bad list where X|Y
    let (_, (ordering, updates)) = parse_input(input)
        .map_err(|x| Report::from(x.to_owned()))
        .change_context(Day5Part1Error::ParseError)?;
    let middles: u32 = updates
        .iter()
        .filter_map(|update| {
            let update_len = update.len();
            for i in 0..update_len {
                let before = &update[..i];
                if let Some(a) = update.get(i) {
                    if let Some(rules) = ordering.get(a) {
                        if rules.iter().any(|b| before.contains(b)) {
                            return None;
                        }
                    }
                }
            }
            Some(update[update_len / 2])
        })
        .sum();
    Ok(middles.to_string())
}

fn parse_ordering(input: &str) -> IResult<&str, Orderings> {
    let (input, rules) = separated_list1(
        complete::line_ending,
        separated_pair(complete::u32, tag("|"), complete::u32),
    )(input)?;
    let ordering = rules.iter().fold(HashMap::new(), |mut acc: Orderings, (a, b)| {
        acc.entry(*a).or_default().push(*b);
        acc
    });
    Ok((input, ordering))
}

fn parse_update(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32)(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(complete::line_ending, parse_update)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Orderings, Vec<Vec<u32>>)> {
    let (input, ordering) = parse_ordering(input)?;
    let (input, _) = complete::line_ending(input)?;
    let (input, _) = complete::line_ending(input)?;
    let (input, updates) = parse_updates(input)?;
    Ok((input, (ordering, updates)))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part1_works() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, "143".to_string());
    }
}

