#![warn(clippy::all, clippy::pedantic)]

use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fs,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    Parser,
};

#[derive(Debug, Eq)]
pub enum Checker {
    Num(u32),
    Array(Vec<Checker>),
}

impl Checker {
    /// # Errors
    ///
    /// returns an `nom::err::Error<&str>` if there is problems parsing
    pub fn parse(input: &str) -> nom::IResult<&str, Self, nom::error::Error<&str>> {
        alt((
            complete::u32.map(Self::Num),
            delimited(tag("["), separated_list0(tag(","), Self::parse), tag("]")).map(Self::Array),
        ))(input)
    }
}

impl PartialEq for Checker {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Num(l0), Self::Num(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::Num(l0), Self::Array(r0)) => &vec![Self::Num(*l0)] == r0,
            (Self::Array(l0), Self::Num(r0)) => l0 == &vec![Self::Num(*r0)],
        }
    }
}

impl PartialOrd for Checker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Checker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Array(a), Self::Array(b)) => a.cmp(b),
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
            (Self::Array(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (Self::Num(a), Self::Array(b)) => vec![Self::Num(*a)].cmp(b),
        }
    }
}

pub struct Together {
    pub left: Checker,
    pub right: Checker,
}

impl Together {
    /// # Errors
    ///
    /// returns an `nom::err::Error<&str>` if there is problems parsing
    pub fn parse(input: &str) -> nom::IResult<&str, Self, nom::error::Error<&str>> {
        separated_pair(Checker::parse, newline, Checker::parse)(input)
            .map(|(input, (left, right))| (input, Self { left, right }))
    }
}

/// # Errors
///
/// returns an `nom::err::Error<&str>` if there is problems parsing
pub fn parse_data(input: &str) -> nom::IResult<&str, Vec<Together>, nom::error::Error<&str>> {
    separated_list1(tag("\n\n"), Together::parse)(input)
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let (_, all) = parse_data(&file).unwrap();
    let part1 = all
        .iter()
        .enumerate()
        .filter_map(|(i, Together { left, right })| match left.cmp(right) {
            Less => Some(i + 1),
            Equal => panic!(),
            Greater => None,
        })
        .sum::<usize>();
    let two = Checker::Array(vec![Checker::Array(vec![Checker::Num(2)])]);
    let six = Checker::Array(vec![Checker::Array(vec![Checker::Num(6)])]);
    let mut p2 = all
        .iter()
        .flat_map(|Together { left, right }| [left, right])
        .chain([&two, &six])
        .collect::<Vec<_>>();
    p2.sort();

    let i2 = p2
        .iter()
        .enumerate()
        .find_map(|(i, b)| if *b == &two { Some(i + 1) } else { None })
        .unwrap();
    let i6 = p2
        .iter()
        .enumerate()
        .find_map(|(i, b)| if *b == &six { Some(i + 1) } else { None })
        .unwrap();
    let part2 = i2 * i6;
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
