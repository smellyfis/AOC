#![warn(clippy::all, clippy::pedantic)]

use core::ops::Range;
use itertools::Itertools;
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
    pub to: Range<u64>,
    pub from: Range<u64>,
}

#[derive(Debug)]
struct ItemMap {
    pub from_type: Type,
    pub to_type: Type,
    pub mapping: Vec<ItemMapEntry>,
}

impl ItemMap {
    fn input_to_output(&self, input: &Range<u64>) -> Vec<Range<u64>> {
        /*if let Some(within) = self
            .mapping
            .iter()
            .find(|x| x.from.contains(&input.start) && x.from.contains(&input.end))
        {
            //fully contained
            let offset = input.start - within.from.start;
            let to_start = within.to.start + offset;
            let to_end = to_start + u64::abs_diff(input.end, input.start);
                if to_start == 0 {
                    println!("{input:#?}");
                }
            return vec![to_start..to_end];
        }*/
        let mut output = Vec::new();
        let mut input = input.start..input.end;
        loop {
            input =
                if let Some(within) = self.mapping.iter().find(|x| x.from.contains(&input.start)) {
                    //println!("front - {input:?} - {within:#?} - {:?}", self.from_type);
                    let (to_start, to_end) = if within.to.start > within.from.start {
                        let offset = within.to.start - within.from.start;
                        let end = if input.end + offset > within.to.end {
                            within.to.end
                        } else {
                            input.end + offset
                        };
                        (input.start + offset, end)
                    } else {
                        let offset = within.from.start - within.to.start;
                        let end = if input.end - offset > within.to.end {
                            within.to.end
                        } else {
                            input.end - offset
                        };
                        (input.start - offset, end)
                    };
                    output.push(to_start..to_end);
                    if input.end <= within.from.end {
                        break;
                    }
                    within.from.end..input.end
                } else if let Some(within) = self
                    .mapping
                    .iter()
                    .find(|x| x.from.contains(&(input.end - 1)))
                {
                    //println!("end - {input:?} - {within:#?} - {:?}", self.from_type);
                    let (to_start, to_end) = if within.to.start > within.from.start {
                        let offset = within.to.start - within.from.start;
                        let start = if input.start + offset < within.to.start {
                            within.to.start
                        } else {
                            input.start + offset
                        };
                        (start, input.end + offset)
                    } else {
                        let offset = within.from.start - within.to.start;
                        let start = if input.start + offset < within.to.start {
                            within.to.start
                        } else {
                            input.start + offset
                        };
                        (start, input.end - offset)
                    };
                    output.push(to_start..to_end);
                    if input.start >= within.from.start {
                        break;
                    }
                    input.start..within.from.start
                } else {
                    //println!("else - {input:#?} -  {:?}", self.from_type);
                    output.push(input.clone());
                    break;
                };
        }
        output
    }
}

/// part2 of day 5 of AOC 2023
///
/// # Arguments
/// - input the puszzle input
///
/// # Panics
/// panics whenever the input isn't parsable
#[must_use]
pub fn part2(input: &str) -> String {
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
            .flat_map(|x| current_map.input_to_output(x))
            .unique()
            .collect::<Vec<_>>();
        //println!("{to_process:#?}");
        from_type = current_map.to_type;
    }
    //println!("{to_process:#?}");
    to_process
        .iter()
        .map(|x| x.start)
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
    Ok((
        input,
        ItemMapEntry {
            to: to..(to + count),
            from: from..(from + count),
        },
    ))
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

fn parse_seed_range(input: &str) -> IResult<&str, Range<u64>> {
    let (input, (seed, count)) =
        separated_pair(complete::u64, complete::space1, complete::u64)(input)?;
    Ok((input, seed..(seed + count)))
}

//TODO need to change so that it operates on the ranges and not on the actual numbers

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = complete::space1(input)?;
    separated_list1(complete::space1, parse_seed_range)(input)
    //println!("{seed_ranges:?}");
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Range<u64>>, Vec<ItemMap>)> {
    let (input, seeds) = terminated(parse_seeds, complete::line_ending)(input)?;
    let (input, _) = complete::line_ending(input)?;
    let (input, maps) = separated_list1(complete::line_ending, parse_map)(input)?;
    //println!("{seeds:?}");
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
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "46".to_string());
    }
}
