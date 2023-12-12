#![warn(clippy::all, clippy::pedantic)]

use std::{collections::BTreeMap, fmt::Display, fs, ops::RangeInclusive};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
};

trait Pos {
    fn get_pos(self) -> (i64, i64);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sensor {
    pub x: i64,
    pub y: i64,
    pub strength: i64,
}

impl Sensor {
    pub fn x_covereage_at_y(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let dist = (y - self.y).abs();
        let n = self.strength - dist;
        if n < 0 {
            return None;
        }
        Some((self.x - n)..=(self.x + n))
    }
    pub fn coverage(&self) -> Vec<(i64, RangeInclusive<i64>)> {
        ((self.y - self.strength)..=(self.y + self.strength))
            .map(|y| {
                let Some(x_s) = self.x_covereage_at_y(y) else {
                    panic!()
                };
                (y, x_s)
            })
            .collect()
    }
}
impl Pos for Sensor {
    fn get_pos(self) -> (i64, i64) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Beacon {
    pub x: i64,
    pub y: i64,
}
impl Pos for Beacon {
    fn get_pos(self) -> (i64, i64) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Square {
    Covered,
    Beacon(Beacon),
    Sensor(Sensor),
}
impl Square {
    pub fn is_covered(&self) -> bool {
        matches!(self, Self::Covered)
    }
}
impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Covered => '#',
                Self::Beacon(_) => 'B',
                Self::Sensor(_) => 'S',
            }
        )
    }
}
impl From<&Sensor> for Square {
    fn from(value: &Sensor) -> Self {
        Self::Sensor(*value)
    }
}
impl From<Sensor> for Square {
    fn from(value: Sensor) -> Self {
        Self::Sensor(value)
    }
}
impl From<&Beacon> for Square {
    fn from(value: &Beacon) -> Self {
        Self::Beacon(*value)
    }
}
impl From<Beacon> for Square {
    fn from(value: Beacon) -> Self {
        Self::Beacon(value)
    }
}

fn parse_reading(input: &str) -> nom::IResult<&str, (Sensor, Beacon)> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, x_sensor) = complete::i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y_sensor) = complete::i64(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, x_beacon) = complete::i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y_beacon) = complete::i64(input)?;
    let dist = (x_sensor - x_beacon).abs() + (y_sensor - y_beacon).abs();

    Ok((
        input,
        (
            Sensor {
                x: x_sensor,
                y: y_sensor,
                strength: dist,
            },
            Beacon {
                x: x_beacon,
                y: y_beacon,
            },
        ),
    ))
}
fn parse_output(input: &str) -> nom::IResult<&str, Vec<Square>> {
    let (input, readings) = separated_list1(newline, parse_reading)(input)?;
    let output = readings
        .iter()
        .flat_map(|(sensor, beacon)| vec![sensor.into(), beacon.into()])
        .unique()
        .collect();

    Ok((input, output))
}

fn coverage_count_at_y(board: &[Square], y: i64) -> Vec<i64> {
    board
        .iter()
        .filter_map(|square| match square {
            Square::Sensor(sensor) => Some(sensor),
            _ => None,
        })
        .filter_map(|sensor| sensor.x_covereage_at_y(y))
        .flatten()
        .unique()
        .collect()
}
pub fn part1(board: &[Square], y: i64) -> String {
    let pos_covered_on_y = coverage_count_at_y(board, y).len();
    let obs_on_y = board
        .iter()
        .filter_map(|square| match square {
            Square::Beacon(beacon) => {
                if beacon.y == y {
                    Some(beacon.x)
                } else {
                    None
                }
            }
            Square::Sensor(sensor) => {
                if sensor.y == y {
                    Some(sensor.x)
                } else {
                    None
                }
            }
            Square::Covered => None,
        })
        .unique()
        .count();
    (pos_covered_on_y - obs_on_y).to_string()
}

pub fn part2(board: &[Square], lower: i64, upper: i64) -> String {
    let bb = board
        .iter()
        .filter_map(|square| match square {
            Square::Sensor(x) => Some(x),
            _ => None,
        })
        .flat_map(|square| {
            square
                .coverage()
                .into_iter()
                .filter(|(y, _)| y >= &lower && y <= &upper)
                .map(|(y, x_s)| (y, *x_s.start().max(&lower)..=*x_s.end().min(&upper)))
        })
        .fold(BTreeMap::new(), |mut acc, (y, x_s)| {
            acc.entry(y)
                .and_modify(|x_range: &mut Vec<RangeInclusive<i64>>| x_range.push(x_s.clone()))
                .or_insert(vec![x_s]);
            acc
        });
    let (x, y) = bb
        .into_iter()
        .find_map(|(y, mut x_s)| {
            x_s.sort_by(|a, b| a.start().cmp(b.start()));
            x_s.iter()
                .fold(
                    (lower..=lower, None),
                    |mut acc: (RangeInclusive<i64>, Option<i64>), x_range: &RangeInclusive<i64>| {
                        if acc.1.is_some() {
                            return acc;
                        }
                        if acc.0.end() + 1 >= *x_range.start() {
                            acc.0 = *acc.0.start()..=*acc.0.end().max(x_range.end());
                        } else {
                            acc.1 = Some(acc.0.end() + 1);
                        }
                        acc
                    },
                )
                .1
                .map(|x| (x, y))
        })
        .unwrap();
    (4_000_000 * x + y).to_string()
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let (_, board) = parse_output(&file).unwrap();

    let y = 2_000_000;

    println!("Part 1: {}", part1(&board, y));

    println!("Part 2: {}", part2(&board, 0, 4_000_000));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_works() {
        let (_, board) = parse_output(INPUT).unwrap();
        assert_eq!(part1(&board, 10), "26");
    }

    #[test]
    fn part2_works() {
        let (_, board) = parse_output(INPUT).unwrap();
        assert_eq!(part2(&board, 0, 20), "56000011");
    }
}
