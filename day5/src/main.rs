#![warn(clippy::all, clippy::pedantic)]

use std::fs;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, newline};
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::Parser;

#[derive(Clone, Debug, Default, Copy)]
struct Crate<'a> {
    label: &'a str,
}

impl<'a> Parser<&'a str, Self, Error<&'a str>> for Crate<'a> {
    fn parse(&mut self, input: &'a str) -> nom::IResult<&'a str, Self, Error<&'a str>> {
        let (input, label) = delimited::<&str, _, &str, _, Error<&'a str>, _, _, _>(
            tag("["),
            complete::alpha1,
            tag("]"),
        )(input)?;
        self.label = label;
        Ok((input, *self))
    }
}

#[derive(Clone, Debug, Default, Copy)]
struct GameMove {
    quantity: usize,
    from: usize,
    to: usize,
}
impl<'a> Parser<&'a str, Self, Error<&'a str>> for GameMove {
    fn parse(&mut self, input: &'a str) -> nom::IResult<&'a str, Self, Error<&'a str>> {
        let (input, quantity) = preceded(tag("move "), complete::u8.map(|x| x as usize))(input)?;
        let (input, from) = preceded(tag(" from "), complete::u8.map(|x| x as usize))(input)?;
        let (input, to) = preceded(tag(" to "), complete::u8.map(|x| x as usize))(input)?;
        self.quantity = quantity;
        self.from = from;
        self.to = to;
        Ok((input, *self))
    }
}

#[derive(Debug, Default)]
struct GameBoard<'a> {
    _labels: Vec<String>,
    board: Vec<Vec<Crate<'a>>>,
}

impl<'a> GameBoard<'a> {
    pub fn game1_move(&mut self, m: &GameMove) {
        let v = &mut Vec::new();
        let work = self.board.get_mut(m.from - 1).unwrap();
        for _ in 0..m.quantity {
            v.push(work.pop().unwrap());
        }
        let work = self.board.get_mut(m.to - 1).unwrap();
        for _ in 0..m.quantity {
            work.append(v);
        }
    }
    pub fn game2_move(&mut self, m: &GameMove) {
        let v = &mut Vec::new();
        let work = self.board.get_mut(m.from - 1).unwrap();
        for _ in 0..m.quantity {
            v.push(work.pop().unwrap());
        }
        v.reverse();
        let work = self.board.get_mut(m.to - 1).unwrap();
        for _ in 0..m.quantity {
            work.append(v);
        }
    }
    fn get_tops(&self) -> String {
        self.board
            .iter()
            .map(|x| x.last().unwrap().label.clone())
            .fold(String::new(), |acc, x| acc + x)
    }
}

impl<'a> Parser<&'a str, Self, Error<&'a str>> for GameBoard<'a> {
    fn parse(&mut self, input: &'a str) -> nom::IResult<&'a str, Self, Error<&'a str>> {
        let (input, crates) = separated_list1(
            newline,
            separated_list1(
                tag(" "),
                alt((tag("   ").map(|_| None), Crate::default().map(Some))),
            ),
        )(input)?;
        let (input, _) = newline(input)?;
        let (input, labels) = separated_list1(
            tag(" "),
            delimited(tag(" "), complete::u8.map(|x| x.to_string()), tag(" ")),
        )(input)?;
        let (input, _) = newline(input)?;
        //self._labels = labels;
        let mut board = vec![Vec::new(); crates[0].len()];
        for cols in crates {
            for (col, c) in cols.iter().enumerate() {
                if c.is_none() {
                    continue;
                }
                board[col].push(c.unwrap());
            }
        }
        board.iter_mut().for_each(|col| col.reverse());
        self.board = board;
        let b = GameBoard {
            _labels: labels,
            board: self.board.clone(),
        };
        Ok((input, b))
    }
}

fn parse_input(input: &str) -> nom::IResult<&str, (GameBoard, Vec<GameMove>)> {
    separated_pair(
        GameBoard::default(),
        newline,
        separated_list1(newline, GameMove::default()),
    )(input)
}

fn part1(input: &str) -> String {
    let (_, (mut board, moves)) = parse_input(input).unwrap();
    for m in moves {
        board.game1_move(&m);
    }
    board.get_tops()
}

fn part2(input: &str) -> String {
    let (_, (mut board, moves)) = parse_input(input).unwrap();
    for m in moves {
        board.game2_move(&m);
    }
    board.get_tops()
}

fn main() {
    //Read in file
    let file = fs::read_to_string("input").unwrap();

    //read in the parts of the file

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), "CMZ");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), "MCD");
    }
}
