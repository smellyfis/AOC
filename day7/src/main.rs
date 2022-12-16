use std::fs::File;
use std::rc::Rc;
use std::io::{prelude::*, BufReader};

struct MyFile {
    name: String,
    size: usize,
}

struct MyDir <'a>{
    name: String,
    objects: Vec<FileSystemTypes<'a>>,
    //TODO should this bee reference counted
    parent_dir: Option<&'a MyDir<'a>>,
}

impl <'a> MyDir<'a> {
    fn move_up(&self) -> Option<&MyDir>{
        self.parent_dir
    }

    fn move_down(&self, dir:impl Into<String>) -> Option<&MyDir> {
        let dir = dir.into();
        self.objects.iter()
                    .filter_map(|x| match x {
                        FileSystemTypes::MyDir(y) => Some(y),
                        _ => None,
                    })
                    .find(|x| x.name == dir)
    }
    fn touch(&mut self, name: impl Into<String>, size:usize) {
        self.objects.push(FileSystemTypes::MyFile(MyFile{name:name.into(), size}));
    }
    fn mkdir(&'a mut self, name: impl Into<String>) {
        self.objects.push(FileSystemTypes::MyDir(MyDir::new(name, Some(self))));
    }


    fn new(name: impl Into<String>, parent_dir: Option<&'a MyDir<'a>>) -> Self {
        let name: String = name.into();
        let parent_dir = match parent_dir {
            Some(x) => Some(x),
            None => None,
        };
        MyDir {
            name,
            objects: Vec::new(),
            parent_dir,
        }
    }
}

enum FileSystemTypes <'a>{
    MyFile(MyFile),
    MyDir(MyDir<'a>),
}

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    //create root file object on heap
    let root = MyDir::new("/", None);
    let mut ls_mode = false;
    //set a pointer to the currently on MyDir, in this case start at roo
    let mut cursor = Box::from(&root).as_mut();

    // loop through the input files lines
    reader.lines().for_each(|line| {
        //need to unwrap the line cause lines() returns an Option
        let line = line.unwrap();
        //if we are listing files we need to  get the information from the input
        if ls_mode && line.as_bytes()[0] != '$' as u8 {
            //do adding to cursor
            match line.split_whitespace().collect::<Vec<_>>()[..] {
                ["dir", name] =>  cursor.mkdir(name),
                [size, name] => cursor.touch(name, size.parse::<usize>().unwrap()),
                _ => panic!("oops {}", line),
            };
            // end the for_each
            return;
        }
        ls_mode = false;
        //parse all other lines as commands
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["$", "ls"] => ls_mode = true,
            ["$", "cd", "/"] => cursor = Box::from(&root).as_mut(), //set current directory back to root
            ["$", "cd", ".."] => cursor = Box::from(cursor.move_up().unwrap()).as_mut(),
            ["$", "cd", dir] => cursor = Box::from(cursor.move_down(dir).unwrap()).as_mut(),
            _ => panic!("unknown command {}", line),
        }
    });

    Ok(())
}
