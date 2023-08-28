#![warn(clippy::all, clippy::pedantic)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let value = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            // split and parse
            let (a, b) = match line.split(',').take(2).collect::<Vec<&str>>()[..] {
                [a, b] => (
                    a.split('-')
                        .take(2)
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                    b.split('-')
                        .take(2)
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                ),
                _ => panic!("no good"),
            };
            (
                // part 1 wholly overlapping
                i32::from((a[0] <= b[0] && a[1] >= b[1]) || (a[0] >= b[0] && a[1] <= b[1])),
                // part 2 any overlapping
                i32::from(
                    (a[0] >= b[0] && a[0] <= b[1])
                        || (a[1] >= b[0] && a[1] <= b[1])
                        || (b[0] >= a[0] && b[0] <= a[1])
                        || (b[1] >= a[0] && b[1] <= a[1]),
                ),
            )
        })
        // using folding instead of sum() so that we can do both parts in one call
        .fold((0, 0), |mut acc: (i32, i32), x| {
            acc.0 += x.0;
            acc.1 += x.1;
            acc
        });
    println!("Part 1: {}", value.0);
    println!("Part 2: {}", value.1);

    Ok(())
}
