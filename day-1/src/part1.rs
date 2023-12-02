use nom::{multi::{separated_list1}, self, character::complete::{alphanumeric1, newline}};

pub fn part1(input: &str) -> nom::IResult<&str,String> {
    let (_, values) = parse_input(input)?;
    println!("{values:?}");
    Ok(("", values.iter().map(|v| v.first().unwrap() * 10 + v.last().unwrap() ).sum::<u32>().to_string()))
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Vec<u32>>> {
    let (i, j) = separated_list1(newline, alphanumeric1)(input)?;
    let res = j.iter().map(|v| v.chars().filter_map(|x| x.to_digit(10)).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
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
}}
