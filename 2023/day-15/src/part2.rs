#![warn(clippy::all, clippy::pedantic)]

use std::iter::repeat_with;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{pair, preceded},
    IResult, Parser,
};

enum Op {
    Set(u8),
    Remove,
}
struct Lens {
    pub label: String,
    pub power: u8,
}

fn unhash(hash: &str) -> usize {
    hash.chars()
        .fold(0, |acc, x| (acc + (x as usize)) * 17 % 256)
}

/// day 15 part 2 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part2(input: &str) -> String {
    let (_, steps) = parse_input(input).expect("aoc always good");
    let mut boxes = repeat_with(Vec::<Lens>::new).take(256).collect::<Vec<_>>();
    for (label, op) in steps {
        let box_index = unhash(label);
        let lenses = boxes.get_mut(box_index).unwrap(); //u8 should always be there
        if let Some(lens_index) = lenses.iter().position(|lens| lens.label == label) {
            match op {
                Op::Set(power) => lenses.get_mut(lens_index).unwrap().power = power,
                Op::Remove => {
                    lenses.remove(lens_index);
                }
            }
        } else {
            match op {
                Op::Set(power) => lenses.push(Lens {
                    label: label.to_string(),
                    power,
                }),
                Op::Remove => (),
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_num, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_index, lens)| (box_num + 1) * (lens_index + 1) * (lens.power as usize))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, Op)>> {
    separated_list1(
        tag(","),
        pair(
            complete::alpha1,
            alt((
                tag("-").map(|_| Op::Remove),
                preceded(tag("="), complete::u8).map(Op::Set),
            )),
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "145".to_string());
    }
}
