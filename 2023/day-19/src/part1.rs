#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, iter::successors};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

type ParserReturn<'a> = (HashMap<OpLabel<'a>, Vec<Op<'a>>>, Vec<Part>);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Part {
    fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum RatingType {
    ExtremelyCool,
    Musical,
    AeroDynamic,
    Shiny,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum OpLabel<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Op<'a> {
    Goto(OpLabel<'a>),
    Greater(RatingType, u32, OpLabel<'a>),
    Less(RatingType, u32, OpLabel<'a>),
}

/// day 19 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part1(input: &str) -> String {
    let (_, (workflows, parts)) = parse_input(input).expect("valid aoc input");
    parts
        .iter()
        .filter_map(|part| {
            (successors(Some(OpLabel::Workflow("in")), |label| {
                if *label == OpLabel::Accept || *label == OpLabel::Reject {
                    return None;
                }
                let workflow = dbg!(workflows.get(label)).unwrap();
                workflow
                    .iter()
                    .find_map(|op| match op {
                        Op::Goto(label) => Some(label),
                        Op::Greater(rating_type, value, label) => (match rating_type {
                            RatingType::ExtremelyCool => part.x,
                            RatingType::Musical => part.m,
                            RatingType::AeroDynamic => part.a,
                            RatingType::Shiny => part.s,
                        } > *value)
                            .then_some(label),
                        Op::Less(rating_type, value, label) => (match rating_type {
                            RatingType::ExtremelyCool => part.x,
                            RatingType::Musical => part.m,
                            RatingType::AeroDynamic => part.a,
                            RatingType::Shiny => part.s,
                        } < *value)
                            .then_some(label),
                    })
                    .copied()
            })
            .last()
            .unwrap()
                == OpLabel::Accept)
                .then_some(part.rating())
        })
        .sum::<u32>()
        .to_string()
}

fn parse_op_label(input: &str) -> IResult<&str, OpLabel> {
    alt((
        tag("A").map(|_| OpLabel::Accept),
        tag("R").map(|_| OpLabel::Reject),
        complete::alpha1.map(OpLabel::Workflow),
    ))(input)
}

fn parse_rating_type(input: &str) -> IResult<&str, RatingType> {
    alt((
        tag("x").map(|_| RatingType::ExtremelyCool),
        tag("m").map(|_| RatingType::Musical),
        tag("a").map(|_| RatingType::AeroDynamic),
        tag("s").map(|_| RatingType::Shiny),
    ))(input)
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        separated_pair(
            separated_pair(parse_rating_type, tag("<"), complete::u32),
            tag(":"),
            parse_op_label,
        )
        .map(|((typ, value), to)| Op::Less(typ, value, to)),
        separated_pair(
            separated_pair(parse_rating_type, tag(">"), complete::u32),
            tag(":"),
            parse_op_label,
        )
        .map(|((typ, value), to)| Op::Greater(typ, value, to)),
        parse_op_label.map(Op::Goto),
    ))(input)
}

fn parse_workflow(input: &str) -> IResult<&str, (OpLabel, Vec<Op>)> {
    let (input, label) =
        complete::alpha1(input).map(|(input, label)| (input, OpLabel::Workflow(label)))?;
    let (input, ops) = delimited(tag("{"), separated_list1(tag(","), parse_op), tag("}"))(input)?;
    Ok((input, (label, ops)))
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<OpLabel, Vec<Op>>> {
    fold_many1(
        terminated(parse_workflow, complete::line_ending),
        HashMap::new,
        |mut acc, (label, ops)| {
            acc.insert(label, ops);
            acc
        },
    )(input)
}

fn parse_rating(input: &str) -> IResult<&str, Part> {
    delimited(
        tag("{"),
        tuple((
            delimited(tag("x="), complete::u32, tag(",")),
            delimited(tag("m="), complete::u32, tag(",")),
            delimited(tag("a="), complete::u32, tag(",")),
            preceded(tag("s="), complete::u32),
        ))
        .map(|(x, m, a, s)| Part { x, m, a, s }),
        tag("}"),
    )(input)
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(complete::line_ending, parse_rating)(input)
}

fn parse_input(input: &str) -> IResult<&str, ParserReturn> {
    let (input, workflows) = parse_workflows(input)?;
    let (input, _) = complete::line_ending(input)?;
    let (input, parts) = parse_parts(input)?;
    Ok((input, (workflows, parts)))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "19114".to_string());
    }
}
