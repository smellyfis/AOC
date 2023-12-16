#![warn(clippy::all, clippy::pedantic)]

use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::terminated,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Gadget {
    Horizontal,
    Vertical,
    UlDr,
    UrDl,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FromDir {
    Left,
    Right,
    Up,
    Down,
}

/// day 16 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part1(input: &str) -> String {
    let input = Span::new(input);
    let (_, (gadgets, maxes)) = parse_input(input).expect("always aoc");
    let mut movement_cache = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(IVec2::new(0, 0), FromDir::Left)]);
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if !movement_cache.insert(pos) {
            continue; // cycle detection
        }
        let (pos, from) = pos;
        if pos.x >= maxes.x || pos.x < 0 || pos.y >= maxes.y || pos.y < 0 {
            continue; //outside grid
        }
        visited.insert(pos);
        if let Some(gadget) = gadgets.get(&pos) {
            match (gadget, from) {
                (Gadget::Horizontal, FromDir::Left) => {
                    queue.push_back((pos + IVec2::new(1, 0), from));
                }
                (Gadget::Horizontal, FromDir::Right) => {
                    queue.push_back((pos + IVec2::new(-1, 0), from));
                }
                (Gadget::Horizontal, FromDir::Up | FromDir::Down) => {
                    queue.push_back((pos + IVec2::new(1, 0), FromDir::Left));
                    queue.push_back((pos + IVec2::new(-1, 0), FromDir::Right));
                }
                (Gadget::Vertical, FromDir::Up) => queue.push_back((pos + IVec2::new(0, 1), from)),
                (Gadget::Vertical, FromDir::Down) => {
                    queue.push_back((pos + IVec2::new(0, -1), from));
                }
                (Gadget::Vertical, FromDir::Left | FromDir::Right) => {
                    queue.push_back((pos + IVec2::new(0, 1), FromDir::Up));
                    queue.push_back((pos + IVec2::new(0, -1), FromDir::Down));
                }
                (Gadget::UlDr, FromDir::Up) | (Gadget::UrDl, FromDir::Down) => {
                    queue.push_back((pos + IVec2::new(1, 0), FromDir::Left));
                }
                (Gadget::UlDr, FromDir::Down) | (Gadget::UrDl, FromDir::Up) => {
                    queue.push_back((pos + IVec2::new(-1, 0), FromDir::Right));
                }
                (Gadget::UlDr, FromDir::Left) | (Gadget::UrDl, FromDir::Right) => {
                    queue.push_back((pos + IVec2::new(0, 1), FromDir::Up));
                }
                (Gadget::UlDr, FromDir::Right) | (Gadget::UrDl, FromDir::Left) => {
                    queue.push_back((pos + IVec2::new(0, -1), FromDir::Down));
                }
                _ => unimplemented!("This should never happen"),
            };
        } else {
            let next_pos = pos
                + match from {
                    FromDir::Left => IVec2::new(1, 0),
                    FromDir::Right => IVec2::new(-1, 0),
                    FromDir::Up => IVec2::new(0, 1),
                    FromDir::Down => IVec2::new(0, -1),
                };
            queue.push_back((next_pos, from));
        }
    }
    visited.len().to_string()
}

fn with_xy(span: Span) -> SpanIVec2 {
    let x = i32::try_from(span.get_column()).expect("overflow") - 1;
    let y = i32::try_from(span.location_line()).expect("wrap around") - 1;
    span.map_extra(|()| IVec2::new(x, y))
}

fn parse_gadget(input: Span) -> IResult<Span, (IVec2, Gadget)> {
    alt((
        tag("-").map(with_xy).map(|x| (x.extra, Gadget::Horizontal)),
        tag("|").map(with_xy).map(|x| (x.extra, Gadget::Vertical)),
        tag(r"\").map(with_xy).map(|x| (x.extra, Gadget::UlDr)),
        tag(r"/").map(with_xy).map(|x| (x.extra, Gadget::UrDl)),
        tag(".").map(with_xy).map(|x| (x.extra, Gadget::None)),
    ))(input)
}

fn parse_input(input: Span) -> IResult<Span, (HashMap<IVec2, Gadget>, IVec2)> {
    let (input, (gadgets, max_x, max_y)) = fold_many1(
        terminated(many1(parse_gadget), alt((complete::line_ending, eof))),
        || (HashMap::new(), 0, 0),
        |(mut acc, _, max_y), row| {
            let max_x = row.len().try_into().unwrap();
            row.into_iter()
                .filter(|(_, gadget)| *gadget != Gadget::None)
                .for_each(|(pos, gadget)| {
                    acc.insert(pos, gadget);
                });
            (acc, max_x, max_y + 1)
        },
    )(input)?;
    Ok((input, (gadgets, IVec2::new(max_x, max_y))))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "46".to_string());
    }
}
