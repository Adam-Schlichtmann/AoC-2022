use std::collections::HashMap;
use std::fs;

struct Dir {
    contents: HashMap<String, Dir>,
    size: i32,
}

pub struct Directory {
    directories: HashMap<String, Directory>,
    files: HashMap<String, i32>,
    size: i32,
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            directories: HashMap::new(),
            files: HashMap::new(),
            size: 0,
        }
    }

    pub fn add_file(&mut self, name: String, size: i32, path: Vec<String>) {
        if path.iter().len() == 1 {
            self.files.insert(name, 0);
        } else {
            let mut root_dirs = &mut self.directories;
            let mut root_files = &mut self.files;
            for dir in path.iter() {
                if dir != "." {
                    match root_dirs.get::<String>(&dir) {
                        Some(sub_dir) => {
                            root_files = &mut sub_dir.files;
                            root_dirs = &mut sub_dir.directories;
                        }
                        None => {
                            // Add key if it doesn't exist
                            let new_dir = Directory {
                                directories: HashMap::new(),
                                files: HashMap::new(),
                                size: 0,
                            };
                            root_dirs.insert(dir.to_string(), new_dir);
                            root_files = &mut new_dir.files;
                            root_dirs = &mut new_dir.directories;
                        }
                    }
                }
            }
            root_files.insert(name, size);
        }
    }

    pub fn add_directory(&mut self, name: String, size: i32, path: Vec<String>) {
        let mut root = &mut self.directories;
        for dir in path.iter() {
            if dir != "." {
                match root.get::<String>(&dir) {
                    Some(child) => {
                        // if the key already exists set the root to the children of the child
                        root = &mut child.directories;
                    }
                    None => {
                        // Add key if it doesn't exist
                        let new_dir = Directory {
                            directories: HashMap::new(),
                            files: HashMap::new(),
                            size: size,
                        };
                        root = &mut new_dir.directories;
                        root.insert(dir.to_string(), new_dir);
                    }
                }
            }
        }
        root.insert(
            name,
            Directory {
                directories: HashMap::new(),
                files: HashMap::new(),
                size: 0,
            },
        );
    }
}

fn part1() {
    println!("Day X Part X");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let mut cd: Vec<String> = Vec::from([String::from("/")]);
    let mut directories = Directory::new();

    for line in lines {
        // If it is a command
        if line.starts_with("$") {
            let parsed_command: Vec<&str> = line.split(" ").collect();
            // If it is a cd command
            if parsed_command[1] == "cd" {
                if parsed_command[2] == "/" {
                    println!("GOING BACK TO ROOT");
                    cd.truncate(1);
                } else if parsed_command[2] == ".." {
                    println!("GOING BACK 1 DIR ");
                    cd.truncate(cd.iter().len() - 2);
                } else {
                    println!(
                        "Adding {:?} at {:?} if necessary",
                        parsed_command[2],
                        cd.join("/")
                    );
                    directories.add_directory(String::from(parsed_command[2]), 0, cd);
                    cd.push(String::from(parsed_command[2]));
                }
            } else if parsed_command[1] == "ls" {
                println!("LS in {:?}", cd.join("/"));
            }
        } else {
            // Sub Dir or File
            let parsed_ls: Vec<&str> = line.split(" ").collect();
            if parsed_ls[0] == "dir" {
                println!("LIST DIR {:?}", parsed_ls[1]);
            } else {
                println!("LIST FILE {:?}", parsed_ls[1]);
                let size: i32 = parsed_ls[0].parse().unwrap();
                directories.add_file(String::from(parsed_ls[1]), size, cd);
            }
        }
    }
    // Iterate over everything.
    // for (a, _b) in directories.contents {
    //     println!("{a}");
    // }
}

// fn part2() {
//     println!("Day X Part X");
//     let file_path = "./src/input.txt";
//     println!("In file {}", file_path);

//     let contents: String =
//         fs::read_to_string(file_path).expect("Should have been able to read the file");
// }

fn main() {
    part1();
    // part2();
}

// {
//     name: "/",
//     files: {"test1": 100, "test2":100}
//     size: 300,
//     contents: {
//         "dir1": {
//             name: "dir1",
//             contents: {}
//             size: 100,
//         }
//     }
// }
