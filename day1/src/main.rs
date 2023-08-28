#![warn(clippy::all, clippy::pedantic)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut elves = reader.lines().fold(vec![0_u64], |mut acc, line| {
        let line = line.unwrap();
        //empty lines mean new elf
        if line.is_empty() {
            acc.push(0_u64);
        } else {
            // the first time through is an edge case preventing an else here
            let last = acc.last_mut().unwrap();
            *last += line.parse::<u64>().unwrap();
        }
        acc
    });

    //order the elves since we don't care about position anymore
    elves.sort_by(|a, b| b.cmp(a));
    let max = *elves.first().expect("faliure");
    let counts = elves.iter().take(3).sum::<u64>();

    //part 1 is get the max
    println!("Part 1: {max}");

    //Part 2 is get the sum of the largest 3
    println!("Part 2: {counts}");

    Ok(())
}
