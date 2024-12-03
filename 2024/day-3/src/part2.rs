#![warn(clippy::all, clippy::pedantic)]

use error_stack::Result;
use regex::Regex;
use thiserror::Error;

// day-3
#[derive(Debug, Error)]
pub enum Day3Part2Error{
    #[error("Problem parsing Day 3")]
    ParseError,
}

pub fn part2 (input: &str) -> Result<String, Day3Part2Error> {
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dos = do_re.find_iter(input).map(|x| (x.start(),x.end())).collect::<Vec<_>>();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let donts = dont_re.find_iter(input).map(|x| (x.start(),x.end())).collect::<Vec<_>>();

    let mut dos_index = 0;
    let mut donts_index = 0;
    let mut white_list = true;
    let mut blackout_ranges = Vec::new();
    let mut blacklist_start = 0;
    while dos_index < dos.len() && donts_index < donts.len(){
        if white_list {
            if dos[dos_index].1 < donts[donts_index].0 {
                //currently whitelisted so dos are no-ops
                dos_index +=1;
            } else {
                blacklist_start = donts[donts_index].0;
                white_list = false;
            }
        } else {
            if donts[donts_index].1 < dos[dos_index].0 {
                //in a black list so donts are no-ops
                donts_index +=1;
            } else {
                blackout_ranges.push(blacklist_start..dos[dos_index].1);
                blacklist_start = 0;
                white_list = true;
            }
        }
    }
    if donts_index < donts.len()  {
        blackout_ranges.push(donts[donts_index].0..input.len());

    } else if dos_index < dos.len() {
        if blacklist_start  != 0 {
            blackout_ranges.push(blacklist_start..dos[dos_index].1);
        }
    }
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for mult_match in re.find_iter(input){
        if blackout_ranges.iter().any(|x| x.contains(&mult_match.start())) {
            continue;
        }
        let values = re.captures(mult_match.as_str()).unwrap();
        sum += &values[1].parse::<i64>().unwrap() * &values[2].parse::<i64>().unwrap();
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test_log::test]
    #[test_log(default_log_filter = "trace")]
    fn part2_works() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, "48".to_string());
    }
}

