#![warn(clippy::all, clippy::pedantic)]

use std::cell::RefCell;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::rc::Rc;

trait Sizer {
    fn size(&self) -> usize;
}

impl Sizer for Vec<usize> {
    fn size(&self) -> usize {
        self.iter().sum()
    }
}

impl<T: Sizer> Sizer for Vec<T> {
    fn size(&self) -> usize {
        self.iter().map(Sizer::size).sum()
    }
}

#[derive(Clone, Debug)]
struct MyFile {
    _name: String,
    size: usize,
}

impl Sizer for MyFile {
    fn size(&self) -> usize {
        self.size
    }
}

#[derive(Clone, Debug)]
struct MyDir {
    name: String,
    objects: Vec<FileSystemTypes>,
    parent_dir: Option<Rc<RefCell<MyDir>>>,
}

impl MyDir {
    fn move_up(&self) -> Option<Rc<RefCell<MyDir>>> {
        self.parent_dir.clone()
    }

    fn move_down(&self, dir: impl Into<String>) -> Option<Rc<RefCell<MyDir>>> {
        let dir = dir.into();
        Some(
            self.objects
                .iter()
                .filter_map(|x| match x {
                    FileSystemTypes::MyDir(y) => Some(y),
                    FileSystemTypes::MyFile(_) => None,
                })
                .find(|x| *x.borrow().name == dir)?
                .clone(),
        )
    }
    fn touch(&mut self, name: impl Into<String>, size: usize) {
        self.objects.push(FileSystemTypes::MyFile(MyFile {
            _name: name.into(),
            size,
        }));
    }
    //has to be  a static method...
    fn mkdir(self_: &Rc<RefCell<MyDir>>, name: impl Into<String>) {
        self_
            .borrow_mut()
            .objects
            .push(FileSystemTypes::MyDir(Rc::new(RefCell::new(MyDir::new(
                name,
                Some(self_.clone()),
            )))));
        //me.clone()
    }

    fn new(name: impl Into<String>, parent_dir: Option<Rc<RefCell<MyDir>>>) -> Self {
        let name: String = name.into();
        MyDir {
            name,
            objects: Vec::new(),
            parent_dir,
        }
    }
}
impl Sizer for MyDir {
    fn size(&self) -> usize {
        self.objects.size()
    }
}

#[derive(Clone, Debug)]
enum FileSystemTypes {
    MyFile(MyFile),
    MyDir(Rc<RefCell<MyDir>>),
}

impl Sizer for FileSystemTypes {
    fn size(&self) -> usize {
        match self {
            FileSystemTypes::MyFile(file) => file.size(),
            FileSystemTypes::MyDir(dir) => dir.borrow().size(),
        }
    }
}

fn recurse_part1(collector: &mut Vec<usize>, cwd: &MyDir) -> usize {
    let cwd_size: usize = cwd
        .objects
        .iter()
        .map(|x| match x {
            FileSystemTypes::MyFile(y) => y.size(),
            FileSystemTypes::MyDir(dir) => recurse_part1(collector, &dir.borrow()),
        })
        .sum();
    collector.push(cwd_size);
    cwd_size
}

fn process_part1(input: &str) -> String {
    todo!()
}

fn process_part2(input: &str) -> String {
    todo!()
}

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    //create root file object on heap
    let root = Rc::new(RefCell::new(MyDir::new("/", None)));
    let mut ls_mode = false;
    //set a pointer to the currently on MyDir, in this case start at roo
    let mut cursor = root.clone();

    // loop through the input files lines
    reader.lines().for_each(|line| {
        //need to unwrap the line cause lines() returns an Option
        let line = line.unwrap();
        //if we are listing files we need to  get the information from the input
        if ls_mode && line.as_bytes()[0] != b'$' {
            //do adding to cursor
            match line.split_whitespace().collect::<Vec<_>>()[..] {
                ["dir", name] => MyDir::mkdir(&cursor.clone(), name),
                [size, name] => cursor
                    .borrow_mut()
                    .touch(name, size.parse::<usize>().unwrap()),
                _ => panic!("oops {line}"),
            };
            // end the for_each
            return;
        }
        ls_mode = false;
        //parse all other lines as commands
        cursor = match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["$", "ls"] => {
                ls_mode = true;
                cursor.clone()
            }
            ["$", "cd", "/"] => root.clone(), //set current directory back to root
            ["$", "cd", ".."] => cursor.borrow().move_up().unwrap(),
            ["$", "cd", dir] => cursor.borrow().move_down(dir).unwrap(),
            _ => panic!("unknown command {line}"),
        }
    });
    let mut part1 = Vec::new();
    let max = recurse_part1(&mut part1, &root.borrow());
    let part1_ans: usize = part1.iter().filter(|x| **x <= 100_000).sum();
    println!("Part 1: {part1_ans}");

    //part 2
    let free_space = 70_000_000_usize - max;
    let needed_clear_space = 30_000_000_usize - free_space;
    let part2 = part1.iter().filter(|x| **x >= needed_clear_space).min();
    println!("Part 2: {}", part2.unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "95437");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "24933642");
    }
}
