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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    On,
    Off,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Signal {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ModuleType<'a> {
    Broadcast,
    FlipFlop(Status),
    Conjunction(BTreeMap<&'a str, Signal>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Module<'a> {
    pub label: &'a str,
    pub mod_type: ModuleType<'a>,
    pub connections: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn handle_pulse(&mut self, from: &'a str, pulse: Signal) -> Vec<(&'a str, &'a str, Signal)> {
        /*println!(
            "{from} -{}-> {}",
            if pulse == Signal::Low { "low" } else { "high" },
            self.label
        );*/
        let signal_to_send = match (&mut self.mod_type, pulse) {
            (ModuleType::Broadcast, _) => Some(pulse),
            (ModuleType::FlipFlop(_), Signal::High) => None,
            (ModuleType::FlipFlop(ref mut state), Signal::Low) => {
                *state = if *state == Status::Off {
                    Status::On
                } else {
                    Status::Off
                };
                Some(if *state == Status::On {
                    Signal::High
                } else {
                    Signal::Low
                })
            }
            (ModuleType::Conjunction(memory), pulse) => {
                *memory.get_mut(from).unwrap() = pulse;
                Some(if memory.values().all(|x| *x == Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                })
            }
        };
        //println!("{self:#?}");
        if let Some(signal_to_send) = signal_to_send {
            self.connections
                .iter()
                .map(|x| (*x, self.label, signal_to_send))
                .collect()
        } else {
            vec![]
        }
    }

    fn state_hash(&self) -> String {
        match &self.mod_type {
            ModuleType::Broadcast => (self.label).to_string(),
            ModuleType::FlipFlop(memory) => {
                "%".to_string()
                    + &{
                        if *memory == Status::On {
                            self.label.to_uppercase()
                        } else {
                            self.label.to_lowercase()
                        }
                    }
                    + "%"
            }
            ModuleType::Conjunction(memory) => {
                "&".to_string()
                    + self.label
                    + &memory
                        .iter()
                        .map(|(key, value)| {
                            if *value == Signal::High {
                                key.to_uppercase()
                            } else {
                                key.to_lowercase()
                            }
                        })
                        .collect::<String>()
                    + "&"
            }
        }
    }
}

fn push_button<'a>(
    setup: &mut BTreeMap<&'a str, Module<'a>>,
    cache: &mut BTreeMap<&'a str, Vec<String>>,
) -> (bool, Vec<(&'a str, Signal)>) {
    let mut queue = VecDeque::from(vec![("broadcaster", "button", Signal::Low)]);
    let mut triggered = Vec::new();
    while let Some((current_label, from, signal)) = queue.pop_front() {
        let Some(current) = setup.get_mut(current_label) else {
            // if not found then in a sink
            if current_label == "rx" && signal == Signal::Low {
                return (true, vec![]);
            }
            continue;
        };

        cache
            .entry(current_label)
            .and_modify(|list| list.push(current.state_hash()))
            .or_insert(vec![current.state_hash()]);

        let signal_to_send = current.handle_pulse(from, signal);
        triggered.push((current_label, signal));
        queue.extend(signal_to_send);
    }
    (false, triggered)
}

#[allow(dead_code)]
fn setup_to_key(setup: &BTreeMap<&str, Module>) -> String {
    setup
        .iter()
        .map(|(_, module)| module.state_hash())
        .collect::<String>()
}

/// day 20 part 2 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part2(input: &str) -> String {
    let (_, mut setup) = parse_input(input).expect("aoc input always valid");
    let mut penulttimate_lengths = Vec::new();

    //get last node "rx"'s connections
    //TODO this is ugly cause it assumes the input only has one
    let last_node = setup
        .values()
        .find(|module| module.connections.iter().any(|label| *label == "rx"))
        .unwrap();

    //get last node's connections
    let mut penultimate_modules = setup
        .iter()
        .filter_map(|(label, module)| {
            module
                .connections
                .contains(&last_node.label)
                .then_some(*label)
        })
        .collect::<Vec<_>>();

    let mut cache = BTreeMap::new();
    //loop through pushing the button till we found all the connecting nodes cycles
    for i in 0_u64.. {
        if penultimate_modules.is_empty() {
            break;
        }

        //push the button
        //println!("push the button {i}");
        let (triggered_early, triggered) = push_button(&mut setup, &mut cache);
        if triggered_early {
            return (i + 1).to_string();
        }

        //check if the penultimate has cycled yet
        let mut y = 0;
        while y < penultimate_modules.len() {
            let label = penultimate_modules[y];
            y += 1;
            if triggered.contains(&(label, Signal::Low)) {
                penulttimate_lengths.push(i + 1);
                let index = penultimate_modules
                    .iter()
                    .position(|x| *x == label)
                    .unwrap();
                penultimate_modules.remove(index);
            }
        }
    }
    //get the LCM of all the last nodes connections
    // TODO this is ugly cause it assumes lineailty from the end of the input

    //println!("{cache:#?}");
    lcm(&penulttimate_lengths).to_string()
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn parse_line(input: &str) -> IResult<&str, (&str, Module)> {
    let (input, mod_type) = opt(alt((tag("%"), tag("&"))))(input)?;
    let (input, (label, connections)) = separated_pair(
        complete::alpha1,
        tuple((complete::space0, tag("->"), complete::space0)),
        separated_list1(tuple((tag(","), complete::space0)), complete::alpha1),
    )(input)?;
    let mod_type = match mod_type {
        Some("%") => ModuleType::FlipFlop(Status::Off),
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
                if let ModuleType::Conjunction(memory) = &mut module.mod_type {
                    memory.insert(to_key, Signal::Low);
                }
            });
        });
    Ok((input, lines))
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "broadcaster -> a, c, inv
%c -> d
%a -> b
&inv -> con
%b -> con
%d -> con
&con -> rx",
        "1"
    )]
    fn part2_works(#[case] input: &str, #[case] expected: &str) {
        let result = part2(input);
        assert_eq!(result, expected);
    }
}
