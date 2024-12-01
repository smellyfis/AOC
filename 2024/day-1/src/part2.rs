#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use error_stack::{Report, Result, ResultExt};
use nom::{character::complete, multi::separated_list1, sequence::separated_pair, IResult};
use thiserror::Error;

// day-1
#[derive(Debug, Error)]
pub enum Day1Part2Error {
    #[error("Problem parsing Day 1")]
    ParseError,
}

pub fn part2(input: &str) -> Result<u64, Day1Part2Error> {
    let (_, (col1, col2)) = parse_input(input)
        .map_err(|x| Report::from(x.to_owned()))
        .change_context(Day1Part2Error::ParseError)?;
    let col2_bucket: HashMap<u64, u64> = col2.into_iter().fold(HashMap::new(), |mut acc, x| {
        let val = acc.entry(x).or_insert(0);
        *val += 1;
        acc
    });
    Ok(col1.iter().map(|x| *x * col2_bucket.get(x).or(Some(&0)).unwrap()).sum())
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, combo) = separated_list1(
        complete::line_ending,
        separated_pair(complete::u64, complete::space1, complete::u64),
    )(input)?;
    Ok((input, combo.into_iter().unzip()))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3 ";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part2_works() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, 31);
    }
}

