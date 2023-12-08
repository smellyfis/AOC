#![warn(clippy::all, clippy::pedantic)]
use itertools::Itertools;
use nom::{character::complete, multi::separated_list1, sequence::separated_pair, IResult};
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::BTreeMap,
    str::FromStr,
};
use std::fmt;

#[derive(Debug)]
struct Day1Part2Error;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
enum Card {
    Joker = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}
impl FromStr for Card {
    type Err = Day1Part2Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::Ten),
            "J" => Ok(Self::Joker),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(Day1Part2Error),
        }
    }
}

impl From<&Card> for &u32 {
    fn from(value: &Card) -> Self {
        match value {
            Card::Two => &2,
            Card::Three => &3,
            Card::Four => &4,
            Card::Five => &5,
            Card::Six => &6,
            Card::Seven => &7,
            Card::Eight => &8,
            Card::Nine => &9,
            Card::Ten => &10,
            Card::Joker => &1,
            Card::Queen => &12,
            Card::King => &13,
            Card::Ace => &14,
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let c = match self  {
            Card::Joker => 'J',
            Card::Two =>  '2',
            Card::Three =>  '3',
            Card::Four =>  '4',
            Card::Five =>  '5',
            Card::Six =>  '6',
            Card::Seven =>  '7',
            Card::Eight =>  '8',
            Card::Nine =>  '9',
            Card::Ten =>  'T',
            Card::Queen =>  'Q',
            Card::King =>  'K',
            Card::Ace =>  'A',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let mut map = value.cards.iter().fold(BTreeMap::new(), |mut acc, card| {
            if let Some(c) = acc.get_mut(card) {
                *c += 1;
            } else {
                acc.insert(card, 1);
            }
            acc
        });
        let jokers = map.remove(&Card::Joker).unwrap_or(0);
        match map
            .iter()
            .sorted_by(|a, b| b.1.cmp(a.1))
            .collect::<Vec<_>>()[..]
        {
            [(_, x), ..] if jokers + x == 5 => Self::FiveOfAKind,
            [] if jokers  == 5 => Self::FiveOfAKind,
            [(_, x), ..] if jokers + x == 4 => Self::FourOfAKind,
            [(_, 3), (_, 2)] => Self::FullHouse,
            [(_, 2), (_, 2)] if jokers == 1 => Self::FullHouse,
            [(_, x), ..] if jokers + x == 3 => Self::ThreeOfAKind,
            [(_, 2), (_, 2), ..] => Self::TwoPair,
            [(_, x), ..] if jokers + x == 2 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Hand {
    pub cards: [Card; 5],
    pub bet: u32,
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = HandType::from(self);
        let b = HandType::from(other);
        let c = a.cmp(&b);
        match c {
            Ordering::Equal => self
                .cards
                .iter()
                .interleave(other.cards.iter())
                .tuples::<(_, _)>()
                .find_map(|(a, b)| match a.cmp(b) {
                    Ordering::Equal => None,
                    x => Some(x),
                })
                .unwrap_or(Ordering::Equal),
            x => x,
        }
    }
}

/// part2 of day 7 of AOC 2023
///
/// # Arguments
/// - input the puszzle input
///
/// # Panics
/// panics whenever the input isn't parsable
#[must_use]
pub fn part2(input: &str) -> String {
    let (_, mut hands) = parse_input(input).expect("always valid input");
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bet as usize)
        .sum::<usize>()
        .to_string()
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bet)) =
        separated_pair(complete::alphanumeric1, complete::space1, complete::u32)(input)?;
    let cards = cards
        .chars()
        .filter_map(|c| c.to_string().parse().ok())
        .collect::<Vec<_>>()
        .try_into()
        .expect("should work");
    Ok((input, Hand { cards, bet }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(complete::line_ending, parse_hand)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn qtest() {
        let a_str = "JKKK2 9";
        let b_str = "QQQQ2 8";
        let (_, a_hand) = parse_hand(a_str).expect("shoould parse a");
        let (_, b_hand) = parse_hand(b_str).expect("should parse b");
        let c = a_hand.cmp(&b_hand);
        assert_eq!(c, Ordering::Less);
    }

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "5905".to_string());
    }
}
