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
pub fn part2(input: &str) -> String {
    let (maxes, grid) = parse_input(input);
    let result = dijkstra(
        &(IVec2::splat(0), VecDeque::from([IVec2::splat(0)])),
        |(pos, lasts)| {
            [
                (4..=10).map(|y| IVec2::new(0, y)).collect::<Vec<_>>(),
                (-10..=-4).map(|y| IVec2::new(0, y)).collect::<Vec<_>>(),
                (4..=10).map(|x| IVec2::new(x, 0)).collect::<Vec<_>>(),
                (-10..=-4).map(|x| IVec2::new(x, 0)).collect::<Vec<_>>(),
            ]
            .into_iter()
            .flatten()
            .filter_map(|x| {
                let next = x + *pos;
                //check that it is on the range
                if !((0..maxes.x).contains(&next.x) && (0..maxes.y).contains(&next.y)) {
                    return None;
                }
                if lasts.len() > 1 && lasts[1] == next {
                    return None;
                }

                let mut next_lasts = lasts.clone();
                next_lasts.push_front(next);
                if next_lasts.len() >= 3 {
                    let dir = (next_lasts[1] - next_lasts[0]).signum();
                    let a = (next_lasts[2] - next_lasts[1]).signum();
                    //let b = (next_lasts[3] - next_lasts[2]).signum();
                    //let c = next_lasts[4] - next_lasts[3]).signum();

                    if a == dir || a *-1 == dir{
                        None
                    } else {
                        next_lasts.pop_back();
                        Some((next, next_lasts))
                    }
                } else {
                    Some((next, next_lasts))
                }
            })
            .map(|pos| {
                let range = pos.0 - pos.1[1];
                let total = if range.x == 0  && range.y > 0 {
                    (0..range.y).map(|y| pos.0 - IVec2::new(0,y)).map(|v| grid.get(&v).unwrap()).sum::<u32>()
                } else if range.x == 0 && range.y < 0 {
                    (range.y+1..=0).map(|y| pos.0 - IVec2::new(0,y)).map(|v| grid.get(&v).unwrap()).sum::<u32>()
                } else if range.y == 0 && range.x > 0 {
                    (0..range.x).map(|x| pos.0 - IVec2::new(x,0)).map(|v| grid.get(&v).unwrap()).sum::<u32>()
                } else {
                    (range.x+1..=0).map(|x| pos.0 - IVec2::new(x,0)).map(|v| grid.get(&v).unwrap()).sum::<u32>()
                };
                (pos, total)
            })
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

    use rstest::rstest;

    #[rstest]
    #[case("2413432311323
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
4322674655533", "94")]
    #[case("111111111111
999999999991
999999999991
999999999991
999999999991", "71")]
    fn part2_works(#[case] input: &str, #[case] expected: &str) {
        let result = part2(input);
        assert_eq!(result, expected);
    }
}

