#![warn(clippy::all, clippy::pedantic)]

use std::fs;

#[derive(Debug)]
struct HoHoError {}

#[derive(PartialEq)]
enum Choice {
    Rock = 1,
    Paper,
    Scissors,
}

impl std::str::FromStr for Choice {
    type Err = HoHoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(HoHoError {}),
        }
    }
}

impl Choice {
    fn cmp(&self, opponent: &Self) -> i32 {
        if self == opponent {
            return 3;
        }
        if self.beats() == *opponent {
            return 6;
        }
        0
    }
    fn beats(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }
    fn loses(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }
}

struct Game1 {
    pub opponent: Choice,
    pub you: Choice,
}
impl Game1 {
    fn score(self) -> i32 {
        let outcome = self.you.cmp(&self.opponent);
        (self.you as i32) + outcome
    }
}

impl std::str::FromStr for Game1 {
    type Err = HoHoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_split = s.split(' ').collect::<Vec<&str>>();
        let opponent: Choice = str_split[0].parse()?;
        let you: Choice = str_split[1].parse()?;
        Ok(Self { opponent, you })
    }
}

struct Game2 {
    pub opponent: Choice,
    pub you: Choice,
}

impl std::str::FromStr for Game2 {
    type Err = HoHoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_split = s.split(' ').collect::<Vec<&str>>();
        let opponent: Choice = str_split[0].parse()?;
        // game1
        //let you: Choice = str_split[1].parse()?;
        let you = match *str_split.get(1).expect("msg") {
            "X" => opponent.beats(),
            "Y" => str_split[0].parse()?,
            "Z" => opponent.loses(),
            _ => return Err(HoHoError {}),
        };
        Ok(Self { opponent, you })
    }
}

impl Game2 {
    fn outcome(&self) -> i32 {
        self.you.cmp(&self.opponent)
    }

    fn score(self) -> i32 {
        let outcome = self.outcome();
        (self.you as i32) + outcome
    }
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<Game1>().unwrap().score())
        .sum::<i32>()
        .to_string()
}
fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<Game2>().unwrap().score())
        .sum::<i32>()
        .to_string()
}

fn main() -> std::io::Result<()> {
    //read in file
    let file = fs::read_to_string("input")?;

    println!("Part2: {}", part1(&file));
    println!("Part2: {}", part2(&file));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        debug_assert_eq!(part1(INPUT), "15");
    }

    #[test]
    fn part2_works() {
        debug_assert_eq!(part2(INPUT), "12");
    }
}
