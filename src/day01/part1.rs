use std::fs;

pub fn part1() {
    println!("Day 1 Part 1");
    let file_path = "./src/day01/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let elves: Vec<&str> = contents.split("\n\n").collect();

    let mut max: i32 = 0;
    for elf in elves {
        let foods: Vec<&str> = elf.split("\n").collect();
        println!("Foods:\n{:?}", foods);
        let mut sum: i32 = 0;
        for food in foods {
            if food != "" {
                sum += food.parse::<i32>().unwrap()
            }
        }
        if sum > max {
            max = sum
        }
    }
    println!("Most Calories {max}")
}

// Answer: 72478
