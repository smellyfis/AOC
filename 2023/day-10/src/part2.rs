#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, fmt::Display, iter::successors};

use glam::IVec2;
use itertools::Itertools;
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum PipeFrom {
    Up,
    Down,
    Left,
    Right,
}
impl PipeFrom {
    fn from_ivecs(a: IVec2, b: IVec2) -> Option<Self> {
        match (a - b).into() {
            (0, 1) => Some(Self::Down),
            (0, -1) => Some(Self::Up),
            (1, 0) => Some(Self::Right),
            (-1, 0) => Some(Self::Left),
            _ => None,
            //value => unimplemented!("this can't be {a:?} - {b:?} = {value:?}"),
        }
    }
    fn to_ivec(self) -> IVec2 {
        match self {
            PipeFrom::Up => (0, -1).into(),
            PipeFrom::Down => (0, 1).into(),
            PipeFrom::Left => (-1, 0).into(),
            PipeFrom::Right => (1, 0).into(),
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
    Outer,
    Inner,
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
            value => unimplemented!("this should never have been called for type {value:?}"),
        }
    }
}
impl Display for PipeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Start => "S",
                Self::Horizontal => "-",
                Self::Vertical => "|",
                Self::DownRight => "F",
                Self::DownLeft => "7",
                Self::UpRight => "L",
                Self::UpLeft => "J",
                Self::None => ".",
                Self::Outer => "O",
                Self::Inner => "I",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Pipe {
    pub pipe_type: PipeType,
    pub position: IVec2,
}

impl Pipe {
    fn get_adjacent(&self) -> Vec<(IVec2, PipeFrom)> {
        self.pipe_type
            .get_adjacents()
            .into_iter()
            .map(|x| x + self.position)
            .filter_map(|x| PipeFrom::from_ivecs(self.position, x).map(|y| (x, y)))
            .collect()
    }
    fn next(&self, from: PipeFrom) -> IVec2 {
        use PipeFrom::{Down, Left, Right, Up};
        use PipeType::{DownLeft, DownRight, Horizontal, UpLeft, UpRight, Vertical};
        match (from, self.pipe_type) {
            (Up, Vertical) | (Left, DownLeft) | (Right, DownRight) => Down,
            (Up, UpLeft) | (Down, DownLeft) | (Right, Horizontal) => Left,
            (Up, UpRight) | (Down, DownRight) | (Left, Horizontal) => Right,
            (Down, Vertical) | (Left, UpLeft) | (Right, UpRight) => Up,
            _ => unimplemented!("no"),
        }
        .to_ivec()
            + self.position
    }
}

/// day 10 part 2 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part2(input: &str) -> String {
    let input = Span::new(input);
    let (_, grid) = parse_input(input).expect("aoc always parse");
    let start_node = grid
        .values()
        .find(|x| x.pipe_type == PipeType::Start)
        .expect("has a start");
    let start_node_true_type = match &start_node
        .get_adjacent()
        .iter()
        .filter_map(|(x, from)| grid.get(x).map(|y| (y, *from)))
        .filter_map(|(x, from)| {
            x.get_adjacent()
                .iter()
                .map(|(y, _)| y)
                .contains(&start_node.position)
                .then_some(from)
        })
        .collect::<Vec<_>>()[..]
    {
        [PipeFrom::Up, PipeFrom::Left] | [PipeFrom::Left, PipeFrom::Up] => PipeType::DownRight,
        [PipeFrom::Up, PipeFrom::Right] | [PipeFrom::Right, PipeFrom::Up] => PipeType::DownLeft,
        [PipeFrom::Down, PipeFrom::Left] | [PipeFrom::Left, PipeFrom::Down] => PipeType::UpRight,
        [PipeFrom::Down, PipeFrom::Right] | [PipeFrom::Right, PipeFrom::Down] => PipeType::UpLeft,
        [PipeFrom::Up, PipeFrom::Down] | [PipeFrom::Down, PipeFrom::Up] => PipeType::Vertical,
        [PipeFrom::Right, PipeFrom::Left] | [PipeFrom::Left, PipeFrom::Right] => {
            PipeType::Horizontal
        }
        _ => PipeType::Start,
    };

    let mut pieces = HashMap::new();
    pieces.insert(start_node.position, start_node_true_type);

    successors(
        Some(
            start_node
                .get_adjacent()
                .iter()
                .filter_map(|(x, from)| grid.get(x).map(|y| (y, *from)))
                .filter(|(x, _)| {
                    x.get_adjacent()
                        .iter()
                        .map(|(y, _)| y)
                        .contains(&start_node.position)
                })
                .collect::<Vec<_>>(),
        ),
        |front_nodes| {
            if front_nodes[0].0 == front_nodes[1].0 {
                return None;
            }
            Some(
                front_nodes
                    .iter()
                    .filter_map(|(pipe, from)| {
                        grid.get(&pipe.next(*from))
                            .map(|x| (x, PipeFrom::from_ivecs(pipe.position, x.position).unwrap()))
                    })
                    .collect::<Vec<_>>(),
            )
        },
    )
    .filter(|x| !x.is_empty())
    .for_each(|x| {
        for (pipe, _) in &x {
            pieces.insert(pipe.position, pipe.pipe_type);
        }
    });
    let corners = pieces.keys().fold(
        ((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)),
        |((minimum_x, min_y), (maximal_x, max_y)), pos| {
            let minimum_x = minimum_x.min(pos.x);
            let miny = min_y.min(pos.y);
            let maximal_x = maximal_x.max(pos.x);
            let maxy = max_y.max(pos.y);
            ((minimum_x, miny), (maximal_x, maxy))
        },
    );
    /* Debug
    (corners.0 .1..=corners.1 .1).for_each(|y| {
        (corners.0.0..=corners.1.0).for_each(|x| {
            let p = pieces.get(&(x,y).into()).unwrap_or(&PipeType::None);
            print!("{p}");
        });
        print!("\n");
    });
    */
    (corners.0 .1..=corners.1 .1).for_each(|y| {
        let mut status = false;
        (corners.0 .0..=corners.1 .0)
            .map(|x| IVec2::new(x, y))
            .for_each(|pos| {
                if let Some(piece) = pieces.get(&pos) {
                    status = match piece {
                        PipeType::Vertical | PipeType::DownRight | PipeType::DownLeft => !status,
                        _ => status,
                    };
                } else if status {
                    pieces.insert(pos, PipeType::Inner);
                } else {
                    pieces.insert(pos, PipeType::Outer);
                }
            });
    });
    /* Debug
    println!();
    (corners.0 .1..=corners.1 .1).for_each(|y| {
        (corners.0.0..=corners.1.0).for_each(|x| {
            let p = pieces.get(&(x,y).into()).unwrap();
            print!("{p}");
        });
        print!("\n");
    });
    */
    pieces
        .values()
        .filter(|x| **x == PipeType::Inner)
        .count()
        .to_string()
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
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        "4"
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        "8"
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        "10"
    )]

    fn part2_works(#[case] input: &str, #[case] expected: &str) {
        let result = part2(input);
        assert_eq!(result, expected);
    }
}
