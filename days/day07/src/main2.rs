use core::ops::Index;
use core::ops::IndexMut;
use std::fs;

pub struct Dir {
    name: String,
    children: Vec<&Dir>,
    size: i32,
}

impl Index<usize> for Dir {
    type Output = Dir;
    fn index<'a>(&'a self, idx: usize) -> &'a Dir {
        return &self.children[idx];
    }
}

impl IndexMut<usize> for Dir {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Dir {
        // even here I cannot get mutable reference to self.data[idx]
        return self.children.index_mut(idx);
    }
}

fn part1() {
    println!("Day 7 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let root: Dir = Dir {
        name: String::from("."),
        children: Vec::new(),
        size: 0,
    };
    // List of references to Dir nodes
    let mut stack: Vec<&Dir> = Vec::new();
    stack.push(&root);
    for line in lines {
        // If it is a command
        if line.starts_with("$") {
            let parsed_command: Vec<&str> = line.split(" ").collect();

            // If it is a cd command
            if parsed_command[1] == "cd" {
                if parsed_command[2] == "/" {
                    // Pop back to the top
                    stack.truncate(1);
                } else if parsed_command[2] == ".." {
                    stack.truncate(stack.len() - 2);
                } else
                // if !stack[stack.len() - 1]
                //     .children
                //     .iter()
                //     .any(|x| x.name == parsed_command[2])
                {
                    // Adding new dir
                    let newDir = Dir {
                        name: String::from(parsed_command[2]),
                        size: 0,
                        children: Vec::new(),
                    };

                    let len = stack.len() - 1;
                    stack[len].children.push(&newDir);
                    stack.push(&newDir);
                }
            } else if parsed_command[1] == "ls" {
                println!("LS in {:?}", stack[stack.len() - 1].name);
            }
        } else {
            // Sub Dir or File
            let parsed_ls: Vec<&str> = line.split(" ").collect();
            if parsed_ls[0] == "dir" {
                println!("LIST DIR {:?}", parsed_ls[1]);
            } else {
                let size: i32 = parsed_ls[0].parse().unwrap();

                let stack_len = stack.len();
                for i in 0..stack_len {
                    let mut sum = 0;
                    let child_len = stack[i].children.len();
                    for c in 0..child_len {
                        sum += stack[i].children[c].size;
                    }
                    stack[i].size = sum + size;
                }
            }
        }
    }

    // println!("ROOT SIZE: {:?}", root.size)
    // println!("TOTAL FOR DIRS UNDER 100000 {:?}", root.size)
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
