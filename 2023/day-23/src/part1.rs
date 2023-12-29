#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use glam::IVec2;
use petgraph::{algo, prelude::*};

#[derive(Debug, Copy, Clone)]
enum PointType {
    Any,
    OnlyDown,
    OnlyLeft,
    OnlyRight,
    OnlyUp,
}

impl PointType {
    fn next_possibles(self) -> Vec<IVec2> {
        match self {
            PointType::Any => vec![IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y],
            PointType::OnlyDown => vec![IVec2::Y],
            PointType::OnlyLeft => vec![IVec2::NEG_X],
            PointType::OnlyRight => vec![IVec2::X],
            PointType::OnlyUp => vec![IVec2::NEG_Y],
        }
    }
}

/// day 23 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part1(input: &str) -> String {
    let maze = parse_input(input);
    //get the start position (assuming there is only one)
    let start = *maze.keys().find(|pos| pos.y == 0).unwrap();
    let end = maze.keys().fold(IVec2::splat(0), |max, current| {
        if max.y.max(current.y) == current.y {
            *current
        } else {
            max
        }
    });
    let mut maze_graph = DiGraph::<&PointType, u32>::new();
    let node_map = maze
        .iter()
        .map(|(pos, point_type)| (pos, maze_graph.add_node(point_type)))
        .collect::<HashMap<_, _>>();

    maze.iter()
        .flat_map(|(pos, point_type)| {
            point_type
                .next_possibles()
                .iter()
                .copied()
                .filter_map(|dir| {
                    let next_pos = dir + *pos;
                    node_map
                        .get(&next_pos)
                        .is_some()
                        .then(|| (node_map[pos], node_map[&next_pos], 1))
                })
                .collect::<Vec<_>>()
        })
        .for_each(|(a, b, weight)| {
            maze_graph.add_edge(a, b, weight);
        });

    (algo::all_simple_paths::<Vec<_>, _>(&maze_graph, node_map[&start], node_map[&end], 0, None)
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len()
        - 1)
    .to_string()
}

fn parse_input(input: &str) -> HashMap<IVec2, PointType> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, c)| {
                let pos = IVec2::new(i32::try_from(x).unwrap(), i32::try_from(y).unwrap());
                match c {
                    '.' => Some((pos, PointType::Any)),
                    '>' => Some((pos, PointType::OnlyRight)),
                    'v' => Some((pos, PointType::OnlyDown)),
                    '^' => Some((pos, PointType::OnlyUp)),
                    '<' => Some((pos, PointType::OnlyLeft)),
                    _ => None,
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "94".to_string());
    }
}
