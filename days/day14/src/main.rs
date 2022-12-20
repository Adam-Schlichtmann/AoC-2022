use std::cmp;
use std::fs;

fn part1() {
    println!("Day 14 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let mut graph: [[char; 100]; 200] = [['.'; 100]; 200];
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 1000;
    let mut min_y = 1000;

    // Set source
    graph[0][50] = '+';
    for i in 0..lines.len() {
        let rocks: Vec<&str> = lines[i].split("->").filter(|s| !s.is_empty()).collect();
        let mut prev_x = 1000;
        let mut prev_y = 1000;
        for j in 0..rocks.len() {
            let coords_pairs: Vec<&str> = rocks[j]
                .trim()
                .split(",")
                .filter(|s| !s.is_empty())
                .collect();

            let new_x = coords_pairs[0].trim().parse::<usize>().unwrap();
            let new_y = coords_pairs[1].trim().parse::<usize>().unwrap();
            if prev_x < 1000 && prev_y < 1000 {
                if new_x != prev_x && new_y != prev_y {
                    panic!("Are diagonal coords allowed");
                }
                if new_x == prev_x {
                    for y in 0..(cmp::max(prev_y, new_y) - cmp::min(prev_y, new_y)) {
                        graph[cmp::min(prev_y, new_y) + y][prev_x - 450] = '#';
                    }
                } else {
                    for x in 0..(cmp::max(prev_x, new_x) - cmp::min(prev_x, new_x) + 1) {
                        graph[new_y][cmp::min(prev_x, new_x) + x - 450] = '#';
                    }
                }
            }
            prev_x = new_x;
            prev_y = new_y;
            if new_x > max_x {
                max_x = new_x
            }
            if new_y > max_y {
                max_y = new_y
            }

            if new_x < min_x {
                min_x = new_x
            }
            if new_y < min_y {
                min_y = new_y
            }
        }
    }

    let mut count = 0;
    let mut rock_stopped = true;
    while rock_stopped {
        if graph[0][50] != '+' {
            rock_stopped = false;
        } else {
            let mut new_x = 50;
            let mut new_y = 0;
            let mut can_move = true;
            while can_move {
                if new_y + 1 >= graph.len() {
                    println!("THE ABYSS");
                    can_move = false;
                    rock_stopped = false;
                } else if graph[new_y + 1][new_x] == '.' {
                    new_y += 1;
                } else if graph[new_y + 1][new_x - 1] == '.' {
                    new_y += 1;
                    new_x -= 1;
                } else if graph[new_y + 1][new_x + 1] == '.' {
                    new_y += 1;
                    new_x += 1;
                } else {
                    can_move = false;
                    count += 1;
                    graph[new_y][new_x] = 'o';
                }
            }
        }
    }
    println!("Max Values: {max_x},{max_y}");
    println!("Min Values: {min_x},{min_y}");
    println!("X Range: {}", max_x - min_x);
    println!("Y Range: {}", max_y - min_y);
    println!("{} Rocks stopped before the abyss", count); // Answer: 638
}

fn part2() {
    const X_SIZE: usize = 400;
    const Y_SIZE: usize = 184;
    println!("Day 14 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    // height in my case is 184
    // This means the width needs to be 184 * sqr(2) * 2 which is ~520
    // There is some math that could be done to not require making the graph this big but...
    // I barely have time to do this as it is so we are using more processing
    let mut graph = [['.'; X_SIZE]; Y_SIZE];
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 1000;
    let mut min_y = 1000;

    // Set source
    graph[0][X_SIZE / 2] = '+';
    // Make the floor
    for x in 0..graph[0].len() {
        graph[graph.len() - 1][x] = '#';
    }
    for i in 0..lines.len() {
        let rocks: Vec<&str> = lines[i].split("->").filter(|s| !s.is_empty()).collect();
        let mut prev_x = 1000;
        let mut prev_y = 1000;
        for j in 0..rocks.len() {
            let coords_pairs: Vec<&str> = rocks[j]
                .trim()
                .split(",")
                .filter(|s| !s.is_empty())
                .collect();

            let new_x = coords_pairs[0].trim().parse::<usize>().unwrap();
            let new_y = coords_pairs[1].trim().parse::<usize>().unwrap();
            if prev_x < 1000 && prev_y < 1000 {
                if new_x != prev_x && new_y != prev_y {
                    panic!("Are diagonal coords allowed");
                }
                if new_x == prev_x {
                    for y in 0..(cmp::max(prev_y, new_y) - cmp::min(prev_y, new_y)) {
                        graph[cmp::min(prev_y, new_y) + y][prev_x - (500 - (X_SIZE / 2))] = '#';
                    }
                } else {
                    for x in 0..(cmp::max(prev_x, new_x) - cmp::min(prev_x, new_x) + 1) {
                        graph[new_y][cmp::min(prev_x, new_x) + x - (500 - (X_SIZE / 2))] = '#';
                    }
                }
            }
            prev_x = new_x;
            prev_y = new_y;
            if new_x > max_x {
                max_x = new_x
            }
            if new_y > max_y {
                max_y = new_y
            }

            if new_x < min_x {
                min_x = new_x
            }
            if new_y < min_y {
                min_y = new_y
            }
        }
    }

    let mut count = 0;
    let mut rock_stopped = true;
    while rock_stopped {
        if graph[0][X_SIZE / 2] != '+' {
            rock_stopped = false;
        } else {
            let mut new_x = X_SIZE / 2;
            let mut new_y = 0;
            let mut can_move = true;
            while can_move {
                if graph[new_y + 1][new_x] == '.' {
                    new_y += 1;
                } else if graph[new_y + 1][new_x - 1] == '.' {
                    new_y += 1;
                    new_x -= 1;
                } else if graph[new_y + 1][new_x + 1] == '.' {
                    new_y += 1;
                    new_x += 1;
                } else {
                    can_move = false;
                    count += 1;
                    graph[new_y][new_x] = 'o';
                }
            }
        }
    }
    println!("Max Values: {max_x},{max_y}");
    println!("Min Values: {min_x},{min_y}");
    println!("X Range: {}", max_x - min_x);
    println!("Y Range: {}", max_y - min_y);
    println!("{} Rocks stopped before the abyss", count); // Answer: 31722
}

fn main() {
    part1();
    part2();
}
