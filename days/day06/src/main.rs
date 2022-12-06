use std::fs;

fn part1() {
    println!("Day 6 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = contents.trim();
    let chars: Vec<char> = input.chars().collect();
    for (i, window) in chars.windows(4).enumerate() {
        if window.iter().all(|x| {
            window
                .iter()
                .filter(|y| x.to_string() == y.to_string())
                .collect::<Vec<&char>>()
                .len()
                == 1
        }) {
            println!("FOUND UNIQUE {i} + 4"); // Answer: 1175
            break;
        }
    }
}

fn part2() {
    println!("Day 6 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = contents.trim();
    let chars: Vec<char> = input.chars().collect();
    for (i, window) in chars.windows(14).enumerate() {
        if window.iter().all(|x| {
            window
                .iter()
                .filter(|y| x.to_string() == y.to_string())
                .collect::<Vec<&char>>()
                .len()
                == 1
        }) {
            println!("FOUND UNIQUE {i} + 14"); // Answer: 3217
            break;
        }
    }
}

fn main() {
    part1();
    part2();
}
