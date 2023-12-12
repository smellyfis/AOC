#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, iter::successors};

use glam::IVec2;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::{fold_many1, many1},
    sequence::terminated,
    IResult, Parser, combinator::eof,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum PipeFrom {
    Up,
    Down,
    Left,
    Right,
}
impl PipeFrom {
    fn from_ivecs(a: IVec2, b: IVec2) -> Option<Self> {
        match (a-b).into() {
            (0, 1) => Some(Self::Down),
            (0, -1) => Some(Self::Up),
            (1, 0)  => Some(Self::Right),
            (-1, 0) => Some(Self::Left),
            _ => None,
            //value => unimplemented!("this can't be {a:?} - {b:?} = {value:?}"),
        }
    }
    fn to_ivec(self) -> IVec2 {
        match self {
            PipeFrom::Up => (0,-1).into(),
            PipeFrom::Down => (0,1).into(),
            PipeFrom::Left => (-1,0).into(),
            PipeFrom::Right => (1,0).into(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum PipeType {
    // 'S'
    Start,
    // '-'
    Horizontal,
    // '|'
    Vertical,
    // 'F'
    DownRight,
    // '7'
    DownLeft,
    // 'L'
    UpRight,
    // 'J'
    UpLeft,
    // '.' -this is so we can parse but should be discarded
    None,
}

impl PipeType {
    fn get_adjacents(self) -> Vec<IVec2> {
        match self {
            PipeType::Start => vec![(-1, 0).into(), (0, -1).into(), (0, 1).into(), (1, 0).into()],
            PipeType::Horizontal => vec![(1, 0).into(), (-1, 0).into()],
            PipeType::Vertical => vec![(0, 1).into(), (0, -1).into()],
            PipeType::DownRight => vec![(0, 1).into(), (1, 0).into()],
            PipeType::DownLeft => vec![(0, 1).into(), (-1, 0).into()],
            PipeType::UpRight => vec![(0, -1).into(), (1, 0).into()],
            PipeType::UpLeft => vec![(0, -1).into(), (-1, 0).into()],
            PipeType::None => unimplemented!("this should never have been called"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Pipe {
    pub pipe_type: PipeType,
    pub position: IVec2,
}

impl Pipe {
    fn get_adjacent(&self ) -> Vec<(IVec2, PipeFrom)> {
        self.pipe_type
            .get_adjacents()
            .into_iter()
            .map(|x| x + self.position)
            .filter_map(|x| PipeFrom::from_ivecs(self.position , x).map(|y| (x,y) ))
            .collect()
    }
    fn next(&self, from: PipeFrom) -> IVec2 {
        use PipeFrom::*;
        use PipeType::*;
        match (from, self.pipe_type) {
            (Up, Vertical) | (Left, DownLeft) | (Right, DownRight) => Down,
            (Up, UpLeft) | (Down, DownLeft) | (Right, Horizontal) => Left,
            (Up, UpRight) | (Down, DownRight) | (Left, Horizontal)  => Right,
            (Down, Vertical) | (Left, UpLeft) | (Right, UpRight) => Up,
            _ => unimplemented!("no"),
        }.to_ivec() + self.position
    }
}

/// day 10 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part1(input: &str) -> String {
    let input = Span::new(input);
    let (_, grid) = parse_input(input).expect("aoc always parse");
    let start_node = grid
        .values()
        .find(|x| x.pipe_type == PipeType::Start)
        .expect("has a start");

    (successors(
        Some(
            start_node
                .get_adjacent()
                .iter()
                .filter_map(|(x, from)| grid.get(x).map(|y|(y,*from)))
                .filter(|(x, _)| x.get_adjacent().iter().map(|(y,_)|y).contains(&start_node.position))
                .collect::<Vec<_>>()
        ),
        |front_nodes| {
            Some(front_nodes
                .iter()
                .filter_map(|(pipe, from)| {
                    grid.get(&pipe.next(*from)).map(|x|(x,PipeFrom::from_ivecs(pipe.position,x.position ).unwrap()))
                })
                .collect::<Vec<_>>())
        },
    )
        .filter(|x| !x.is_empty())
        .position(|a| a[0].0 == a[1].0)
        .unwrap()
        +1)
    .to_string()
    //todo!()
}

fn with_xy(span: Span) -> SpanIVec2 {
    let x = i32::try_from(span.get_column()).expect("overflow") - 1;
    let y = i32::try_from(span.location_line()).expect("wrap around") - 1;
    span.map_extra(|()| IVec2::new(x, y))
}

fn parse_pipe(input: Span) -> IResult<Span, Pipe> {
    alt((
        tag("S").map(with_xy).map(|position| Pipe {
            pipe_type: PipeType::Start,
            position: position.extra,
        }),
        tag("-").map(with_xy).map(|position| Pipe {
            pipe_type: PipeType::Horizontal,
            position: position.extra,
        }),
        tag("|").map(with_xy).map(|position| Pipe {
            pipe_type: PipeType::Vertical,
            position: position.extra,
        }),
        tag("F").map(with_xy).map(|position| Pipe {
            pipe_type: PipeType::DownRight,
            position: position.extra,
        }),
        tag("7").map(with_xy).map(|position| Pipe {
            pipe_type: PipeType::DownLeft,
            position: position.extra,
        }),
        tag("L").map(with_xy).map(|position| Pipe {
            pipe_type: PipeType::UpRight,
            position: position.extra,
        }),
        tag("J").map(with_xy).map(|position| Pipe {
            pipe_type: PipeType::UpLeft,
            position: position.extra,
        }),
        tag(".").map(with_xy).map(|position| Pipe {
            pipe_type: PipeType::None,
            position: position.extra,
        }),
    ))(input)
}

fn parse_input(input: Span) -> IResult<Span, HashMap<IVec2, Pipe>> {
    fold_many1(
        terminated(many1(parse_pipe), alt((complete::line_ending, eof))),
        HashMap::new,
        |mut acc, x| {
            x.into_iter()
                .filter(|x| x.pipe_type != PipeType::None)
                .for_each(|x| {
                    acc.insert(x.position, x);
                });
            acc
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        "4"
    )]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        "4"
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        "8"
    )]
    #[case(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        "8"
    )]

    fn part1_works(#[case] input: &str, #[case] expected: &str) {
        let result = part1(input);
        assert_eq!(result, expected);
    }
}

