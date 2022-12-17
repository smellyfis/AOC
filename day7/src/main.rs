use std::cell::RefCell;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::rc::Rc;

trait Sizer {
    fn size(&self) -> usize;
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
                    _ => None,
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
    fn mkdir(self_: Rc<RefCell<MyDir>>, name: impl Into<String>) {
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
        let parent_dir = parent_dir.map(|x| x.clone());
        MyDir {
            name,
            objects: Vec::new(),
            parent_dir,
        }
    }
}
impl Sizer for MyDir {
    fn size(&self) -> usize {
        self.objects
            .iter()
            .map(|obj|  obj.size())
            .sum::<usize>()
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
                ["dir", name] => MyDir::mkdir(cursor.clone(), name),
                [size, name] => cursor
                    .borrow_mut()
                    .touch(name, size.parse::<usize>().unwrap()),
                _ => panic!("oops {}", line),
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
            ["$", "cd", ".."] => cursor.borrow().move_up().unwrap().clone(),
            ["$", "cd", dir] => cursor.borrow().move_down(dir).unwrap().clone(),
            _ => panic!("unknown command {}", line),
        }
    });
    let mut part1 = vec![root.borrow().size()];
    // TODO do a recursive function that get current directory size and puts it in vector then go
    // into each sub directory
    // find all less than 100_000 and sum

    Ok(())
}
