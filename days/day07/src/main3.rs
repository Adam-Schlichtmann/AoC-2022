use std::fs;
use std::ptr;

pub struct Dir {
    parent: *const Dir,
    name: String,
    children: Vec<Dir>,
    size: i32,
}

fn part1() {
    println!("Day 7 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let mut root: Dir = Dir {
        parent: ptr::null(),
        name: String::from("."),
        children: Vec::new(),
        size: 0,
    };

    let mut current: &mut Dir = &mut root;

    for line in lines {
        // If it is a command
        if line.starts_with("$") {
            let parsed_command: Vec<&str> = line.split(" ").collect();

            if parsed_command[1] == "cd" {
                if parsed_command[2] == "/" {
                    // Pop back to the top
                    current = &mut root;
                } else if parsed_command[2] == ".." {
                    current = &mut *(*current).parent;
                } else
                // if !stack[stack.len() - 1]
                //     .children
                //     .iter()
                //     .any(|x| x.name == parsed_command[2])
                {
                    // Adding new dir
                    let newDir = Dir {
                        parent: current,
                        name: String::from(parsed_command[2]),
                        size: 0,
                        children: Vec::new(),
                    };

                    (*current).children.push(newDir)
                }
            } else if parsed_command[1] == "ls" {
                println!("LS in {:?}", (*current).name);
            }
        } else {
            // Sub Dir or File
            let parsed_ls: Vec<&str> = line.split(" ").collect();
            if parsed_ls[0] == "dir" {
                println!("LIST DIR {:?}", parsed_ls[1]);
            } else {
                // LS Of file
                let size: i32 = parsed_ls[0].parse().unwrap();
                (*current).size += size;
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
