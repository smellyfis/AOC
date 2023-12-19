#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Step {
    pub direction: Direction,
    pub count: i32,
    pub _color: String,
}

/// day 18 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part1(input: &str) -> String {
    let (_, steps) = parse_input(input).expect("valid aoc content not found");
    let mut cursor = IVec2::splat(0);
    let mut grid = HashMap::from([(IVec2::splat(0), '#')]);
    steps.iter().for_each(|step| {
        match step.direction {
            Direction::Up => {
                (1..=step.count).for_each(|x| {
                    let pos = cursor - IVec2::new(0, x);
                    grid.insert(pos, '#');
                });
                cursor -= IVec2::new(0, step.count);
            }
            Direction::Down => {
                (1..=step.count).for_each(|x| {
                    let pos = cursor + IVec2::new(0, x);
                    grid.insert(pos, '#');
                });
                cursor += IVec2::new(0, step.count);
            }
            Direction::Left => {
                (1..=step.count).for_each(|x| {
                    let pos = cursor - IVec2::new(x, 0);
                    grid.insert(pos, '#');
                });
                cursor -= IVec2::new(step.count, 0);
            }
            Direction::Right => {
                (1..=step.count).for_each(|x| {
                    let pos = cursor + IVec2::new(x, 0);
                    grid.insert(pos, '#');
                });
                cursor += IVec2::new(step.count, 0);
            }
        };
    });
    let (min_x, min_y, max_x, max_y) = grid.keys().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(min_x, min_y, max_x, max_y), pos| {
            (
                min_x.min(pos.x),
                min_y.min(pos.y),
                max_x.max(pos.x),
                max_y.max(pos.y),
            )
        },
    );
    (min_y..=max_y).for_each(|y| {
        let mut inside = false;
        (min_x..=max_x).for_each(|x| {
            let square = grid.get(&IVec2::new(x, y));
            //print!("{}", square.unwrap_or(&'.'));
            //is it in or out ogf the loop
            inside = if square.is_some() && grid.get(&IVec2::new(x, y + 1)).is_some() {
                !inside
            } else {
                inside
            };
            if square.is_none() && inside {
                grid.insert(IVec2::new(x, y), '#');
            }
        });
        //print!("\n");
    });
    grid.len().to_string()
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    let (input, ((direction, count), color)) = separated_pair(
        separated_pair(
            alt((
                tag("U").map(|_| Direction::Up),
                tag("D").map(|_| Direction::Down),
                tag("L").map(|_| Direction::Left),
                tag("R").map(|_| Direction::Right),
            )),
            complete::space1,
            complete::i32,
        ),
        complete::space1,
        delimited(tag("("), preceded(tag("#"), complete::hex_digit1), tag(")")),
    )(input)?;
    Ok((
        input,
        Step {
            direction,
            count,
            _color: color.to_string(),
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(complete::line_ending, parse_step)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "62".to_string());
    }
}
