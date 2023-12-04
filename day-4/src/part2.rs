#![warn(clippy::all, clippy::pedantic)]

use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::map,
    multi::{fold_many1, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::collections::{BTreeMap, HashSet};

struct Card {
    pub id: usize,
    pub game_numbers: HashSet<u8>,
    pub my_numbers: HashSet<u8>,
}
impl Card {
    fn get_win_count(&self) -> usize {
        self.my_numbers.intersection(&self.game_numbers).count()
    }
}

/// day 4 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
/// usize
#[must_use]
pub fn part2(input: &str) -> String {
    let (_, cards) = parse_input(input).expect("there should be input");
    let mut cards_had = BTreeMap::new();
    for card in cards {
        if let Some(x) = cards_had.get_mut(&card.id) {
            *x += 1;
        } else {
            cards_had.insert(card.id, 1);
        }
        let next_id = card.id + 1;
        let last_winner = card.id + card.get_win_count();
        //println!("{} - {next_id} {last_winner}", card.id);
        if last_winner < next_id {
            continue;
        }
        let card_count = *cards_had.get(&card.id).expect("already should have cards");
        for id in next_id..=last_winner {
            if let Some(x) = cards_had.get_mut(&id) {
                *x += card_count;
            } else {
                cards_had.insert(id, card_count);
            }
        }
    }
    //println!("{cards_had:#?}");

    cards_had.values().sum::<usize>().to_string()
}

fn parse_num_list(input: &str) -> IResult<&str, HashSet<u8>> {
    fold_many1(
        tuple((complete::u8, complete::space0)),
        HashSet::new,
        |mut acc, (x, _)| {
            acc.insert(x);
            acc
        },
    )(input)
}

fn parse_numbers(input: &str) -> IResult<&str, (HashSet<u8>, HashSet<u8>)> {
    separated_pair(
        parse_num_list,
        tuple((tag("|"), complete::space1)),
        parse_num_list,
    )(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, (id, (my_numbers, game_numbers))) = separated_pair(
        preceded(
            tuple((tag("Card"), complete::space1)),
            map(complete::u8, usize::from),
        ),
        tuple((tag(":"), complete::space1)),
        parse_numbers,
    )(input)?;

    Ok((
        input,
        Card {
            id,
            game_numbers,
            my_numbers,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(complete::line_ending, parse_card)(input)
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
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "30".to_string());
    }
}
