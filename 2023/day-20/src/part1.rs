#![warn(clippy::all, clippy::pedantic)]

use std::collections::{BTreeMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::opt,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ModuleType<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(BTreeMap<&'a str, bool>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Module<'a> {
    pub label: &'a str,
    pub mod_type: ModuleType<'a>,
    pub connections: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn handle_pulse(&mut self, from: &'a str, is_high_pulse: bool) -> Option<bool> {
        //let mut m = self.clone();
        match (&mut self.mod_type, is_high_pulse) {
            (ModuleType::Broadcast, _) => Some(is_high_pulse),
            (ModuleType::FlipFlop(_), true) => None,
            (ModuleType::FlipFlop(ref mut is_on), false) => {
                *is_on = !*is_on;
                Some(*is_on)
            }
            (ModuleType::Conjunction(memory), is_high_pulse) => {
                *memory.get_mut(from).unwrap() = is_high_pulse;
                Some(!memory.values().all(|x| *x))
            }
        }
    }

    fn state_hash(&self) -> String {
        match &self.mod_type {
            ModuleType::Broadcast => (self.label).to_string(),
            ModuleType::FlipFlop(is_on) => {
                if *is_on {
                    self.label.to_uppercase()
                } else {
                    self.label.to_lowercase()
                }
            }
            ModuleType::Conjunction(last_froms_was_high) => {
                "%".to_string()
                    + self.label
                    + &last_froms_was_high
                        .iter()
                        .map(|(key, value)| {
                            if *value {
                                key.to_uppercase()
                            } else {
                                key.to_lowercase()
                            }
                        })
                        .collect::<String>()
                    + "%"
            }
        }
    }
}

fn push_button<'a>(
    setup: &mut BTreeMap<&'a str, Module<'a>>,
) -> (usize, usize) {
    let mut queue = VecDeque::from(vec![("broadcaster", None, false)]);
    let mut low_signals = 1;
    let mut high_signals = 0;
    //println!("push the button");
    loop {
        //println!("{queue:?}");
        if queue.is_empty() {
            break;
        }
        let (current_label, from, signal) = queue.pop_front().unwrap();
        let Some(current) = setup.get_mut(current_label) else {
            // if not found then in a sink
            continue;
        };

        let signal_to_send = current.handle_pulse(from.unwrap_or("button"), signal);
        if let Some(signal_to_send) = signal_to_send {
            current
                .connections
                .iter()
                .map(|x| (*x, Some(current_label), signal_to_send))
                /*.inspect(|(x, _, is_high_signal)| {
                    println!(
                        "{current_label} -{}-> {x}",
                        if *is_high_signal { "high" } else { "low" }
                    )
                })*/
                .for_each(|x| {
                    queue.push_back(x);
                    low_signals += usize::from(!signal_to_send);
                    high_signals += usize::from(signal_to_send);
                });
        }
        //setup.insert(current_label, new_current);
    }
    (low_signals, high_signals)
}

#[allow(dead_code)]
fn setup_to_key(setup: &BTreeMap<&str, Module>) -> String {
    setup.values().map(Module::state_hash).collect::<String>()
}

/// day 20 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part1(input: &str) -> String {
    let (_, mut setup) = parse_input(input).expect("aoc input always valid");
    let mut high_count = 0;
    let mut low_count = 0;
    for _ in 0..1000 {
        let (low, high) = push_button(&mut setup);
        high_count += high;
        low_count += low;
    };

    (high_count * low_count).to_string()
}

fn parse_line(input: &str) -> IResult<&str, (&str, Module)> {
    let (input, mod_type) = opt(alt((tag("%"), tag("&"))))(input)?;
    let (input, (label, connections)) = separated_pair(
        complete::alpha1,
        tuple((complete::space0, tag("->"), complete::space0)),
        separated_list1(tuple((tag(","), complete::space0)), complete::alpha1),
    )(input)?;
    let mod_type = match mod_type {
        Some("%") => ModuleType::FlipFlop(false),
        Some("&") => ModuleType::Conjunction(BTreeMap::new()),
        None => ModuleType::Broadcast,
        Some(x) => unimplemented!("No module type {x}"),
    };
    Ok((
        input,
        (
            label,
            Module {
                label,
                mod_type,
                connections,
            },
        ),
    ))
}

fn parse_input(input: &str) -> IResult<&str, BTreeMap<&str, Module>> {
    let (input, mut lines) = separated_list1(complete::line_ending, parse_line)(input)
        .map(|(input, v)| (input, v.into_iter().collect::<BTreeMap<_, _>>()))?;
    let conjunctions = lines
        .iter()
        .filter_map(|(key, module)| {
            if let ModuleType::Conjunction(_) = module.mod_type {
                Some(*key)
            } else {
                None
            }
        })
        .map(|conjunction| {
            (
                conjunction,
                lines
                    .iter()
                    .filter_map(|(key, module)| {
                        module.connections.contains(&conjunction).then_some(*key)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    lines
        .iter_mut()
        .filter(|(key, _)| conjunctions.contains_key(*key))
        .for_each(|(key, module)| {
            conjunctions.get(key).unwrap().iter().for_each(|to_key| {
                if let ModuleType::Conjunction(tos) = &mut module.mod_type {
                    tos.insert(to_key, false);
                }
            });
        });
    Ok((input, lines))
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[test_log::test(rstest)]
    #[case(
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        "32000000"
    )]
    #[case(
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        "11687500"
    )]
    fn part1_works(#[case] input: &str, #[case] expected: &str) {
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
