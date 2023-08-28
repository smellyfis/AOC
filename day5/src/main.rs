#![warn(clippy::all, clippy::pedantic)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Debug)]
struct Crate {
    label: String,
}

#[derive(Debug)]
struct GameBoard {
    _labels: Vec<String>,
    board: Vec<Vec<Crate>>,
}

impl GameBoard {
    fn _game1_move(&mut self, count: usize, from: usize, to: usize) {
        let v = &mut Vec::new();
        let work = self.board.get_mut(from - 1).unwrap();
        for _ in 0..count {
            v.push(work.pop().unwrap());
        }
        let work = self.board.get_mut(to - 1).unwrap();
        for _ in 0..count {
            work.append(v);
        }
    }
    fn game2_move(&mut self, count: usize, from: usize, to: usize) {
        let v = &mut Vec::new();
        let work = self.board.get_mut(from - 1).unwrap();
        for _ in 0..count {
            v.push(work.pop().unwrap());
        }
        v.reverse();
        let work = self.board.get_mut(to - 1).unwrap();
        for _ in 0..count {
            work.append(v);
        }
    }
    fn get_tops(&self) -> String {
        self.board
            .iter()
            .map(|x| x.last().unwrap().label.clone())
            .fold(String::new(), |acc, x| acc + &x)
    }
}
impl From<Vec<String>> for GameBoard {
    fn from(v: Vec<String>) -> Self {
        let mut board_vec = v.clone();
        let label_vec = board_vec.pop().unwrap();
        // get labels
        let labels = label_vec
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        //TODO sscanf for crates
        board_vec.reverse();
        let mut board = vec![Vec::new(); labels.len()];
        for line in &board_vec {
            board.iter_mut().enumerate().for_each(|(i, col)| {
                let (begin, end) = (i * 4, std::cmp::min(i * 4 + 4, line.len()));
                let crate_str = line[begin..end]
                    .to_string()
                    .matches(char::is_alphabetic)
                    .collect::<String>();
                if !crate_str.is_empty() {
                    col.push(Crate { label: crate_str });
                }
            });
        }
        GameBoard {
            _labels: labels,
            board,
        }
    }
}

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    //read in the parts of the file
    let (board_lines, movement_lines, _) = reader.lines().fold(
        (Vec::new(), Vec::new(), false),
        |mut acc: (Vec<String>, Vec<String>, bool), line| {
            let line = line.unwrap();
            if line.is_empty() {
                acc.2 = true;
            } else if acc.2 {
                acc.1.push(line);
            } else {
                acc.0.push(line);
            }
            acc
        },
    );
    println!("{board_lines:?}");
    let mut board = GameBoard::from(board_lines);

    for line in &movement_lines {
        match line
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<usize>>()[..]
        {
            [count, from, to] => board.game2_move(count, from, to),
            _ => panic!(
                "{:#?} {:?}",
                board,
                line.matches(char::is_numeric)
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            ),
        };
    }

    println!("{}", board.get_tops());

    Ok(())
}
