use std::fs;

pub struct Dir<'a> {
    parent: Option<&'a Dir<'a>>,
    name: String,
    children: Vec<&'a Dir<'a>>,
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
        parent: None,
        name: String::from("."),
        children: Vec::new(),
        size: 0,
    };
    // reference to current location
    let mut current = &mut root;
    for line in lines {
        // If it is a command
        if line.starts_with("$") {
            let parsed_command: Vec<&str> = line.split(" ").collect();

            // If it is a cd command
            if parsed_command[1] == "cd" {
                if parsed_command[2] == "/" {
                    // Pop back to the top
                    current = &mut root;
                } else if parsed_command[2] == ".." {
                    current = &mut current.parent.unwrap();
                } else if !current.children.iter().any(|x| x.name == parsed_command[2]) {
                    let newDir = Dir {
                        parent: Some(&current),
                        name: String::from(parsed_command[2]),
                        size: 0,
                        children: Vec::new(),
                    };

                    current.children.push(&newDir);
                    current = &mut newDir;
                } else {
                    match current.parent {
                        Some(inner) => current = &mut inner,
                        None => {
                            panic!("TRIED TO ACCESS PARENT OF ROOT");
                        }
                    }
                }
            } else if parsed_command[1] == "ls" {
                println!("LS in {:?}", current.name);
            }
        } else {
            // Sub Dir or File
            let parsed_ls: Vec<&str> = line.split(" ").collect();
            if parsed_ls[0] == "dir" {
                println!("LIST DIR {:?}", parsed_ls[1]);
            } else {
                let size: i32 = parsed_ls[0].parse().unwrap();
                current.size = current.size + size;
                let temp = current.parent.unwrap();

                loop {
                    let sum = temp.children.iter().map(|x| x.size).sum();
                    temp.size = sum;
                    match temp.parent {
                        Some(inner) => temp = inner,
                        None => {
                            break;
                        }
                    }
                    // if temp.parent.is_empty() {
                    //     break;
                    // }
                    // temp = temp.parent.unwrap();
                }
                // let reversed = stack.iter().rev();
                // for r in reversed {
                //     let children: &[Dir] = &*r.children;
                //     let mut sum = 0;
                //     for child in children {
                //         sum += child.size;
                //     }

                //     r.size = sum;
                // }
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
