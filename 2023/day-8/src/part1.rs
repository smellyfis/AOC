#![warn(clippy::all, clippy::pedantic)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::{many1, separated_list1},
    sequence::{delimited, pair, separated_pair, tuple},
    IResult, Parser,
};
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Branches {
    pub left: String,
    pub right: String,
}

impl Branches {
    fn choose(&self, direction: Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

/// day 4 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part1(input: &str) -> String {
    let (_, (steps, branches)) = parse_input(input).expect("aoc expects valid input");

    let mut current = "AAA";
    let mut count = 0_usize;
    for x in steps.iter().cycle() {
        if current == "ZZZ" {
            break;
        }
        current = branches.get(current).expect("aoc").choose(*x);
        count += 1;
    }
    count.to_string()
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, directions) = many1(alt((
        tag("L").map(|_| Direction::Left),
        tag("R").map(|_| Direction::Right),
    )))(input)?;
    let (input, _) = complete::line_ending(input)?;
    Ok((input, directions))
}

fn parse_branches(input: &str) -> IResult<&str, Branches> {
    let (input, (left, right)) = delimited(
        pair(tag("("), complete::space0),
        separated_pair(
            complete::alpha1,
            pair(tag(","), complete::space1),
            complete::alpha1,
        ),
        pair(complete::space0, tag(")")),
    )(input)?;
    let left = left.to_string();
    let right = right.to_string();
    Ok((input, Branches { left, right }))
}

fn parse_nodes(input: &str) -> IResult<&str, (String, Branches)> {
    let (input, (node, branches)) = separated_pair(
        complete::alpha1,
        tuple((complete::space1, tag("="), complete::space1)),
        parse_branches,
    )(input)?;
    Ok((input, (node.to_string(), branches)))
}

fn parse_node_tree(input: &str) -> IResult<&str, BTreeMap<String, Branches>> {
    let (input, map) = separated_list1(complete::line_ending, parse_nodes)(input)?;
    let map = map.into_iter().collect();
    Ok((input, map))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Direction>, BTreeMap<String, Branches>)> {
    let (input, x) =
        separated_pair(parse_directions, complete::line_ending, parse_node_tree)(input)?;
    Ok((input, x))
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        "2"
    )]
    #[case(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        "6"
    )]

    fn part1_works(#[case] input: &str, #[case] expected: &str) {
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
