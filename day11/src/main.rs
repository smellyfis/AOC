#![warn(clippy::all, clippy::pedantic)]
use derive_getters::Getters;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use log::{debug, trace};
use std::{
    collections::VecDeque, error, fmt::Display, fs::File, io::Read, num::ParseIntError,
    str::FromStr,
};

pub type BoxError = std::boxed::Box<
    dyn std::error::Error // must implement Error to satisfy ?
        + std::marker::Send // needed for threads
        + std::marker::Sync, // needed for threads
>;

#[derive(Debug)]
enum MyParseError {
    ParseIntError(ParseIntError),
}
impl error::Error for MyParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            Self::ParseIntError(ref e) => Some(e),
        }
    }
}
impl From<ParseIntError> for MyParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
impl Display for MyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(err) => {
                write!(f, "There was a problem parsing an integer: {err}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tok {
    Num(u64),
    Old,
}
impl Tok {
    pub fn get_value_or(&self, current: u64) -> u64 {
        match self {
            Self::Num(i) => *i,
            Self::Old => current,
        }
    }
    pub fn parse_tok(input: &str) -> IResult<&str, Self> {
        alt((
            tag("old").map(|_| Self::Old),
            nom::character::complete::u64.map(Self::Num),
        ))(input)
    }
}
impl Default for Tok {
    fn default() -> Self {
        Self::Old
    }
}
impl FromStr for Tok {
    type Err = MyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = match s {
            "old" => Self::Old,
            x => Self::Num(x.parse()?),
        };
        Ok(val)
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum Op {
    Add(Tok, Tok),
    Mul(Tok, Tok),
    #[default]
    Noop,
}
impl Op {
    pub fn do_op(&self, current: u64) -> u64 {
        match self {
            Self::Add(a, b) => a.get_value_or(current) + b.get_value_or(current),
            Self::Mul(a, b) => a.get_value_or(current) * b.get_value_or(current),
            Self::Noop => panic!("No operation implemented"),
        }
    }
    pub fn parse_op(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Operation: new = ")(input)?;
        let (input, value_1) = Tok::parse_tok(input)?;
        let (input, operator) =
            delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
        let (input, value_2) = Tok::parse_tok(input)?;
        let op = match operator {
            "+" => Self::Add(value_1, value_2),
            "*" => Self::Mul(value_1, value_2),
            _ => Self::Noop,
        };
        Ok((input, op))
    }
}

impl FromStr for Op {
    type Err = MyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = match s.split_whitespace().collect::<Vec<_>>()[..] {
            ["new", "=", a, "+", b] => Self::Add(a.parse()?, b.parse()?),
            ["new", "=", a, "*", b] => Self::Mul(a.parse()?, b.parse()?),
            _ => Self::Noop,
        };
        Ok(val)
    }
}
#[derive(Debug, Default, Getters, Clone, Copy)]
struct Test {
    divisor: u64,
    true_to: usize,
    false_to: usize,
}
impl Test {
    pub fn get_to(&self, item: u64) -> usize {
        match item % self.divisor {
            0 => self.true_to,
            _ => self.false_to,
        }
    }
    pub fn parse_test(input: &str) -> IResult<&str, Self> {
        trace!("parse test 1");
        let (input, divisor) =
            preceded(tag("Test: divisible by "), nom::character::complete::u64)(input)?;
        trace!("parse test 2");
        let (input, _) = multispace1(input)?;
        trace!("parse test 3");
        let (input, true_to) = match preceded(
            tag("If true: throw to monkey "),
            nom::character::complete::u64.map(|x| usize::try_from(x).unwrap()),
        )(input){
            Err(e) => {println!("{e:?}"); Err(e)},
            x => x,
        }?;
        trace!("parse test 4");
        let (input, _) = multispace1(input)?;
        trace!("parse test 5");
        let (input, false_to) = preceded(
            tag("If false: throw to monkey "),
            nom::character::complete::u64.map(|x| usize::try_from(x).unwrap()),
        )(input)?;
        trace!("parse test 6");
        Ok((
            input,
            Self {
                divisor,
                true_to,
                false_to,
            },
        ))
    }
}

#[derive(Debug, Default, Getters, Clone)]
struct Ape {
    items: VecDeque<u64>,
    operation: Op,
    test: Test,
    inspected: u64,
}
impl Ape {
    pub fn catch(&mut self, item: u64) {
        self.items.push_back(item);
    }

    pub fn inspect(&mut self, relief: bool) -> Option<u64> {
        let relief: u64 = if relief {3} else {1};
        let item = self.items.pop_front()?;
        self.inspected += 1;
        Some(self.operation.do_op(item) / relief)
    }
    pub fn throw(&self, item: u64) -> usize {
        self.test.get_to(item)
    }
    //impl nom::ParseTo<Ape> for String{
    //    todo!()

    //    }

    pub fn parse_monkey(input: &str) -> IResult<&str, Self> {
        trace!("parse monkey 1");
        let (input, _id) =
            delimited(tag("Monkey "), nom::character::complete::u64, tag(":"))(input)?;
        trace!("parse monkey 2");
        let (input, _) = multispace1(input)?;
        trace!("parse monkey 3");
        let (input, items) = match preceded(
            tag("Starting items: "),
            separated_list1(tag(", "), nom::character::complete::u64),
        )(input)
        {
            Err(x) => {
                println!("{x:?}");
                Err(x)
            }
            x => x,
        }?;
        trace!("parse monkey 4");
        let items = VecDeque::from(items);
        let (input, _) = multispace1(input)?;
        trace!("parse monkey 5");
        let (input, operation) = Op::parse_op(input)?;
        trace!("parse monkey 6");
        let (input, _) = multispace1(input)?;
        trace!("parse monkey 7");
        let (input, test) = match Test::parse_test(input) {
            Err(e) => {
                println!("{e:?}");
                Err(e)
            }
            x => x,
        }?;
        trace!("parse monkey 8");
        Ok((
            input,
            Self {
                items,
                operation,
                test,
                inspected: 0,
            },
        ))
    }
}
impl FromStr for Ape {
    type Err = Error<String>;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
        //Self::parse_monkey(s)?.finalize()
    }
}

#[derive(Debug, Default, Getters, Clone)]
struct Monkeys {
    monkeys: Vec<Ape>,
}

impl FromStr for Monkeys {
    type Err = Error<String>;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl AsRef<Vec<Ape>> for Monkeys {
    fn as_ref(&self) -> &Vec<Ape> {
        self.monkeys.as_ref()
    }
}
struct MonkeyReader {
    buffer: String,
}
impl MonkeyReader {
    pub fn new(file: &str) -> Result<Self, std::io::Error> {
        let mut f = File::open(file)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        Ok(Self { buffer })
    }
}
impl IntoIterator for MonkeyReader {
    type Item = Ape;
    type IntoIter = MonkeyReaderIter;

    fn into_iter(self) -> Self::IntoIter {
        MonkeyReaderIter {
            content: self.buffer,
        }
    }
}

struct MonkeyReaderIter {
    content: String,
}

impl Iterator for MonkeyReaderIter {
    type Item = Ape;
    fn next(&mut self) -> Option<Self::Item> {
        match self.content.as_str() {
            "" => {
                trace!("done");
                None
            }
            x => {
                trace!("Starting ape parse");
                let (input, _) = nom::combinator::opt(tag::<_, _, Error<&str>>("\n\n"))(x).ok()?;
                trace!("pass the optional");
                let (input, monkey) = Ape::parse_monkey(input).ok()?;
                trace!("Saving off ape");
                self.content = String::from(input);
                debug!("{monkey:?}");
                Some(monkey)
            }
        }
    }
}

fn main() {
    let mut monkeys = MonkeyReader::new("./test.txt")
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    println!("{monkeys:?}");
    //parse

    let mut monkeys_game2 = monkeys.clone();
    //game 1
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some( item) = monkeys[i].inspect(true){
                let throw_to = monkeys[i].throw(item);
                monkeys[throw_to].catch(item);
            }
        }
    }
    println!("{monkeys:?}");

    let val = monkeys
        .iter()
        .map(Ape::inspected)
        .sorted()
        .rev()
        .take(2)
        .product::<u64>();
    println!("Part 1: {val}");

    let magic = monkeys_game2.iter().map(|x| x.test().divisor()).product::<u64>();

    for _ in 0..10_000 {
        for i in 0..monkeys_game2.len() {
            while let Some(item) = monkeys_game2[i].inspect(false) {
                let item = item % magic;
                let throw_to = monkeys_game2[i].throw(item);
                monkeys_game2[throw_to].catch(item);
            }
        }
    }
    let val2 = monkeys_game2
        .iter()
        .map(Ape::inspected)
        .sorted()
        .rev()
        .take(2)
        .product::<u64>();
    println!("part 2: {val2}");
}
