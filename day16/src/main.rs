#![warn(clippy::all, clippy::pedantic)]

use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    error::Error,
    multi::{separated_list0, separated_list1},
    sequence::preceded,
    Parser,
};

#[derive(Debug, Clone, Default)]
struct Valve {
    pub label: String,
    pub release: usize,
    pub connected_to: Vec<String>,
}
impl<'a> Parser<&'a str, Self, Error<&'a str>> for Valve {
    fn parse(&mut self, input: &'a str) -> nom::IResult<&'a str, Self, Error<&'a str>> {
        let (input, label) =
            preceded(tag("Valve "), complete::alpha1.map(ToOwned::to_owned))(input)?;
        let (input, release) =
            preceded(tag(" has flow rate="), complete::u128.map(|s| s as usize))(input)?;
        let (input, connected_to) = preceded(
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), complete::alpha1.map(ToOwned::to_owned)),
        )(input)?;
        self.label = label;
        self.release = release;
        self.connected_to = connected_to.clone();
        Ok((input, self.clone()))
    }
}

//* nom parser to take string input and turn it in to a hashmaps of valves */
fn parse_input(input: &str) -> nom::IResult<&str, HashMap<String, Valve>> {
    let (input, valves) = separated_list0(newline, Valve::default())(input)?;
    Ok((
        input,
        valves
            .iter()
            .map(|x| (x.label.clone(), x.clone()))
            .collect(),
    ))
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
struct SimpleValve {
    pub _label: String,
    pub release: usize,
    pub connected_to: Vec<(usize, usize)>,
}

//* takes a map of valves represented by strings and removes the unnexasrray steps and will create a complete graph of the connected */
fn convert_to_distance(board: &HashMap<String, Valve>) -> Vec<SimpleValve> {
    let care_about = board
        .iter()
        .filter(|(pos, valve)| valve.release != 0 || *pos == "AA")
        .collect::<Vec<_>>();
    let mut distances: HashMap<(&str, &str), usize> = HashMap::new();
    //get the distance for each possible connection
    for (from_pos, from_valve) in &care_about {
        let mut queue = VecDeque::from([(0_usize, *from_valve)]);
        while let Some((dist, check)) = queue.pop_front() {
            let dist = dist + 1;
            for v in check.connected_to.iter().filter_map(|i| board.get(i)) {
                if let Some(d) = distances.get(&(from_pos, &v.label)) {
                    if dist >= *d {
                        continue;
                    }
                }
                distances.insert((from_pos, &v.label), dist);

                queue.push_back((dist, v));
            }
        }
    }

    //makeing distances immutable
    let distances = distances;
    let holder = care_about
        .iter()
        .sorted_by(|(a, _), (b, _)| (*a).cmp(*b))
        .map(|(a, _)| (*a).clone())
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect::<HashMap<_, _>>();
    care_about
        .iter()
        .map(|(pos, valve)| {
            let connected_to = care_about
                .iter()
                .filter(|(key, _)| "AA" != *key && **pos != **key)
                .map(|(to_pos, _)| {
                    (
                        *holder.get(*to_pos).unwrap(),
                        *distances.get(&(pos, *to_pos)).unwrap(),
                    )
                })
                .collect();
            (
                (*pos).clone(),
                SimpleValve {
                    _label: valve.label.clone(),
                    release: valve.release,
                    connected_to,
                },
            )
        })
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, a)| a)
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StructKey(
    /*step*/ usize,
    /*valve*/ usize,
    /*pathmask*/ Pathmask,
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pathmask {
    pub mask: usize,
}
impl From<usize> for Pathmask {
    fn from(mask: usize) -> Self {
        Self { mask }
    }
}
impl From<Pathmask> for usize {
    fn from(mask: Pathmask) -> Self {
        mask.mask
    }
}

fn recurse(
    board: &Vec<SimpleValve>,
    vavle_index: usize,
    path: Pathmask,
    steps: usize,
    score: usize,
    cache: &mut HashMap<StructKey, usize>,
) -> usize {
    //has this been cached?
    if let Some(sc) = cache.get(&StructKey(steps, vavle_index, path)) {
        println!("choo");
        return *sc;
    }
    let valve = board.get(vavle_index).unwrap();
    let new_path =
        Pathmask::from(std::convert::Into::<usize>::into(path) | (1_usize << vavle_index));
    let Some(v) = valve
        .connected_to
        .iter()
        .filter(|x| std::convert::Into::<usize>::into(path) & (1 << x.0) == 0 && steps > x.1 + 1)
        .map(|(next, new_dist)| {
            let new_steps = steps - 1 - new_dist;
            let score = score + board.get(*next).unwrap().release * new_steps;
            recurse(board, *next, new_path, new_steps, score, cache)
        })
        .fold(None, |acc: Option<usize>, new_spot: usize| {
            if acc.is_none() || new_spot > acc.unwrap() {
                Some(new_spot)
            } else {
                acc
            }
        })
    else {
        return score;
    };
    cache.insert(StructKey(steps, score, path), v);
    v
}

fn part1(input: &str) -> String {
    //explore graph
    let board = convert_to_distance(&parse_input(input).unwrap().1);
    let cache = &mut HashMap::new();
    let r = recurse(&board, 0, 0.into(), 30, 0, cache);
    r.to_string()
}

fn part2(input: &str) -> String {
    let board = convert_to_distance(&parse_input(input).unwrap().1);
    let cache = &mut HashMap::new();
    let top_mask: usize = (1 << board.len()) - 1;
    (0..=((top_mask + 1) / 2))
        .map(|i| {
            recurse(&board, 0, Pathmask::from(i), 26, 0, cache)
                + recurse(&board, 0, Pathmask::from(top_mask ^ i), 26, 0, cache)
        })
        .max()
        .unwrap()
        .to_string()
}
//if open don't close
fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), "1651")
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), "1707")
    }
    #[test]
    fn testssss() {
        let board = convert_to_distance(&parse_input(INPUT).unwrap().1);
        let top_mask: usize = (1 << board.len()) - 1;
        (0..=((top_mask + 1) / 2)).for_each(|i| {
            let other = i ^ top_mask;
            let test = i | other;

            assert_eq!(test, top_mask, "{i}");
        });
    }
}
