#![warn(clippy::all, clippy::pedantic)]

use glam::I64Vec2;
use itertools::Itertools;
use std::collections::HashSet;

/// day 11 part 2 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part2(input: &str, modr: i64) -> String {
    let points = parse_input(input);
    let ((min_x, min_y), (mut max_x, /*mut*/ max_y)) = points.iter().fold(
        ((i64::MAX, i64::MAX), (i64::MIN, i64::MIN)),
        |((min_x, min_y), (max_x, max_y)), pos| {
            let min_x = min_x.min(pos.x);
            let min_y = min_y.min(pos.y);
            let max_x = max_x.max(pos.x);
            let max_y = max_y.max(pos.y);
            ((min_x, min_y), (max_x, max_y))
        },
    );
    let mut modifier = 0;
    let mut adjusted_points = HashSet::new();
    for x in min_x..=max_x {
        let column = (min_y..=max_y)
            .filter_map(|y| points.get(&(x, y).into()))
            .collect::<Vec<_>>();
        if column.is_empty() {
            modifier += modr-1;
        }
        for point in column {
            adjusted_points.insert(*point + I64Vec2::new(modifier, 0));
        }
    }
    max_x += modifier;

    let mut modifier = 0;
    let mut points = HashSet::new();
    for y in min_y..=max_y {
        let row = (min_x..=max_x)
            .filter_map(|x| adjusted_points.get(&(x, y).into()))
            .collect::<Vec<_>>();
        if row.is_empty() {
            modifier += modr-1;
        }
        for point in row {
            points.insert(*point + I64Vec2::new(0, modifier));
        }
    }
    //max_y += modifier;
    (points
        .iter()
        .cartesian_product(points.iter())
        .filter_map(|(a, b)| (*a != *b).then_some(*a-*b))
        .map(|a| u64::try_from(a.x.abs()+a.y.abs()).unwrap())
        .sum::<u64>()
        / 2)
    .to_string()
}

fn parse_input(input: &str) -> HashSet<I64Vec2> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                (c != '.').then_some(I64Vec2::new(
                    i64::try_from(x).unwrap(),
                    i64::try_from(y).unwrap(),
                ))
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part2_works() {
        let result = part2(INPUT, 10);
        assert_eq!(result, "1030".to_string());
    }
}

