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

/// day 8 part 2 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part2(input: &str) -> String {
    let (_, (steps, branches)) = parse_input(input).expect("aoc expects valid input");

    let starting_node: Vec<&str> = branches
        .keys()
        .map(String::as_str)
        .filter(|x| x.ends_with('A'))
        .collect();

    let cycles = starting_node
        .iter()
        .map(|node| {
            let mut visited_nodes = vec![*node];
            let mut current = *node;
            steps
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(i, direction)| {
                    let new = branches.get(current).expect("aoc1").choose(*direction);
                    if new.ends_with('Z') {
                        return Some(i + 1);
                    }
                    visited_nodes.push(new);
                    dbg!(current = new);
                    None
                })
                .expect("aoc4")
        })
        .collect::<Vec<_>>();
    lcm(&cycles).to_string()
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
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
            complete::alphanumeric1,
            pair(tag(","), complete::space1),
            complete::alphanumeric1,
        ),
        pair(complete::space0, tag(")")),
    )(input)?;
    let left = left.to_string();
    let right = right.to_string();
    Ok((input, Branches { left, right }))
}

fn parse_nodes(input: &str) -> IResult<&str, (String, Branches)> {
    let (input, (node, branches)) = separated_pair(
        complete::alphanumeric1,
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
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        "6"
    )]

    fn part2_works(#[case] input: &str, #[case] expected: &str) {
        let result = part2(input);
        assert_eq!(result, expected);
    }
}
