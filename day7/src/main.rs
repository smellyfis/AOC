use std::fs::File;
use std::io::{prelude::*, BufReader};

struct MyFile {
    name: String,
    size: usize,
}

struct MyDir <'a>{
    name: String,
    objects: Vec<FileSytemTypes<'a>>,
    //TODO should this bee reference counted
    parent_dir: Option<&'a MyDir<'a>>,
}

impl <'a> MyDir<'a> {
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

enum FileSytemTypes <'a>{
    MyFile(MyFile),
    MyDir(&'a Box<MyDir<'a>>),
}

fn main() -> std::io::Result<()> {
    //Read in file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    //create root file object on heap
    let root = Box::new(MyDir::new("/", None));
    let mut ls_mode = false;
    //set a pointer to the currently on MyDir, in this case start at roo
    let mut cursor = root.as_mut();

    // loop through the input files lines
    reader.lines().for_each(|line| {
        //need to unwrap the line cause lines() returns an Option
        let line = line.unwrap();
        //if we are listing files we need to  get the information from the input
        if ls_mode && line.as_bytes()[0] != '$' as u8 {
            //do adding to cursor
            match line.split_whitespace().collect::<Vec<_>>()[..] {
                ["dir", name] => { // sub directories start with dir
                    // create the subdirectory on the heap
                    let new_directory = &Box::new(MyDir::new(name, Some(cursor)));
                    // attach the new directory to the current directory
                    let obj: &mut Vec<FileSytemTypes<'_>> = cursor.objects.as_mut();
                    obj.push(FileSytemTypes::MyDir(new_directory));
                }
                [size, name] => cursor.objects.push(FileSytemTypes::MyFile(MyFile {
                    name: name.to_string(),
                    size: size.parse::<usize>().unwrap(),
                })),
                _ => panic!("oops {}", line),
            };
            // end the for_each
            return;
        }
        ls_mode = false;
        //parse all other lines as commands
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["$", "ls"] => ls_mode = true,
            ["$", "cd", "/"] => cursor = root.as_mut(), //set current directory back to root
            ["$", "cd", ".."] => {
                // set current directory to the parent directory
                cursor = match cursor.parent_dir {
                    Some( x) => &mut x,
                    _ => panic!("issue {}", line),
                }
            }
            ["$", "cd", dir] => {
                //set the currect directory to the subdirectory named dir
                cursor = cursor
                    .objects
                    .iter()
                    .filter_map(|x| match x {
                        FileSytemTypes::MyDir(y) => Some(y.as_mut()),
                        _ => None,
                    })
                    .find(|x| x.name == dir)
                    .unwrap();
            }
            _ => panic!("unknown command {}", line),
        }
    });

    Ok(())
}
