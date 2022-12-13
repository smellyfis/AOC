use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    reader.lines().enumerate().for_each(|(line_no, line)| {
        let line = line.unwrap();
        'checker: for i in 4..line.len() {
            let window = line[(i - 4)..i].to_string();
            let mut holder: HashMap<char, bool> = HashMap::new();
            for c in window.chars() {
                if holder.contains_key(&c) {
                    continue 'checker;
                }
                holder.insert(c, true);
            }
            println!("Part 1: line: {} marker at: {}", line_no, i);
            break;
        }
        'checker: for i in 14..line.len() {
            let window = line[(i - 14)..i].to_string();
            let mut holder: HashMap<char, bool> = HashMap::new();
            for c in window.chars() {
                if holder.contains_key(&c) {
                    continue 'checker;
                }
                holder.insert(c, true);
            }
            println!("Part 2: line: {} marker at: {}", line_no, i);
            break;
        }
    });
    Ok(())
}
