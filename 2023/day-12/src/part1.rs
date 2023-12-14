#![warn(clippy::all, clippy::pedantic)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum SpringStatus {
    Working,
    Failing,
    Unknown,
}

struct Row {
    springs: Vec<SpringStatus>,
    broken_spans: Vec<u32>,
}

impl Row {
    fn process(&self) -> usize {
        let num_broken = self.broken_spans.iter().sum();
        let row_len = self.springs.len();
        let max_perm = 1_u32 << row_len;
        (1..max_perm)
            .filter(|x| x.count_ones() == num_broken)
            .map(|x| {
                let mut perm = Vec::new();
                (0..row_len).map(|y| 1 << y).for_each(|y| {
                    if y & x == 0 {
                        perm.push(SpringStatus::Working);
                    } else {
                        perm.push(SpringStatus::Failing);
                    }
                });
                perm
            })
            .filter(|x| {
                self.springs
                    .iter()
                    .zip(x.iter())
                    .all(|(a, b)| (a == b || *a == SpringStatus::Unknown))
            })
            .filter(|x| {
                let (mut array, last, current_run) = x.iter().fold(
                    (Vec::new(), SpringStatus::Working, 0_u32),
                    |(mut array, _last, mut current_run), x| {
                        if *x == SpringStatus::Failing {
                            current_run += 1;
                        } else {
                            if current_run > 0 {
                                array.push(current_run);
                            }
                            current_run = 0;
                        }
                        (array, *x, current_run)
                    },
                );
                if last == SpringStatus::Failing {
                    array.push(current_run);
                }
                array
                    .iter()
                    .zip(self.broken_spans.iter())
                    .all(|(a, b)| a == b)
            })
            .count()
    }

    //fn generate_permiatation(&self)
}

/// day 12 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part1(input: &str) -> String {
    let (_, spas) = parse_input(input).expect("AOC always has valid input");
    spas.iter()
        .map(|x| x.process() as u64)
        .sum::<u64>()
        .to_string()
}

fn parse_spa_spots(input: &str) -> IResult<&str, Vec<SpringStatus>> {
    many1(alt((
        tag(".").map(|_| SpringStatus::Working),
        tag("#").map(|_| SpringStatus::Failing),
        tag("?").map(|_| SpringStatus::Unknown),
    )))(input)
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
                    springs,
                    broken_spans,
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
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
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
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "21".to_string());
    }
}
