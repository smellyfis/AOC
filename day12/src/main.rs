#![warn(clippy::all, clippy::pedantic)]

use std::{
    collections::VecDeque,
    fs::File,
    io::{prelude::*, BufReader},
};

pub type BoxError = std::boxed::Box<
    dyn std::error::Error // must implement Error to satisfy ?
        + std::marker::Send // needed for threads
        + std::marker::Sync, // needed for threads
>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Square {
    pub height: u8,
    pub pos: (usize, usize),
}
impl Square {
    pub fn _index(&self, grid_size: &(usize, usize)) -> usize {
        grid_size.1 * self.pos.0 + self.pos.1
    }

    pub fn can_go_to(&self, other: &Self) -> bool {
        let from_height = self.height;
        let to_height = other.height;
        if from_height + 1 < to_height {
            return false;
        }
        let (from_row, from_col) = self.pos;
        let (to_row, to_col) = other.pos;
        (from_row == to_row || from_col == to_col)
            && from_row + 1 >= to_row
            && from_row <= to_row + 1
            && from_col + 1 >= to_col
            && from_col <= to_col + 1
    }
}

#[derive(Debug, Default, Clone)]
struct Game {
    board: Vec<Square>,
    distances: Vec<Option<u64>>,
    board_size: (usize, usize),
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}

impl From<File> for Game {
    fn from(file: File) -> Self {
        let buffer = BufReader::new(file);
        let mut board = Vec::new();
        let mut board_size = (0_usize, 0_usize);
        let mut start_pos = (0_usize, 0_usize);
        let mut end_pos = (0_usize, 0_usize);
        for (row, line) in buffer
            .lines()
            .enumerate()
            .map(|(row, col)| (row, col.unwrap()))
        {
            board_size.0 += 1;
            board_size.1 = if board_size.1 == 0 {
                line.len()
            } else {
                board_size.1
            };
            for (col, val) in line.as_bytes().iter().enumerate() {
                let height = match val {
                    b'E' => {
                        end_pos = (row, col);
                        25
                    }
                    b'S' => {
                        start_pos = (row, col);
                        0
                    }
                    x => x - b'a',
                };
                board.push(Square {
                    pos: (row, col),
                    height,
                });
            }
        }
        let mut distances = std::iter::repeat(None)
            .take(board.len())
            .collect::<Vec<Option<u64>>>();
        let index = board_size.1 * end_pos.0 + end_pos.1;
        distances[index] = Some(0);
        let mut game = Game {
            board,
            distances,
            board_size,
            start_pos,
            end_pos,
        };
        game._set_distances();
        game
    }
}

impl Game {
    fn pos_to_index(&self, pos: &(usize, usize)) -> usize {
        self.board_size.1 * pos.0 + pos.1
    }
    fn _get_adjacent_from(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut vec = Vec::new();
        let index = self.pos_to_index(pos);
        if pos.0 != 0 {
            let new_pos = (pos.0 - 1, pos.1);
            let new_index = self.pos_to_index(&new_pos);
            if self
                .board
                .get(new_index)
                .unwrap()
                .can_go_to(self.board.get(index).unwrap())
            {
                vec.push(new_pos);
            }
        }
        if pos.0 + 1 < self.board_size.0 {
            let new_pos = (pos.0 + 1, pos.1);
            let new_index = self.pos_to_index(&new_pos);
            if self.board[new_index].can_go_to(&self.board[index]) {
                vec.push(new_pos);
            }
        }
        if pos.1 != 0 {
            let new_pos = (pos.0, pos.1 - 1);
            let new_index = self.pos_to_index(&new_pos);
            if self.board[new_index].can_go_to(&self.board[index]) {
                vec.push(new_pos);
            }
        }
        if pos.1 + 1 < self.board_size.1 {
            let new_pos = (pos.0, pos.1 + 1);
            let new_index = self.pos_to_index(&new_pos);
            if self.board[new_index].can_go_to(&self.board[index]) {
                vec.push(new_pos);
            }
        }
        vec
    }
    fn _set_distances(&mut self) {
        let mut queue = VecDeque::new();
        queue.push_back(self.end_pos);
        while let Some(cur_pos) = queue.pop_front() {
            let adjacent = self._get_adjacent_from(&cur_pos);
            let cur_index = self.pos_to_index(&cur_pos);
            let cur_dist = self.distances.get(cur_index).unwrap().unwrap();
            adjacent
                .iter()
                .filter(|pos| {
                    let index = self.pos_to_index(pos);
                    match self.distances.get(index).unwrap() {
                        None => {
                            self.distances[index] = Some(cur_dist + 1);
                            true
                        }
                        Some(check_dist) if *check_dist > cur_dist + 1 => {
                            self.distances[index] = Some(cur_dist + 1);
                            true
                        }
                        _ => false,
                    }
                })
                .for_each(|check_pos| {
                    queue.push_back(*check_pos);
                });
        }
    }
    pub fn get_distance_to_end(&self, pos: &(usize, usize)) -> Option<u64> {
        *self.distances.get(self.pos_to_index(pos)).unwrap()
    }
    pub fn get_all_at_height(&self, height: u8) -> Vec<Square> {
        self.board
            .iter()
            .filter(|x| x.height == height)
            .copied()
            .collect()
    }
}
fn main() -> Result<(), BoxError> {
    let file = File::open("./test.txt")?;
    let game: Game = file.into();
    let start = game.start_pos;
    let s = game
        .get_distance_to_end(&start)
        .expect("No end to this game");
    println!("Part 1: {s}");
    let shortest = game
        .get_all_at_height(0)
        .iter()
        .filter_map(|x| game.get_distance_to_end(&x.pos))
        .min()
        .expect("problem");
    println!("Part 2: {shortest}");

    Ok(())
}
