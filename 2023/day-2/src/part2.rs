#![warn(clippy::all, clippy::pedantic)]

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

#[derive(Debug)]
struct Round {
    pub red_n: u32,
    pub green_n: u32,
    pub blue_n: u32,
}

#[derive(Debug)]
struct Game {
    pub _id: u32,
    pub rounds: Vec<Round>,
}

impl Game {
    fn to_power(&self) -> u64 {
        let (r, g, b) = self.rounds.iter().fold((0_u64, 0_u64, 0_u64), |acc, x| {
            let (mut val_r, mut val_g, mut val_b) = acc;
            if u64::from(x.red_n) > acc.0 {
                val_r = x.red_n.into();
            }
            if u64::from(x.green_n) > acc.1 {
                val_g = x.green_n.into();
            }
            if u64::from(x.blue_n) > acc.2 {
                val_b = x.blue_n.into();
            }
            (val_r, val_g, val_b)
        });
        r * g * b
    }
}

/// part2 of day 2 of AOC 2023
///
/// # Arguments
/// - input the puszzle input
///
/// # Panics
/// panics whenever the input isn't parsable
pub fn part2(input: &str) -> String {
    let (_, games) = process_input(input).expect("there should be input");
    games.iter().map(Game::to_power).sum::<u64>().to_string()
}

fn process_block(input: &str) -> nom::IResult<&str, (u32, String)> {
    let (i, (cnt, color)) =
        separated_pair(complete::u32, complete::space1, complete::alpha1)(input)?;
    Ok((i, (cnt, color.to_owned())))
}

fn process_round(input: &str) -> nom::IResult<&str, Round> {
    let (i, blocks) = separated_list1(tag(", "), process_block)(input)?;
    let mut round = Round {
        red_n: 0,
        green_n: 0,
        blue_n: 0,
    };
    for (cnt, color) in blocks {
        match color.as_str() {
            "red" => round.red_n = cnt,
            "green" => round.green_n = cnt,
            "blue" => round.blue_n = cnt,
            _ => panic!("this should be a color name"),
        };
    }
    Ok((i, round))
}

fn process_game(input: &str) -> nom::IResult<&str, Game> {
    let (i, (id, rounds)) = separated_pair(
        preceded(tag("Game "), complete::u32),
        tag(": "),
        separated_list1(tag("; "), process_round),
    )(input)?;
    Ok((i, Game { _id: id, rounds }))
}

fn process_input(input: &str) -> nom::IResult<&str, Vec<Game>> {
    separated_list1(newline, process_game)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "2286".to_string());
    }
}
