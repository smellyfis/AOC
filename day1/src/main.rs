use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()>{
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut elves = reader.lines()
        .fold(Vec::new(), |mut acc, line| {
            let line = line.unwrap();
            //empty lines mean new elf
            if line.is_empty() || acc.is_empty() {
                acc.push(0_u64);
            }

            // the first time through is an edge case preventing an else here
            if ! line.is_empty() {
                let last = acc.last_mut().unwrap();
                *last += line.parse::<u64>().unwrap();
            }
            acc
        });

    //order the elves since we don't care about position anymore
    elves.sort();
    let max = elves.len() - 1;

    //part 1 is get the max
    println!("Part 1: {}", elves[max]);

    //Part 2 is get the sum of the largest 3
    let counts: u64 = elves[(max-2)..].iter().sum();
    println!("Part 2: {counts}");

    Ok(())
}
