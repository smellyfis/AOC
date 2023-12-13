#![warn(clippy::all, clippy::pedantic)]

use std::{iter::repeat, collections::HashMap};

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, is_a},
    character::complete,
    multi::separated_list1,
    sequence::separated_pair,
    IResult};


struct Row {
    springs: String,
    broken_spans: Vec<u32>,
}

impl Row {
    fn process(&self) -> usize {
        let mut cache = HashMap::new();
        self.dynamic_search(&mut cache, (0,0,0))
    }

    fn dynamic_search(&self, cache: &mut HashMap<(usize,usize,u32),usize>, search: (usize, usize, u32)) -> usize {
        let (data_index, group_index, group_size) = search;
        //are we at the end of the input
        if data_index >= self.springs.len(){
            // when group_index is greater we are here then golden
            if group_index >= self.broken_spans.len() {
                return 1;
            }

            //we haven't satisfied groups but the end is failing
            if group_index == self.broken_spans.len() - 1 && self.broken_spans[group_index] == group_size {
                return 1;
            }
            return 0;
        }
        match self.springs.as_bytes()[data_index] {
            b'.' => {
                //previous was also working just go to next data point
                if group_size == 0 {
                    return self.dynamic_search(cache, (data_index + 1, group_index, 0));
                }

                //we failed to match the group
                if group_index >= self.broken_spans.len() || self.broken_spans[group_index] != group_size{
                    return 0;
                }

                //completed a group keep going
                self.dynamic_search(cache,(data_index+1, group_index +1, 0))
            },
            b'#' => {
                //too many for our group
                if group_index >= self.broken_spans.len() || group_size + 1 > self.broken_spans[group_index] {
                    return 0;
                }

                //haven't completed group yet keep looking
                self.dynamic_search(cache, (data_index+1, group_index, group_size + 1))
            },
            b'?' => {
                if let Some(res) = cache.get(&(data_index,group_index,group_size)).copied() {
                    return res;
                }

                let mut perms = 0_usize;

                //pretend to be a undamaged, if in a working group
                if 0 == group_size {
                    perms += self.dynamic_search(cache, (data_index +1, group_index, 0));
                }

                //pretend to be damaged
                if group_index < self.broken_spans.len() && group_size < self.broken_spans[group_index] {
                    perms += self.dynamic_search(cache, (data_index + 1, group_index, group_size +1));
                }

                //pretend to be undamage, thus ending a damaged group
                if group_index < self.broken_spans.len() && group_size == self.broken_spans[group_index] {
                    perms += self.dynamic_search(cache, (data_index +1, group_index+1, 0));
                }

                cache.insert((data_index, group_index, group_size),perms);
                perms
            },
            _ => unreachable!(),
        }
    }
}

/// day 12 part 2 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part2(input: &str) -> String {
    let (_, spas) = parse_input(input).expect("AOC always has valid input");
    spas.iter()
        .map(|x| x.process() as u64)
        .sum::<u64>()
        .to_string()
}

fn parse_spa_spots(input: &str) -> IResult<&str, &str> {
    is_a(".#?")(input)
}

fn parse_spa_spans(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32)(input)
}

fn parse_spa_rows(input: &str) -> IResult<&str, Row> {
    separated_pair(parse_spa_spots, complete::space1, parse_spa_spans)(input).map(
        |(input, (springs, broken_spans))| {
            (
                input,
                Row {
                    springs: std::iter::repeat(springs).take(5).join("?"),
                    broken_spans: repeat(broken_spans.iter())
                        .take(5)
                        .flatten()
                        .copied()
                        .collect(),
                },
            )
        },
    )
}

fn parse_input(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(complete::line_ending, parse_spa_rows)(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16_384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2_500)]
    #[case("?###???????? 3,2,1", 506_250)]
    fn line_test(#[case] input: &str, #[case] expected: usize) {
        let (_, row) = parse_spa_rows(input).expect("should parse");
        assert_eq!(row.process(), expected);
    }

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "525152".to_string());
    }
}

