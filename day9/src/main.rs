use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut snake = vec![(0_i32, 0_i32); 10];
    let mut visited: Vec<HashSet<(i32, i32)>> = vec![HashSet::new(); 10];

    //intialize the visited
    snake.iter().enumerate().for_each(|(i, x)| {
        visited[i].insert(*x);
    });

    //read in input
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let (direction, steps) = match line.split(' ').collect::<Vec<&str>>()[..] {
            [dir, step] => (dir, step.parse::<usize>().unwrap()),
            _ => panic!("failed parseing line {}", line),
        };
        for _ in 0..steps {
            let (mut cur_head_x, mut cur_head_y) = snake[0];
            match direction {
                "L" => cur_head_x -= 1,
                "R" => cur_head_x += 1,
                "U" => cur_head_y += 1,
                "D" => cur_head_y -= 1,
                x => panic!("invalid movement {}", x),
            };
            let new_head_pos = (cur_head_x, cur_head_y);
            snake[0] = new_head_pos;
            visited[0].insert(new_head_pos);
            for i in 1..snake.len() {
                let (cur_head_x, cur_head_y) = snake[i - 1];
                let mut new_pos = snake[i];
                let (cur_tail_x, cur_tail_y) = new_pos;
                let mut x_offset = cur_head_x - cur_tail_x;
                let mut y_offset = cur_head_y - cur_tail_y;
                if std::cmp::max(x_offset.abs(), y_offset.abs()) > 1 {
                    if y_offset.abs() >= 2 && x_offset == 0 {
                        y_offset = y_offset.clamp(-1, 1);
                    } else if x_offset.abs() > 2 && y_offset == 0 {
                        x_offset = x_offset.clamp(-1, 1);
                    } else if x_offset.abs() > 1 || y_offset.abs() > 1 {
                        x_offset = x_offset.clamp(-1, 1);
                        y_offset = y_offset.clamp(-1, 1);
                    } else {
                        x_offset = 0;
                        y_offset = 0;
                    }
                    new_pos = (cur_tail_x + x_offset, cur_tail_y + y_offset);
                }
                snake[i] = new_pos;
                visited[i].insert(new_pos);
            }
        }
    });
    let part1 = visited[1].len();
    println!("Part 1: {}", part1);
    let part2 = visited[9].len();
    println!("Part 2: {}", part2);
    Ok(())
}
