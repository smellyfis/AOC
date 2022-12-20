use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str;

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut part1: Vec<i32> = Vec::new();
    let mut part2 = [[b'.'; 40]; 6];
    let mut x_reg = 1;
    let mut pc = 0;

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let op = match line.split(' ').collect::<Vec<_>>()[..] {
            ["addx", x] => Some(x.parse::<i32>().unwrap()),
            ["noop"] => None,
            _ => panic!("invalid command: {}", line),
        };
        let steps = if op.is_some() { 2 } else { 1 };
        for i in 0..steps {
            let row = pc / 40;
            let col = pc % 40;
            if col - 1 == x_reg || col == x_reg || col + 1 == x_reg {
                part2[row as usize][col as usize] = b'#';
            }
            pc += 1;
            if pc < 221 && (pc - 20) % 40 == 0 {
                part1.push(pc * x_reg);
            }
            if i == steps - 1 {
                if let Some(x) = op {
                    x_reg += x;
                }
            }
        }
    });

    println!("Part 1: {}", part1.iter().sum::<i32>());
    for row in part2 {
        println!("Part 2: {}", str::from_utf8(&row).unwrap());
    }
    Ok(())
}
