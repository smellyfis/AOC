#![warn(clippy::all, clippy::pedantic)]

use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::opt,
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct ParseTypeError;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
impl FromStr for Type {
    type Err = ParseTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),
            _ => Err(ParseTypeError),
        }
    }
}

#[derive(Debug)]
struct ItemMapEntry {
    pub to: u64,
    pub from: u64,
    pub count: u64,
}

impl ItemMapEntry {
    fn to_out(&self, from: u64) -> Option<u64> {
        if from < self.from || self.from + self.count < from {
            None
        } else {
            Some(self.to + (from - self.from))
        }
    }
}

#[derive(Debug)]
struct ItemMap {
    pub from_type: Type,
    pub to_type: Type,
    pub mapping: Vec<ItemMapEntry>,
}

impl ItemMap {
    fn map(&self, from: u64) -> u64 {
        self.mapping
            .iter()
            .find_map(|x| x.to_out(from))
            .or(Some(from))
            .expect("always")
    }
}
/// part1 of day 5 of AOC 2023
///
/// # Arguments
/// - input the puszzle input
///
/// # Panics
/// panics whenever the input isn't parsable

#[must_use]
pub fn part1(input: &str) -> String {
    let (_input, (mut to_process, maps)) = parse_input(input).expect("aoc always has input");
    //println!("{_input}");
    let mut from_type = Type::Seed;
    while from_type != Type::Location {
        let current_map = maps
            .iter()
            .find(|x| x.from_type == from_type)
            .expect("should always find");
        to_process = to_process
            .iter()
            .map(|x| current_map.map(*x))
            .collect::<Vec<_>>();
        //println!("{to_process:#?}");
        from_type = current_map.to_type;
    }
    //println!("{to_process:#?}");
    to_process
        .iter()
        .min()
        .expect("always a number")
        .to_string()
}

fn parse_item_map_entry(input: &str) -> IResult<&str, ItemMapEntry> {
    let (input, to) = complete::u64(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, from) = complete::u64(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, count) = complete::u64(input)?;
    Ok((input, ItemMapEntry { to, from, count }))
}

fn parse_to_from(input: &str) -> IResult<&str, (Type, Type)> {
    let (input, (to_type, from_type)) =
        separated_pair(complete::alpha1, tag("-to-"), complete::alpha1)(input)?;
    Ok((
        input,
        (
            to_type.parse().expect("there will be a to type"),
            from_type.parse().expect("there will be a from type"),
        ),
    ))
}

fn parse_map(input: &str) -> IResult<&str, ItemMap> {
    let (input, (from_type, to_type)) =
        terminated(parse_to_from, tuple((complete::space1, tag("map:"))))(input)?;
    let (input, _) = complete::line_ending(input)?;
    let (input, mapping) = separated_list1(complete::line_ending, parse_item_map_entry)(input)?;
    let (input, _) = opt(complete::line_ending)(input)?;
    Ok((
        input,
        ItemMap {
            from_type,
            to_type,
            mapping,
        },
    ))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = complete::space1(input)?;
    separated_list1(complete::space1, complete::u64)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<ItemMap>)> {
    let (input, seeds) = terminated(parse_seeds, complete::line_ending)(input)?;
    let (input, _) = complete::line_ending(input)?;
    let (input, maps) = separated_list1(complete::line_ending, parse_map)(input)?;
    Ok((input, (seeds, maps)))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "35".to_string());
    }
}
