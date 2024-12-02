#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use glam::IVec2;
use nom::{bytes::complete::is_a, character::complete, multi::separated_list1, IResult};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Boulder {
    Round,
    Static,
}
impl From<char> for Boulder {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Round,
            '#' => Self::Static,
            x => unimplemented!("there is no boulder type for this charachter {x}"),
        }
    }
}

/// day 14 part 2 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
#[allow(clippy::cast_sign_loss)]
pub fn part2(input: &str) -> String {
    let (_, (maxes, mut map)) = parse_input(input).expect("stuff");

    let cycles = 1_000_000_000;
    let mut cur_cycle = 0;
    let mut cache = HashMap::new();

    let (start_of_cycle, end_of_cycle) = loop {
        cur_cycle += 1;
        if cur_cycle >= cycles {
            break (0, cur_cycle);
        }
        let grid_hash = hash_grid(&map, maxes);
        if let Some((_, cycle)) = cache.get(&grid_hash) {
            break (*cycle, cur_cycle); //reached steady state?
        }
        let next = tilt_north(maxes, &map);
        let next = tilt_west(maxes, &next);
        let next = tilt_south(maxes, &next);
        let next = tilt_east(maxes, &next);
        cache.insert(grid_hash, (next.clone(), cur_cycle));
        map = next;
    };

    let len_of_cyle = end_of_cycle - start_of_cycle;

    let pos_in_cycle = start_of_cycle + (cycles - end_of_cycle) % len_of_cyle;

    map.clone_from(
        cache
            .values()
            .find_map(|(look_at_map, pos)| (*pos == pos_in_cycle).then_some(look_at_map))
            .unwrap(),
    );

    let mut total = 0_usize;
    for col in 0..maxes.x {
        for row in 0..maxes.y {
            total += match map.get(&(col, row).into()) {
                Some(Boulder::Round) => (maxes.y - row) as usize,
                _ => 0,
            }
        }
    }

    total.to_string()
}

fn hash_grid(map: &HashMap<IVec2, Boulder>, maxes: IVec2) -> String {
    (0..maxes.y)
        .flat_map(|y| {
            (0..maxes.x).map(move |x| match map.get(&IVec2::from((x, y))) {
                Some(Boulder::Static) => "#",
                Some(Boulder::Round) => "O",
                _ => ".",
            })
        })
        .collect::<String>()
}

fn _print_grid(map: &HashMap<IVec2, Boulder>, maxes: IVec2) {
    (0..maxes.y).for_each(|y| {
        println!(
            "{}",
            (0..maxes.x)
                .map(move |x| match map.get(&IVec2::from((x, y))) {
                    Some(Boulder::Static) => "#",
                    Some(Boulder::Round) => "O",
                    _ => ".",
                })
                .collect::<String>()
        );
    });
}

fn tilt_north(maxes: IVec2, in_map: &HashMap<IVec2, Boulder>) -> HashMap<IVec2, Boulder> {
    let mut out_map = HashMap::new();

    for col in 0..maxes.x {
        let mut last = 0;
        for row in 0..maxes.y {
            match in_map.get(&IVec2::from((col, row))) {
                Some(Boulder::Static) => {
                    last = row + 1;
                    out_map.insert(IVec2::from((col, row)), Boulder::Static);
                }
                Some(Boulder::Round) => {
                    out_map.insert(IVec2::from((col, last)), Boulder::Round);
                    last += 1;
                }
                _ => {}
            }
        }
    }

    out_map
}

fn tilt_south(maxes: IVec2, in_map: &HashMap<IVec2, Boulder>) -> HashMap<IVec2, Boulder> {
    let mut out_map = HashMap::new();

    for col in 0..maxes.x {
        let mut last = maxes.y - 1;
        for row in (0..maxes.y).rev() {
            match in_map.get(&IVec2::from((col, row))) {
                Some(Boulder::Static) => {
                    last = row - 1;
                    out_map.insert(IVec2::from((col, row)), Boulder::Static);
                }
                Some(Boulder::Round) => {
                    out_map.insert(IVec2::from((col, last)), Boulder::Round);
                    last -= 1;
                }
                _ => {}
            }
        }
    }

    out_map
}

fn tilt_west(maxes: IVec2, in_map: &HashMap<IVec2, Boulder>) -> HashMap<IVec2, Boulder> {
    let mut out_map = HashMap::new();

    for row in 0..maxes.y {
        let mut last = 0;
        for col in 0..maxes.x {
            match in_map.get(&IVec2::from((col, row))) {
                Some(Boulder::Static) => {
                    last = col + 1;
                    out_map.insert(IVec2::from((col, row)), Boulder::Static);
                }
                Some(Boulder::Round) => {
                    out_map.insert(IVec2::from((last, row)), Boulder::Round);
                    last += 1;
                }
                _ => {}
            }
        }
    }

    out_map
}

fn tilt_east(maxes: IVec2, in_map: &HashMap<IVec2, Boulder>) -> HashMap<IVec2, Boulder> {
    let mut out_map = HashMap::new();

    for row in 0..maxes.y {
        let mut last = maxes.x - 1;
        for col in (0..maxes.x).rev() {
            match in_map.get(&IVec2::from((col, row))) {
                Some(Boulder::Static) => {
                    last = col - 1;
                    out_map.insert(IVec2::from((col, row)), Boulder::Static);
                }
                Some(Boulder::Round) => {
                    out_map.insert(IVec2::from((last, row)), Boulder::Round);
                    last -= 1;
                }
                _ => {}
            }
        }
    }

    out_map
}
fn parse_input(input: &str) -> IResult<&str, (IVec2, HashMap<IVec2, Boulder>)> {
    let (input, rows) = separated_list1(complete::line_ending, is_a(".O#"))(input)?;
    let max_rows = i32::try_from(rows.len()).expect("stuff and things");
    let max_cols = i32::try_from(rows[0].len()).expect("things and stuff?");
    let maxs = IVec2::from((max_cols, max_rows));
    let hash = rows
        .iter()
        .enumerate()
        .flat_map(|(line_no, chars)| {
            chars
                .chars()
                .enumerate()
                .filter_map(move |(col_no, c)| {
                    (c != '.').then_some((
                        IVec2::from((
                            i32::try_from(col_no).expect("hopefully not to small"),
                            i32::try_from(line_no).expect("this shouldn't be too big"),
                        )),
                        c,
                    ))
                })
                .map(|(pos, c)| (pos, Boulder::from(c)))
        })
        .collect();
    Ok((input, (maxs, hash)))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "64".to_string());
    }
}
