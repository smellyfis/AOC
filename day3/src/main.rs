#![warn(clippy::all, clippy::pedantic)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    /*
        let value = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let (comp1, comp2) = line.split_at(line.len() / 2);
                let duplicate = comp2.chars().find(|c| comp1.contains(*c)).unwrap();
                match duplicate {
                    n @'a'..='z' => (n as i32) - ('a' as i32) + 1_i32,
                    n @ 'A'..='Z' => (n as i32) - ('A' as i32) + 27_i32,
                    _ => 0,
                }
            })
            .sum::<i32>();
        println!("Part 1: {value}");
    */
    //part 2
    // fold the lines into groups of three
    let value = reader
        .lines()
        .fold(Vec::new(), |mut acc: Vec<Vec<String>>, line| {
            if acc.is_empty() || acc.last().unwrap().len() == 3 {
                acc.push(Vec::new());
            }
            acc.last_mut().unwrap().push(line.unwrap());
            acc
        })
        .iter()
        .map(|group| {
            let [g1, g2, g3] = group.as_slice() else {
                panic!("not get here")
            };
            match g1
                .chars()
                .fold(Vec::new(), |mut combo: Vec<char>, ch| {
                    if g2.contains(ch) {
                        combo.push(ch);
                    }
                    combo
                })
                .iter()
                .find(|c| g3.contains(**c))
                .unwrap()
            {
                n @ 'a'..='z' => (*n as i32) - ('a' as i32) + 1_i32,
                n @ 'A'..='Z' => (*n as i32) - ('A' as i32) + 27_i32,
                _ => 0,
            }
        })
        .sum::<i32>();
    println!("Part 2: {value}");
    // find common letter in the groups
    //   find common letters in the first 2 then find the common in the third
    // sum the common letters
    Ok(())
}
