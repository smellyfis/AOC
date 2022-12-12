use std::fs::File;
use std::io::{prelude::*, BufReader};

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

struct Game {
    pub opponent: Choice,
    pub you: Choice,
}

impl std::str::FromStr for Game {
    type Err = HoHoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_split = s.split(' ').collect::<Vec<&str>>();
        let opponent: Choice = str_split[0].parse()?;
        // game1
        //let you: Choice = str_split[1].parse()?;
        let you = match str_split[1] {
            "X" => opponent.beats(),
            "Y" => str_split[0].parse()?,
            "Z" => opponent.loses(),
            _ => return Err(HoHoError {}),
        };
        Ok(Game { opponent, you })
    }
}

impl Game {
    fn outcome(&self) -> i32 {
        self.you.cmp(&self.opponent)
    }

    fn score(self) -> i32 {
        let outcome = self.outcome();
        (self.you as i32) + outcome
    }
}

fn main() -> std::io::Result<()> {
    //read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let score: i32 = reader
        .lines()
        .map(|line| line.unwrap().parse::<Game>().unwrap().score())
        .sum();
    println!("Puzzle 1: {score}");
    Ok(())
}
