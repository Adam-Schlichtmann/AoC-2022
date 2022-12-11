use std::fs;
use std::{thread, time};

fn get_signal_strength(cycle: i32, x: i32) -> i32 {
    return cycle * x;
}

fn should_sum_signal_strength(cycle: i32) -> bool {
    return ((cycle - 20) % 40) == 0;
}

fn part1() {
    println!("Day 10 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let instructions: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    for instruction in instructions {
        if instruction == "noop" {
            cycle += 1;
            if should_sum_signal_strength(cycle) {
                sum += get_signal_strength(cycle, x)
            }
        } else if instruction.contains("addx") {
            let parts: Vec<&str> = instruction.split(" ").collect();
            let value = parts[1].parse::<i32>().unwrap();
            for _n in 0..2 {
                cycle += 1;
                if should_sum_signal_strength(cycle) {
                    sum += get_signal_strength(cycle, x)
                }
            }
            x += value;
        } else {
            panic!("UNKNOWN COMMAND {instruction}")
        }
    }
    println!("Sum of cycles: {sum}"); // Answer: 10760
}

fn is_pixel_lit(cycle: i32, x: i32) -> bool {
    return ((cycle % 40) - 1) - x >= -1 && ((cycle % 40) - 1) - x <= 1;
}

fn print_screen(screen: [&str; 240]) {
    let delay = time::Duration::from_millis(100);
    thread::sleep(delay);
    // Clear
    print!("{}[2J", 27 as char);
    for (i, c) in screen.iter().enumerate() {
        if i % 40 == 0 && i != 0 {
            print!("\n")
        }
        print!("{c}");
    }
    print!("\n\n\n\n\n\n")
}

fn part2() {
    println!("Day 10 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let instructions: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let mut cycle = 0;
    let mut x = 1;
    let mut screen: [&str; 240] = ["."; 240];
    for instruction in instructions {
        if instruction == "noop" {
            cycle += 1;
            if is_pixel_lit(cycle, x) {
                screen[(cycle - 1) as usize] = "#";
            } else {
                screen[(cycle - 1) as usize] = ".";
            }
            print_screen(screen);
        } else if instruction.contains("addx") {
            let parts: Vec<&str> = instruction.split(" ").collect();
            let value = parts[1].parse::<i32>().unwrap();
            for _n in 0..2 {
                cycle += 1;
                if is_pixel_lit(cycle, x) {
                    screen[(cycle - 1) as usize] = "#";
                } else {
                    screen[(cycle - 1) as usize] = ".";
                }
                print_screen(screen);
            }
            x += value;
        } else {
            panic!("UNKNOWN COMMAND {instruction}")
        }
    }
    print_screen(screen); // Answer: FPGPHFGH
}

fn main() {
    part1();
    part2();
}
