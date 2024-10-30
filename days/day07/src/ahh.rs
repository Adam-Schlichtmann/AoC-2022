use std::collections::HashMap;
use std::fs;

struct Dir {
    contents: HashMap<String, i32>,
    size: i32,
}

fn part1() {
    println!("Day X Part X");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let mut cd = Vec::from([String::from(".")]);
    let mut directories: HashMap<String, Dir> = HashMap::new();

    for line in lines {
        // If it is a command
        if line.starts_with("$") {
            let parsed_command: Vec<&str> = line.split(" ").collect();
            // If it is a cd command
            if parsed_command[1] == "cd" {
                if parsed_command[2] == "/" {
                    println!("GOING BACK TO ROOT");
                    // Remove DIR
                    cd.truncate(1);
                } else if parsed_command[2] == ".." {
                    println!("GOING BACK");
                    // Remove DIR
                    cd.truncate(cd.iter().len() - 2);
                } else {
                    let full_cd = cd.join("/");
                    println!("Adding new DIR: {full_cd}",);
                    directories.insert(
                        String::from(parsed_command[2]),
                        Dir {
                            contents: HashMap::new(),
                            size: 0,
                        },
                    );

                    cd.push(String::from(parsed_command[2]));
                }
            } else if parsed_command[1] == "ls" {
                println!("LS in {:?}", cd.join("/"))
            }
        } else {
            // Sub Dir or File
            let parsed_ls: Vec<&str> = line.split(" ").collect();
            if parsed_ls[0] == "dir" {
                println!("LIST DIR {:?}", parsed_ls[1]);
            } else {
                println!("LIST FILE {:?}", parsed_ls[1]);
                let full_cd = cd.join("/");
                let size: i32 = parsed_ls[0].parse().unwrap();

                // Ensure That the path exists
                directories.entry(full_cd).or_insert(Dir {
                    contents: HashMap::new(),
                    size: 0,
                });

                // Insure the file
                directories
                    .entry(full_cd)
                    .entry("contents")
                    .entry(String::from(parsed_ls[1]))
                    .or_insert(size);
            }
        }
    }
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
