#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashSet, ops::Sub};

use glam::IVec2;
use thiserror::Error;

// day-6
#[derive(Debug, Error)]
pub enum Day6Part1Error {
    #[error("Problem parsing Day 6")]
    ParseError,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<&Direction> for IVec2 {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => IVec2::NEG_X,
            Direction::East => IVec2::Y,
            Direction::South => IVec2::X,
            Direction::West => IVec2::NEG_Y,
        }
    }
}

impl Sub<&Direction> for &IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: &Direction) -> Self::Output {
        self - (match rhs {
            Direction::North => IVec2::NEG_X,
            Direction::East => IVec2::Y,
            Direction::South => IVec2::X,
            Direction::West => IVec2::NEG_Y,
        })
    }
}

impl Direction {
    pub fn next(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug)]
struct MyMap {
    pub obstacles: HashSet<IVec2>,
    pub height: u32,
    pub width: u32,
}

impl MyMap {
    pub fn next_obstacle(&self, start_pos: &IVec2, direction: &Direction) -> Option<&IVec2> {
        self.obstacles
            .iter()
            .filter(|obstacle| match direction {
                Direction::North => obstacle.y == start_pos.y && obstacle.x < start_pos.x,
                Direction::East => obstacle.x == start_pos.x && obstacle.y > start_pos.y,
                Direction::South => obstacle.y == start_pos.y && obstacle.x > start_pos.x,
                Direction::West => obstacle.x == start_pos.x && obstacle.y < start_pos.y,
            })
            .fold(None, |acc, obstacle| match direction {
                Direction::North if acc == None || obstacle.x > acc.unwrap().x => Some(obstacle),
                Direction::East if acc == None || obstacle.y < acc.unwrap().y => Some(obstacle),
                Direction::South if acc == None || obstacle.x < acc.unwrap().x => Some(obstacle),
                Direction::West if acc == None || obstacle.y > acc.unwrap().y => Some(obstacle),
                _ => acc,
            })
    }
}

/// Day-6 Part 2 for 2024 advent of code
/// Problem can be found here: <https://adventofcode.com/2024/day/6>
///
/// # Errors
/// - `ParseError` there was an issue with the parser
pub fn part1(input: &str) -> Result<String, Day6Part1Error> {
    //let input = Span::new(input);
    //TODO figure out how to real error
    let (mut guard_pos, map) = parse_input(input);
    let mut guard_dir = Direction::North;
    let mut visited = HashSet::new();
    while guard_pos.x >= 0
        && guard_pos.y >= 0
        && (guard_pos.x as u32) < map.height
        && (guard_pos.y as u32) < map.width
    {
        let _ = visited.insert(guard_pos);
        if let Some(next_obstacle) = map.next_obstacle(&guard_pos, &guard_dir) {
        //    println!("Hit row {}, col {} going {:?}", next_obstacle.x, next_obstacle.y, guard_dir);
            match guard_dir {
                Direction::North => {
                    ((next_obstacle.x+1)..guard_pos.x)
                        .map(|x| IVec2::new(x, guard_pos.y))
                        .for_each(|x| {
                            visited.insert(x);
                        });
                },
                Direction::South => {
                    (guard_pos.x..(next_obstacle.x-1))
                        .map(|x| IVec2::new(x, guard_pos.y))
                        .for_each(|x| {
                            visited.insert(x);
                        });
                },
                Direction::East => {
                    (guard_pos.y..(next_obstacle.y-1))
                        .map(|y| IVec2::new(guard_pos.x, y))
                        .for_each(|x| {
                            visited.insert(x);
                        });
                },
                Direction::West => {
                    ((next_obstacle.y+1)..guard_pos.y)
                        .map(|y| IVec2::new(guard_pos.x, y))
                        .for_each(|x| {
                            visited.insert(x);
                        });
                },
            };
            guard_pos = next_obstacle - &guard_dir;
        } else {
            let new_pos = match guard_dir {
                Direction::North => IVec2::new(-1, guard_pos.y),
                Direction::East => IVec2::new(guard_pos.x, map.width.try_into().unwrap()),
                Direction::South => IVec2::new(map.height.try_into().unwrap(), guard_pos.y),
                Direction::West => IVec2::new(guard_pos.x, -1),
            };
      //      println!("Left map at row {}, col {}", new_pos.x, new_pos.y);
            match guard_dir {
                Direction::North => {
                    ((new_pos.x+1)..guard_pos.x)
                        .map(|x| IVec2::new(x, guard_pos.y))
                        .for_each(|x| {
                            visited.insert(x);
                        });
                }
                Direction::South => {
                    (guard_pos.x..new_pos.x)
                        .map(|x| IVec2::new(x, guard_pos.y))
                        .for_each(|x| {
                            visited.insert(x);
                        });
                }
                Direction::East => {
                    (guard_pos.y..new_pos.y)
                        .map(|y| IVec2::new(guard_pos.x, y))
                        .for_each(|x| {
                            visited.insert(x);
                        });
                }
                Direction::West => {
                    ((new_pos.y+1)..guard_pos.y)
                        .map(|y| IVec2::new(guard_pos.x, y))
                        .for_each(|x| {
                            visited.insert(x);
                        });
                }
            };
            guard_pos = new_pos;
            //break
        }
        guard_dir = guard_dir.next();
        /*
    for row in 0.. map.height.try_into().unwrap() {
        for col in 0..map.width.try_into().unwrap() {
            let pos = IVec2::new(row, col);
            if visited.contains(&pos) {
                print!("X");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }*/
    }

    Ok(visited.iter().count().to_string())
}

fn parse_input(input: &str) -> (IVec2, MyMap) {
    let (pos, height, width, obstacles) = input.lines().into_iter().enumerate().fold(
        (IVec2::ZERO, 0, 0, HashSet::new()),
        |mut acc, (row_no, row)| {
            acc.1 = row_no.try_into().unwrap();
            acc.2 = row.len().try_into().unwrap();
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#' || *c == '^')
                .for_each(|(col_no, c)| {
                    if c == '#' {
                        acc.3.insert(IVec2::new(
                            row_no.try_into().unwrap(),
                            col_no.try_into().unwrap(),
                        ));
                    } else {
                        acc.0 = IVec2::new(row_no.try_into().unwrap(), col_no.try_into().unwrap());
                    }
                });
            acc
        },
    );
    (
        pos,
        MyMap {
            obstacles,
            height: height + 1,
            width,
        },
    )
}
#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part1_works() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, "41".to_string());
    }
}

