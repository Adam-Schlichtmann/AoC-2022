use std::collections::HashMap;
use std::fs;

pub struct Dir {
    name: String,
    files: Vec<i32>,
    size: i32,
}

fn part1() {
    println!("Day 7 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let mut cd: Vec<String> = Vec::from([String::from(".")]);
    let mut directories: Vec<Dir> = vec![Dir {
        name: String::from("."),
        files: Vec::new(),
        size: 0,
    }];
    for line in lines {
        println!("{:?}", cd.join("/"));
        println!("{:?}", directories.iter().len());
        // If it is a command
        if line.starts_with("$") {
            let parsed_command: Vec<&str> = line.split(" ").collect();
            // If it is a cd command

            if parsed_command[1] == "cd" {
                cd.push(String::from(parsed_command[2]));
                let full_dir = cd.join("/");
                if parsed_command[2] == "/" {
                    println!("GOING BACK TO ROOT");
                    cd.truncate(1);
                } else if parsed_command[2] == ".." {
                    println!("GOING BACK 1 DIR ");
                    cd.truncate(cd.iter().len() - 2);
                } else if !directories.iter().any(|x| x.name == full_dir) {
                    println!(
                        "Adding {:?} at {:?} if necessary",
                        parsed_command[2], full_dir
                    );

                    directories.push(Dir {
                        name: full_dir,
                        size: 0,
                        files: Vec::new(),
                    })
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
                let full_dir = cd.join("/");
                let position = directories.iter().position(|x| x.name == full_dir).unwrap();
                if position >= 0 {
                    println!("LIST FILE {:?}", parsed_ls[1]);
                    let size: i32 = parsed_ls[0].parse().unwrap();
                    directories[position].files.push(size);
                    directories[position].size = directories[position].files.iter().sum();
                } else {
                    panic!("dir not found")
                }
            }
        }
    }
    // Sum child dirs into parents
    let sums: HashMap<String, i32> = HashMap::new();
    for dir in directories {
        if sums.contains_key(&dir.name) {
            panic!("HOW DOES THIS ALREADY EXIST {:?}", dir.name);
        } else {
            sums.insert(dir.name, dir.size);
        }
    }
    // let sums_including_subs: HashMap<String, i32> = HashMap::new();
    // for (name, size) in &sums {
    //     let mut new_sum: i32 = *size;
    //     for dir in &directories {
    //         if dir.name.contains(name) {
    //             new_sum += dir.size;
    //         }
    //     }
    //     sums_including_subs.insert(*name, new_sum);
    // }
    let sums_including_subs: Vec<i32> = Vec::new();
    for (name, size) in &sums {
        let mut new_sum = 0;
        for dir in &directories {
            if dir.name.contains(name) {
                new_sum += dir.size;
            }
        }
        sums_including_subs.push(new_sum + size);
    }
    // let mut changes = true;
    // while changes {
    //     changes = false;
    //     for dir in directories.iter_mut() {
    //         let sub_dirs: Vec<i32> = directories
    //             .into_iter()
    //             .filter(|x| x.name.contains(&dir.name))
    //             .map(|x| x.size)
    //             .collect();
    //         let sub_dir_sum = sub_dirs.iter().sum();
    //         if sub_dir_sum != dir.size {
    //             changes = true;
    //             dir.size = sub_dir_sum
    //         }
    //     }
    // }

    // Results
    let mut sum: i32 = 0;
    for dir in &directories {
        println!(
            "DIR: {:?} with {:?} files at: {:?}",
            dir.name,
            dir.files.len(),
            dir.size
        );
        if dir.size <= 100000 {
            sum += dir.size;
        }
    }
    let filtered: Vec<&i32> = sums_including_subs
        .iter()
        .filter(|x| **x <= 10000)
        .collect();
    let real_sum: i32 = sums_including_subs.iter().sum();
    // for (name, size) in &sums_including_subs {
    //     if size <= 100000 {
    //         real_sum += size;
    //     }
    // }
    println!("TOTAL FOR DIRS UNDER 100000 {real_sum}")

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
