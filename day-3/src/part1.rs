#![warn(clippy::all, clippy::pedantic)]
use std::collections::BTreeMap;

#[derive(Debug)]
struct SerialNumber {
    pub no: u64,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl SerialNumber {
    fn generate_adjacent(&self) -> Vec<(usize, usize)> {
        let start_row = if self.start.0 == 0 {
            0
        } else {
            self.start.0 - 1
        };
        let start_line = if self.start.1 == 0 {
            0
        } else {
            self.start.1 - 1
        };
        (start_row..=(self.end.0 + 1))
            .flat_map(|x| (start_line..=(self.end.1 + 1)).map(move |y| (x, y)))
            .collect()
    }
}

#[must_use]
pub fn part1(input: &str) -> String {
    let (serials, symbols) = parse_input(input);
    serials
        .iter()
        .filter(|x| {
            x.generate_adjacent()
                .iter()
                .any(|t| symbols.get(t).is_some())
        })
        .map(|x| x.no)
        .sum::<u64>()
        .to_string()
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
            numbers.push( SerialNumber {
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

    const INPUT2: &str="12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "4361".to_string());
    }

    #[test]
    fn part1_works_more() {
        let result = part1(INPUT2);
        assert_eq!(result, "925".to_string());
    }
}
