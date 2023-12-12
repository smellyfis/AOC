#![warn(clippy::all, clippy::pedantic)]
use std::collections::BTreeMap;

#[derive(Debug)]
struct SerialNumber {
    pub no: u64,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl SerialNumber {
    fn is_adjacent(&self, pos: (usize, usize)) -> bool {
        usize::abs_diff(self.start.1, pos.1) < 2
            && self.start.0 < 2 + pos.0
            && pos.0 < 2 + self.end.0
    }
}

#[must_use]
pub fn part2(input: &str) -> String {
    let (serials, symbols) = parse_input(input);
    symbols
        .iter()
        .filter_map(|(key, value)| if *value == '*' { Some(*key) } else { None })
        .filter_map(|pos| {
            let serials = serials
                .iter()
                .filter_map(|serial| {
                    if serial.is_adjacent(pos) {
                        Some(serial.no)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u64>>();
            if serials.len() == 2 {
                Some(serials[0] * serials[1])
            } else {
                None
            }
        })
        .sum::<u64>()
        .to_string()
    //find all serials next to '*' and map with '*' location
}

fn parse_input(input: &str) -> (Vec<SerialNumber>, BTreeMap<(usize, usize), char>) {
    let mut numbers = Vec::new();
    let mut symbols = BTreeMap::new();
    for (line_no, line) in input.lines().enumerate() {
        let mut prev_char = None;
        let mut cur_no = 0_u64;
        let mut cur_no_row_start = 0_usize;
        for (row_no, c) in line.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                if prev_char.is_some() {
                    cur_no = cur_no * 10 + u64::from(d);
                } else {
                    cur_no = u64::from(d);
                    cur_no_row_start = row_no;
                }
                prev_char = Some(c);
            } else {
                if prev_char.is_some() {
                    //handle saving number off
                    numbers.push(SerialNumber {
                        no: cur_no,
                        start: (cur_no_row_start, line_no),
                        end: (row_no - 1, line_no),
                    });
                }
                prev_char = None;
                if c == '.' {
                    //move along space
                    continue;
                }
                //store symbol
                let _ = symbols.insert((row_no, line_no), c);
            }
        }
        //need to account for new line numbers
        if prev_char.is_some() {
            numbers.push(SerialNumber {
                no: cur_no,
                start: (cur_no_row_start, line_no),
                end: (line.len() - 1, line_no),
            });
        }
    }
    (numbers, symbols)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "467835".to_string());
    }
}
