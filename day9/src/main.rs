use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut tail_pos: Vec<(i32, i32)> = vec![(0, 0)];
    let mut head_pos: Vec<(i32, i32)> = vec![(0, 0)];

    //read in input
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let (direction, steps) = match line.split(' ').collect::<Vec<&str>>()[..] {
            [dir, step] => (dir, step.parse::<usize>().unwrap()),
            _ => panic!("failed parseing line {}", line),
        };
        for _ in 0..steps {
            let (mut cur_head_x, mut cur_head_y) = head_pos.last().unwrap();
            let (mut cur_tail_x, mut cur_tail_y) = tail_pos.last().unwrap();
            match direction {
                "L" => cur_head_x -= 1,
                "R" => cur_head_x += 1,
                "U" => cur_head_y += 1,
                "D" => cur_head_y -= 1,
                x => panic!("invalid movement {}", x),
            };
            let x_offset = cur_head_x - cur_tail_x;
            let y_offset = cur_head_y - cur_tail_y;
            if x_offset > 1 {
                if y_offset != 0 {
                    cur_tail_y = cur_head_y;
                }
                cur_tail_x += 1
            } else if x_offset < -1 {
                if y_offset != 0 {
                    cur_tail_y = cur_head_y;
                }
                cur_tail_x -= 1
            } else if y_offset > 1 {
                if x_offset != 0 {
                    cur_tail_x = cur_head_x;
                }
                cur_tail_y += 1
            } else if y_offset < -1 {
                if x_offset != 0 {
                    cur_tail_x = cur_head_x;
                }
                cur_tail_y -= 1
            }
            let new_tail = (cur_tail_x, cur_tail_y);
            if *tail_pos.last().unwrap() != new_tail {
                tail_pos.push(new_tail);
            }
            head_pos.push((cur_head_x, cur_head_y));
        }
    });
    let part1 = tail_pos.iter().unique().count();
    println!("Part 1: {}", part1);
    Ok(())
}
