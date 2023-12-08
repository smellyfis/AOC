#![warn(clippy::all, clippy::pedantic)]

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{pair, preceded},
    IResult,
};

#[must_use]
pub fn part1(input: &str) -> String {
    let (_, races) = parse_input(input).expect("input expected");
    races
        .iter()
        .map(|(time, distance)| {
            (0..=*time)
                .filter_map(|x| {
                    if (time - x) * x > *distance {
                        Some(())
                    } else {
                        None
                    }
                })
                .count()
        })
        .product::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (input, time) = preceded(
        pair(tag("Time:"), complete::space1),
        separated_list1(complete::space1, complete::u64),
    )(input)?;
    let (input, _) = complete::line_ending(input)?;
    let (input, distance) = preceded(
        pair(tag("Distance:"), complete::space1),
        separated_list1(complete::space1, complete::u64),
    )(input)?;

    Ok((
        input,
        time.iter()
            .interleave(distance.iter())
            .copied()
            .tuples()
            .collect(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "288".to_string());
    }
}
