#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use glam::{UVec3, Vec3Swizzles};
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Brick {
    pub cubes: Vec<UVec3>,
}

impl Brick {
    fn supports(&self, other: &Self) -> bool {
        let max_cubes = self.cubes.iter().max_set_by_key(|cube| cube.z);
        let min_cubes = other.cubes.iter().min_set_by_key(|cube| cube.z);
        let top = max_cubes[0].z;
        let bottom = min_cubes[0].z;
        if top + 1 != bottom {
            return false;
        }
        max_cubes.iter().any(|cube| {
            min_cubes
                .iter()
                .map(|t_cube| t_cube.xy())
                .contains(&cube.xy())
        })
    }

    #[allow(dead_code)]
    fn supported_by(&self, other: &Self) -> bool {
        other.supports(self)
    }
}

/// day 22 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part1(input: &str) -> String {
    let (_, mut bricks) = parse_input(input).expect("AOC should have valid input");
    bricks.sort_by(|a, b| {
        a.cubes
            .iter()
            .map(|cube| cube.z)
            .min()
            .unwrap()
            .cmp(&b.cubes.iter().map(|cube| cube.z).min().unwrap())
    });
    //lower the bricks
    let stacked = bricks.iter().fold(Vec::new(), |mut acc, brick| {
        let bottom_cubes = brick.cubes.iter().min_set_by_key(|cube| cube.z);
        let bottom_z = bottom_cubes.iter().map(|cube| cube.z).min().unwrap();
        let top_underneath_cubes = acc
            .iter()
            .flat_map(|settle_brick: &Brick| settle_brick.cubes.iter())
            .filter_map(|cube| {
                bottom_cubes
                    .iter()
                    .map(|c| c.xy())
                    .contains(&cube.xy())
                    .then_some(cube.z)
            })
            .max()
            .unwrap_or(0);
        let step_down = bottom_z - top_underneath_cubes - 1;
        let new_cubes = brick
            .cubes
            .iter()
            .map(|cube| *cube - (UVec3::Z * step_down))
            .collect();
        acc.push(Brick { cubes: new_cubes });

        acc
    });
    let stack_len = stacked.len();

    //do the check
    let undisolvable = stacked
        .into_iter()
        .permutations(2)
        //.inspect(|a| println!("{a:?}"))
        .filter_map(|a| {
            let [ref a_brick, ref b_brick] = a[..] else {
                unimplemented!("should neer reach here")
            };
            a_brick
                .supports(b_brick)
                .then_some((a_brick.clone(), b_brick.clone()))
        })
        .fold(HashMap::new(), |mut acc, (a_brick, b_brick)| {
            acc.entry(b_brick)
                .and_modify(|v: &mut Vec<Brick>| v.push(a_brick.clone()))
                .or_insert(vec![a_brick.clone()]);
            acc
        })
        .values()
        .filter_map(|v| (v.len() == 1).then_some(v[0].clone()))
        .unique()
        //.inspect(|a| println!("{a:?}"))
        .count();
    (stack_len - undisolvable).to_string()
}

fn parse_corner(input: &str) -> IResult<&str, UVec3> {
    let (input, x) = complete::u32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::u32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = complete::u32(input)?;
    Ok((input, UVec3::new(x, y, z)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(
        complete::line_ending,
        separated_pair(parse_corner, tag("~"), parse_corner).map(|(a, b)| {
            let mut cubes = Vec::new();
            for x in (a.x.min(b.x))..=(a.x.max(b.x)) {
                for y in (a.y.min(b.y))..=(a.y.max(b.y)) {
                    for z in (a.z.min(b.z))..=(a.z.max(b.z)) {
                        cubes.push(UVec3::new(x, y, z));
                    }
                }
            }
            Brick { cubes }
        }),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "5".to_string());
    }
}
