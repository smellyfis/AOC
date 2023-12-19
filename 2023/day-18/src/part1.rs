#![warn(clippy::all, clippy::pedantic)]

use glam::I64Vec2;
use itertools::Itertools;
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
    pub count: i64,
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
    let corners = steps
        .iter()
        .scan(I64Vec2::splat(0), |cursor, next| {
            let dir = match next.direction {
                Direction::Up => I64Vec2::NEG_Y,
                Direction::Down => I64Vec2::Y,
                Direction::Left => I64Vec2::NEG_X,
                Direction::Right => I64Vec2::X,
            };
            *cursor += next.count * dir;
            Some(*cursor)
        })
        .collect::<Vec<_>>();
    let perimeter = corners
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let dist = (*b - *a).abs();
            dist.x + dist.y
        })
        .sum::<i64>()
        + {
            let a = corners.last().unwrap();
            let b = corners.first().unwrap();
            let dist = (*b - *a).abs();
            dist.x + dist.y
        };
    let area = (corners
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<i64>()
        + perimeter)
        / 2
        + 1;
    area.to_string()
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
            complete::i64,
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
