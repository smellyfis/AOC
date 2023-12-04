#![warn(clippy::all, clippy::pedantic)]

use nom::{ bytes::complete::tag,
    character::complete,
    IResult,
    multi::separated_list1,
    sequence::{preceded, separated_pair}};

struct Card {
    pub _id: u8,
    pub game_numbers: Vec<u8>,
    pub my_numbers: Vec<u8>,
}

impl Card {
    fn get_score(&self) -> Option<usize>{
        let count = self.my_numbers.iter().filter(|x| self.game_numbers.contains(x)).count();
        if count == 0 {
            None
        } else {
            Some(2_usize.pow(count as u32- 1))
        }
    }
}

pub fn part1(input: &str) -> String {
    let (_, cards) = parse_input(input).expect("there should be input");
    cards.iter().filter_map(Card::get_score).sum::<usize>().to_string()
}

fn parse_num_list (input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(complete::space1, complete::u8)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, (Vec<u8>, Vec<u8>)> {
    separated_pair(parse_num_list, barspace, parse_num_list)(input)
}

fn barspace(input: &str) -> IResult<&str, () > {
    let (i, _) = complete::space1(input)?;
    let (i,_) = tag("|")(i)?;
    let (i, _) = complete::space1(i)?;
    Ok((i, ()))
}

fn colonspace(input: &str) -> IResult<&str, () > {
    let (i,_) = tag(":")(input)?;
    let (i, _) = complete::space1(i)?;
    Ok((i, ()))
}

fn cardspace(input: &str) -> IResult<&str, () > {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = complete::space1(input)?;
    Ok((input, ()))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, (id, (my_numbers, game_numbers))) = separated_pair(preceded(cardspace, complete::u8), colonspace, parse_numbers )(input)?;

    Ok((input, Card{_id:id, my_numbers, game_numbers}))
}

fn parse_input(input:&str) -> IResult<&str, Vec<Card>> {
    separated_list1(complete::newline, parse_card)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "13".to_string());
    }
}
