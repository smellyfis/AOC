#![warn(clippy::all, clippy::pedantic)]

use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_board(file: &File) -> Vec<Vec<u8>> {
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .matches(char::is_numeric)
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;

    let board = get_board(&file);

    let y_len = board.len();
    let x_len = board.iter().map(std::vec::Vec::len).max().unwrap();
    assert!(board.iter().any(|x| x.len() != x_len), "board isn't square");

    let mut visible: Vec<(usize, usize, u8)> = Vec::new();
    let mut max_in_row_from_left = vec![0_usize; y_len];
    let mut max_in_row_from_right = vec![x_len - 1; y_len];
    let mut max_in_col_from_top = vec![0_usize; x_len];
    let mut max_in_col_from_bottom = vec![y_len - 1; x_len];
    let mut scores: Vec<Vec<usize>> = vec![vec![0; x_len]; y_len];

    for y in 0..y_len {
        for x in 0..x_len {
            // Part 1 stuff
            let y_inv = y_len - 1 - y;
            let x_inv = x_len - 1 - x;
            let tree_from_top_left = board[y][x];
            let tree_from_bottom_right = board[y_inv][x_inv];
            let max_from_left = board[y][max_in_row_from_left[y]];
            let max_from_right = board[y_inv][max_in_row_from_right[y_inv]];
            let max_from_top = board[max_in_col_from_top[x]][x];
            let max_from_bottom = board[max_in_col_from_bottom[x_inv]][x_inv];
            if tree_from_top_left > max_from_left || x == max_in_row_from_left[y] {
                visible.push((x, y, board[y][x]));
                max_in_row_from_left[y] = x;
            }
            if tree_from_bottom_right > max_from_right || x_inv == max_in_row_from_right[y_inv] {
                visible.push((x_inv, y_inv, board[y_inv][x_inv]));
                max_in_row_from_right[y_inv] = x_inv;
            }
            if tree_from_top_left > max_from_top || y == max_in_col_from_top[x] {
                visible.push((x, y, board[y][x]));
                max_in_col_from_top[x] = y;
            }
            if tree_from_bottom_right > max_from_bottom || y_inv == max_in_col_from_bottom[x_inv] {
                visible.push((x_inv, y_inv, board[y_inv][x_inv]));
                max_in_col_from_bottom[x_inv] = y_inv;
            }

            //part2 stuff
            //search left
            let right_part = board[y].iter().skip(x).copied().collect::<Vec<u8>>();
            let right_score = if right_part.len() > 1 {
                let score = right_part
                    .iter()
                    .skip(1)
                    .copied()
                    .take_while(|tree| *tree < tree_from_top_left)
                    .count()
                    + 1;
                if score + x >= x_len {
                    score - 1
                } else {
                    score
                }
            } else {
                0
            };
            //            println!("Right: {}, {:?}", right_score, right_part);
            let left_part = board[y].iter().rev().skip(x_inv).collect::<Vec<_>>();
            let left_score = if left_part.len() > 1 {
                let score = left_part
                    .iter()
                    .skip(1)
                    .take_while(|tree| ***tree < tree_from_top_left)
                    .count()
                    + 1;
                if score >= x {
                    score - 1
                } else {
                    score
                }
            } else {
                0
            };
            //            println!("Left: {}, {:?}", left_score, left_part);
            let down_part = board.iter().map(|row| row[x]).skip(y).collect::<Vec<_>>();
            let down_score = if down_part.len() > 1 {
                let score = down_part
                    .iter()
                    .skip(1)
                    .take_while(|tree| **tree < tree_from_top_left)
                    .count()
                    + 1;
                if score + y >= y_len {
                    score - 1
                } else {
                    score
                }
            } else {
                0
            };
            //            println!("Down: {}, {:?}", down_score, down_part);
            let up_part = board
                .iter()
                .map(|row| row[x])
                .rev()
                .skip(y_inv)
                .collect::<Vec<_>>();
            let up_score = if up_part.len() > 1 {
                let score = up_part
                    .iter()
                    .skip(1)
                    .take_while(|tree| **tree < tree_from_top_left)
                    .count()
                    + 1;
                if score >= y {
                    score - 1
                } else {
                    score
                }
            } else {
                0
            };
            //            println!("Up: {}. {:?}", up_score, up_part);
            let tree_score = right_score * left_score * down_score * up_score;
            //            println!(
            //                "({}, {})({}) = {} = {} * {} * {} * {}",
            //                x, y, tree_from_top_left, tree_score, up_score, left_score, down_score, right_score
            //            );
            scores[y][x] = tree_score;
        }
    }
    let part1 = visible.iter().unique().count();
    println!("Part 1: {part1}");

    let part2 = scores
        .iter()
        .map(|x| x.iter().max().unwrap())
        .max()
        .unwrap();
    println!("Part 2: {part2}");

    Ok(())
}
