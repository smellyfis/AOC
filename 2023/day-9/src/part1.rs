#![warn(clippy::all, clippy::pedantic)]

use nom::{character::complete, multi::separated_list1, IResult};
use std::{iter::successors, ops::Not};

/// day 9 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part1(input: &str) -> String {
    let (_, report) = parse_input(input).expect("should have valid input for aoc");
    report.iter().map(|x| get_next(x)).sum::<i64>().to_string()
}

fn get_next(array: &[i64]) -> i64 {
    let array = Vec::from(array);
    successors(Some(array), |a| {
        a.iter()
            .all(|x| x == &0)
            .not()
            .then_some(a.windows(2).map(|a| a[1] - a[0]).collect::<Vec<_>>())
    })
    .map(|x| *x.last().unwrap())
    .sum()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(
        complete::line_ending,
        separated_list1(complete::space1, complete::i64),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![0,3,6,9,12,15], 18)]
    #[case(vec![1,3,6,10,15,21], 28)]
    #[case(vec![10,13,16,21,30,45], 68)]
    fn part1_next(#[case] array: Vec<i64>, #[case] expected: i64) {
        assert_eq!(get_next(&array), expected);
    }

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "114".to_string());
    }
}

