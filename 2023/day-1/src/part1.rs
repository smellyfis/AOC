#![warn(clippy::all, clippy::pedantic)]

use nom::{
    self,
    character::complete::{alphanumeric1, newline},
    multi::separated_list1,
};

/// Day-1 part 1 of AC2023
///
/// # Arguments
/// - input the input for day1 as a string
///
/// # Panics
/// This panics whenever a number isn't present in a line of the input
///
/// # Errors
/// errors when can't parse the input
pub fn part1(input: &str) -> nom::IResult<&str, String> {
    let (_, values) = parse_input(input)?;
    println!("{values:?}");
    Ok((
        "",
        values
            .iter()
            .map(|v| {
                v.first().expect("always at least one number") * 10
                    + v.last().expect("always atleast one number")
            })
            .sum::<u32>()
            .to_string(),
    ))
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Vec<u32>>> {
    let (i, j) = separated_list1(newline, alphanumeric1)(input)?;
    let res = j
        .iter()
        .map(|v| {
            v.chars()
                .filter_map(|x| x.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    Ok((i, res))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn part1_works() {
        let (_, result) = part1(INPUT).unwrap();
        assert_eq!(result, "142".to_string());
    }
}
