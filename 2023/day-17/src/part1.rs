#![warn(clippy::all, clippy::pedantic)]

use std::{
    collections::{HashMap, VecDeque},
};

use glam::IVec2;
use pathfinding::prelude::dijkstra;

/// day 17 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part1(input: &str) -> String {
    let (maxes, grid) = parse_input(input);
    let result = dijkstra(
        &(IVec2::splat(0), VecDeque::from([IVec2::splat(0)])),
        |(pos, lasts)| {
            [
                IVec2::new(0, 1),
                IVec2::new(0, -1),
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
            ]
            .into_iter()
            .filter_map(|x| {
                let next = x + *pos;
                if !((0..maxes.x).contains(&next.x) && (0..maxes.y).contains(&next.y)) {
                    return None;
                }
                if lasts.len() > 1 && lasts[1] == next {
                    return None;
                }

                let mut next_lasts = lasts.clone();
                next_lasts.push_front(next);
                if next_lasts.len() >= 5 {
                    let dir = next_lasts[1] - next_lasts[0];
                    let a = next_lasts[2] - next_lasts[1];
                    let b = next_lasts[3] - next_lasts[2];
                    let c = next_lasts[4] - next_lasts[3];

                    if [a, b, c].into_iter().all(|x| x == dir) {
                        None
                    } else {
                        next_lasts.pop_back();
                        Some((next, next_lasts))
                    }
                } else {
                    Some((next, next_lasts))
                }
            })
            .map(|pos| (pos.clone(), *grid.get(&pos.0).unwrap()))
            .collect::<Vec<((IVec2, VecDeque<IVec2>), u32)>>()
        },
        |win| win.0 == maxes - 1,
    )
    .expect("a path should be found");

    result.1.to_string()
}

fn parse_input(input: &str) -> (IVec2, HashMap<IVec2, u32>) {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, c)| {
                (
                    IVec2::new(col.try_into().unwrap(), row.try_into().unwrap()),
                    c.to_digit(10).unwrap(),
                )
            })
        })
        .collect();
    let max_row = i32::try_from(input.lines().count()).unwrap();
    let max_col = i32::try_from(input.lines().next().unwrap().len()).unwrap();
    (IVec2::new(max_col, max_row), grid)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "102".to_string());
    }
}

