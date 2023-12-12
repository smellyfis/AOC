#![warn(clippy::all, clippy::pedantic)]

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{pair, preceded},
    IResult,
};

/// part2 of day 2 of AOC 2023
///
/// # Arguments
/// - input the puszzle input
///
/// # Panics
/// panics whenever the input isn't parsable
#[must_use]
pub fn part2(input: &str) -> String {
    let (_, race) = parse_input(input).expect("input expected");
    (0..=race.0)
        .filter_map(|x| {
            if (race.0 - x) * x > race.1 {
                Some(())
            } else {
                None
            }
        })
        .count()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, time) = preceded(
        pair(tag("Time:"), complete::space1),
        separated_list1(complete::space1, complete::u64),
    )(input)?;
    let (input, _) = complete::line_ending(input)?;
    let (input, distance) = preceded(
        pair(tag("Distance:"), complete::space1),
        separated_list1(complete::space1, complete::u64),
    )(input)?;
    let distance = distance
        .iter()
        .map(ToString::to_string)
        .join("")
        .parse::<u64>()
        .expect("is a number");
    let time = time
        .iter()
        .map(ToString::to_string)
        .join("")
        .parse::<u64>()
        .expect("is a number");

    Ok((input, (time, distance)))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "71503".to_string());
    }
}
