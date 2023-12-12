#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, fmt::Display, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list0,
    sequence::separated_pair,
};

#[derive(Debug, Clone, Copy)]
enum Square {
    Air,
    Wall,
    Sand,
}

impl Square {
    pub fn is_sand(self) -> bool {
        matches!(self, Self::Sand)
    }
    pub fn is_rock(self) -> bool {
        matches!(self, Self::Wall)
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::Air
    }
}
impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Self::Air => '.',
            Self::Wall => '#',
            Self::Sand => 'o',
        };
        write!(f, "{display}")
    }
}

fn rock_wall(input: &str) -> nom::IResult<&str, HashMap<(usize, usize), Square>> {
    let (input, pts) = separated_list0(
        tag(" -> "),
        separated_pair(complete::u64, tag(","), complete::u64),
    )(input)?;
    let mut rocks = HashMap::new();
    pts.iter()
        .enumerate()
        .take(pts.len() - 1)
        .flat_map(|(i, start)| {
            let start = (
                usize::try_from(start.0).unwrap(),
                usize::try_from(start.1).unwrap(),
            );
            let (ex, ey) = pts.get(i + 1).unwrap();
            let end = (usize::try_from(*ex).unwrap(), usize::try_from(*ey).unwrap());
            if start.0 == end.0 {
                let (s, e) = if start.1 < end.1 {
                    (start.1, end.1)
                } else {
                    (end.1, start.1)
                };
                (s..=e).map(|y| (start.0, y)).collect::<Vec<_>>()
            } else if start.1 == end.1 {
                let (s, e) = if start.0 < end.0 {
                    (start.0, end.0)
                } else {
                    (end.0, start.0)
                };
                (s..=e).map(|x| (x, start.1)).collect::<Vec<_>>()
            } else {
                panic!("wee wooo");
            }
        })
        .for_each(|pt| {
            rocks.insert(pt, Square::Wall);
        });
    Ok((input, rocks))
}

fn parse_input(input: &str) -> nom::IResult<&str, HashMap<(usize, usize), Square>> {
    let (input, walls) = separated_list0(newline, rock_wall)(input)?;
    let walls = walls
        .iter()
        .flatten()
        .map(|(pt, s)| (*pt, *s))
        .collect::<HashMap<(usize, usize), Square>>();
    Ok((input, walls))
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let (_, mut board) = parse_input(&file).unwrap();

    //insert rocks
    //get lowest rock
    let lowest = board
        .iter()
        .filter_map(|((_, y), typ)| if typ.is_rock() { Some(y) } else { None })
        .max()
        .unwrap()
        .to_owned();
    while board.get(&(500_usize, 0_usize)).is_none() {
        let mut square = (500_usize, 0_usize);
        //drop - find the place to insert
        while square.1 < lowest {
            let checks = [
                (square.0, square.1 + 1),
                (square.0 - 1, square.1 + 1),
                (square.0 + 1, square.1 + 1),
            ];
            square = if let Some(new) = checks.iter().find(|pos| board.get(pos).is_none()) {
                *new
            } else {
                break;
            };
        }
        if square.1 >= lowest {
            break;
        }
        board.insert(square, Square::Sand);
    }
    let part1 = board.values().filter(|x| x.is_sand()).count();
    println!("Part 1: {part1}");
    while board.get(&(500_usize, 0_usize)).is_none() {
        let mut square = (500_usize, 0_usize);
        //drop - find the place to insert
        while square.1 < lowest + 1 {
            let checks = [
                (square.0, square.1 + 1),
                (square.0 - 1, square.1 + 1),
                (square.0 + 1, square.1 + 1),
            ];
            square = if let Some(new) = checks.iter().find(|pos| board.get(pos).is_none()) {
                *new
            } else {
                break;
            };
        }
        board.insert(square, Square::Sand);
    }
    let part2 = board.values().filter(|x| x.is_sand()).count();
    println!("Part 2: {part2}");
}
