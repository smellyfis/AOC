use log::debug;
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
    pub id: u32,
    pub rounds: Vec<Round>,
}

impl Game {
    fn to_part1(&self) -> Option<u32> {
        if self
            .rounds
            .iter()
            .find_map(|r| {
                //TODO if inverted use find_map
                if r.red_n > 12 || r.green_n > 13 || r.blue_n > 14 {
                    Some(self.id)
                } else {
                    None
                }
            })
            .is_some()
        {
            None
        } else {
            Some(self.id)
        }
    }
}

pub fn part1(input: &str) -> String {
    let (_, games) = process_input(input).expect("there should be input");
    debug!("{games:?}");
    games
        .iter()
        .filter_map(Game::to_part1)
        .sum::<u32>()
        .to_string()
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
    Ok((i, Game { id, rounds }))
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
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "8".to_string());
    }
}
