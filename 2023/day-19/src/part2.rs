#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, ops::RangeBounds};

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
    pub x: u64,
    pub m: u64,
    pub a: u64,
    pub s: u64,
}

#[allow(dead_code)]
impl Part {
    fn rating(&self) -> u64 {
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

fn range_to_pair<R: RangeBounds<u64>>(range: R) -> (u64, u64) {
    let start = match range.start_bound() {
        std::ops::Bound::Included(x) => *x,
        _ => panic!("not a thing"),
    };
    let end = match range.end_bound() {
        std::ops::Bound::Included(x) => *x + 1,
        std::ops::Bound::Excluded(x) => *x,
        std::ops::Bound::Unbounded => panic!("not a thing"),
    };
    (start, end)
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Op<'a> {
    Goto(OpLabel<'a>),
    Greater(RatingType, u64, OpLabel<'a>),
    Less(RatingType, u64, OpLabel<'a>),
}

fn check<R: RangeBounds<u64>>(
    workflows: &HashMap<OpLabel, Vec<Op>>,
    current_flow: OpLabel,
    ranges: (R, R, R, R),
) -> u64 {
    let mut x = range_to_pair(ranges.0);
    let mut m = range_to_pair(ranges.1);
    let mut a = range_to_pair(ranges.2);
    let mut s = range_to_pair(ranges.3);
    match current_flow {
        OpLabel::Accept => (x.1 - x.0) * (m.1 - m.0) * (a.1 - a.0) * (s.1 - s.0),
        OpLabel::Reject => 0,
        flow @ OpLabel::Workflow(_) => {
            let paths = workflows.get(&flow).unwrap();
            let mut sum = 0;
            for path in paths {
                sum += match path {
                    Op::Goto(label) => {
                        check(workflows, *label, (x.0..x.1, m.0..m.1, a.0..a.1, s.0..s.1))
                    }
                    Op::Greater(rating_type, value, next) => match rating_type {
                        RatingType::ExtremelyCool if x.0 > *value => {
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a.0..a.1, s.0..s.1))
                        }
                        RatingType::ExtremelyCool if (x.0..x.1).contains(value) => {
                            let x_new = (value + 1)..(x.1);
                            x.1 = *value + 1;
                            check(workflows, *next, (x_new, m.0..m.1, a.0..a.1, s.0..s.1))
                        }
                        RatingType::Musical if m.0 > *value => {
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a.0..a.1, s.0..s.1))
                        }
                        RatingType::Musical if (m.0..m.1).contains(value) || m.0 > *value => {
                            let m_new = (value + 1)..m.1;
                            m.1 = *value + 1;
                            check(workflows, *next, (x.0..x.1, m_new, a.0..a.1, s.0..s.1))
                        }
                        RatingType::AeroDynamic if a.0 > *value => {
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a.0..a.1, s.0..s.1))
                        }
                        RatingType::AeroDynamic if (a.0..a.1).contains(value) || a.0 > *value => {
                            let a_new = (value + 1)..a.1;
                            a.1 = *value + 1;
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a_new, s.0..s.1))
                        }
                        RatingType::Shiny if s.0 > *value => {
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a.0..a.1, s.0..s.1))
                        }
                        RatingType::Shiny if (s.0..s.1).contains(value) || s.0 > *value => {
                            let s_new = (value + 1)..s.1;
                            s.1 = *value + 1;
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a.0..a.1, s_new))
                        }
                        _ => 0,
                    },
                    Op::Less(rating_type, value, next) => match rating_type {
                        RatingType::ExtremelyCool if x.1 < *value => {
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a.0..a.1, s.0..s.1))
                        }
                        RatingType::ExtremelyCool if (x.0..x.1).contains(value) => {
                            let x_new = x.0..(*value);
                            x.0 = *value;
                            check(workflows, *next, (x_new, m.0..m.1, a.0..a.1, s.0..s.1))
                        }
                        RatingType::Musical if m.1 < *value => {
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a.0..a.1, s.0..s.1))
                        }
                        RatingType::Musical if (m.0..m.1).contains(value) => {
                            let m_new = m.0..(*value);
                            m.0 = *value;
                            check(workflows, *next, (x.0..x.1, m_new, a.0..a.1, s.0..s.1))
                        }
                        RatingType::AeroDynamic if a.1 < *value => {
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a.0..a.1, s.0..s.1))
                        }
                        RatingType::AeroDynamic if (a.0..a.1).contains(value) => {
                            let a_new = a.0..(*value);
                            a.0 = *value;
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a_new, s.0..s.1))
                        }
                        RatingType::Shiny if s.1 < *value => {
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a.0..a.1, s.0..s.1))
                        }
                        RatingType::Shiny if (s.0..s.1).contains(value) => {
                            let s_new = s.0..(*value);
                            s.0 = *value;
                            check(workflows, *next, (x.0..x.1, m.0..m.1, a.0..a.1, s_new))
                        }
                        _ => 0,
                    },
                };
            }
            sum
        }
    }
}

/// day 19 part 2 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part2(input: &str) -> String {
    let (_, (workflows, _)) = parse_input(input).expect("valid aoc input");
    let ranges = ((1..=4000), (1..=4000), (1..=4000), (1..=4000));
    let start_node = OpLabel::Workflow("in");
    check(&workflows, start_node, ranges).to_string()
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
            separated_pair(parse_rating_type, tag("<"), complete::u64),
            tag(":"),
            parse_op_label,
        )
        .map(|((typ, value), to)| Op::Less(typ, value, to)),
        separated_pair(
            separated_pair(parse_rating_type, tag(">"), complete::u64),
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
            delimited(tag("x="), complete::u64, tag(",")),
            delimited(tag("m="), complete::u64, tag(",")),
            delimited(tag("a="), complete::u64, tag(",")),
            preceded(tag("s="), complete::u64),
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
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "167409079868000".to_string());
    }
}
