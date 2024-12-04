#![warn(clippy::all, clippy::pedantic)]

use error_stack::Result;
use glam::IVec2;
use thiserror::Error;

// day-4
#[derive(Debug, Error)]
pub enum Day4Part2Error {
    #[error("Problem parsing Day 4")]
    ParseError,
}

pub fn part2(input: &str) -> Result<String, Day4Part2Error> {
    //read in grid
    let grid = input
        .lines()
        .map(|line| Vec::from(line.as_bytes()))
        .collect::<Vec<_>>();
    let num_of_rows = grid.len();
    let num_of_cols = grid[0].len();
    //window over each letter (skip over not x's
    let total: usize = grid
        .iter()
        .enumerate()
        .skip(1)
        .take(num_of_rows - 2)
        .map(|(row_num, row)| {
            row.iter()
                .enumerate()
                .skip(1)
                .take(num_of_cols - 2)
                .map(|(col_num, col)| {
                    if *col == b'A' {
                        //window over the rest
                        let point =
                            IVec2::new(row_num.try_into().unwrap(), col_num.try_into().unwrap());
                        let up_forward = point + IVec2::new(-1, 1);
                        let up_back = point + IVec2::NEG_ONE;
                        let down_forward = point + IVec2::ONE;
                        let down_back = point + IVec2::new(1, -1);
                        if ((grid[up_back.x as u32 as usize][up_back.y as u32 as usize] == b'M'
                            && grid[down_forward.x as u32 as usize]
                                [down_forward.y as u32 as usize]
                                == b'S')
                            || (grid[up_back.x as u32 as usize][up_back.y as u32 as usize] == b'S'
                                && grid[down_forward.x as u32 as usize]
                                    [down_forward.y as u32 as usize]
                                    == b'M'))
                            && ((grid[down_back.x as u32 as usize][down_back.y as u32 as usize]
                                == b'M'
                                && grid[up_forward.x as u32 as usize]
                                    [up_forward.y as u32 as usize]
                                    == b'S')
                                || (grid[down_back.x as u32 as usize][down_back.y as u32 as usize]
                                    == b'S'
                                    && grid[up_forward.x as u32 as usize]
                                        [up_forward.y as u32 as usize]
                                        == b'M'))
                        {
                            //println!(" found at {}-{}", point.x, point.y);
                            1
                        } else {
                            0
                        }
                        // todo!("at pos {row_num} - {col_num}")
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
    fn part2_works() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, "9".to_string());
    }
}

