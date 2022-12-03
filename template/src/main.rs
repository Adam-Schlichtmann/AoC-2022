use std::fs;

fn part1() {
    println!("Day X Part X");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
}

// Answer:

fn part2() {
    println!("Day X Part X");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
}

// Answer:

fn main() {
    part1();
    part2();
}
