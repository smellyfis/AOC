#![warn(clippy::all, clippy::pedantic)]

use error_stack::Result;
use glam::IVec2;
use thiserror::Error;

// day-4
#[derive(Debug, Error)]
pub enum Day4Part1Error {
    #[error("Problem parsing Day 4")]
    ParseError,
}

pub fn part1(input: &str) -> Result<String, Day4Part1Error> {
    //read in grid
    let grid = input
        .lines()
        .map(|line| Vec::from(line.as_bytes()))
        .collect::<Vec<_>>();
    let num_of_rows = grid.len().try_into().unwrap();
    let num_of_cols = grid[0].len().try_into().unwrap(); //because we know it will be rectangular
                                                         //window over each letter (skip over not x's
    let total: usize = grid
        .iter()
        .enumerate()
        .map(|(row_num, row)| {
            row.iter()
                .enumerate()
                .map(|(col_num, col)| {
                    if *col == b'X' {
                        //window over the rest
                        let point =
                            IVec2::new(row_num.try_into().unwrap(), col_num.try_into().unwrap());
                        [
                            IVec2::NEG_X,
                            IVec2::NEG_ONE,
                            IVec2::NEG_Y,
                            IVec2::new(1, -1),
                            IVec2::X,
                            IVec2::ONE,
                            IVec2::Y,
                            IVec2::new(-1, 1),
                        ]
                        .iter()
                        .filter(|dir| {
                            let extent = point + (*dir * 3);
                            if extent.x < 0
                                || extent.x >= num_of_rows
                                || extent.y < 0
                                || extent.y >= num_of_cols
                            {
                                return false;
                            }
                            let m = point + *dir;
                            let a = point + 2 * *dir;
                            let s = point + 3 * *dir;
                            grid[m.x as u32 as usize][m.y as u32 as usize] == b'M'
                                && grid[a.x as u32 as usize][a.y as u32 as usize] == b'A'
                                && grid[s.x as u32 as usize][s.y as u32 as usize] == b'S'
                        })
                        .count()
                        //todo!("at pos {row_num} - {col_num}")
                    } else {
                        0_usize
                    }
                })
                .sum::<usize>()
        })
        .sum();
    //count
    Ok(total.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part1_works() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, "18".to_string());
    }
}

