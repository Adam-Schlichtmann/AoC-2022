use std::fs;

// fn parse_line(line: &str, depth: i32) {
//     if line[0] == '[' && line[line.len() - 1] = ']' {
//         let mut new_line = line.remove(line.len() - 1);
//         let mut new_line = line.remove(0);
//         return parse_line(line, depth + 1);
//     }

// }

fn part1() {
    println!("Day 13 Part 1");
    let file_path = "./src/test.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let pairs: Vec<&str> = contents.split("\n\n").filter(|s| !s.is_empty()).collect();

    let mut failed_indexes: Vec<i32> = Vec::new();
    for i in 0..pairs.len() {
        let packets: Vec<&str> = pairs[i].split("\n").collect();
        let mut left: Vec<char> = packets[0].chars().collect();
        let mut left_depth = 0;
        let mut right: Vec<char> = packets[1].chars().collect();
        let mut right_depth = 0;
        println!("{:?}", left);
        for l in 0..left.len() {
            if left[l] == '[' {
                left_depth += 1;
            }
            if right[l] == '[' {
                right_depth += 1;
            }

            if left[l] == ']' {
                left_depth -= 1;
            }
            if right[l] == ']' {
                right_depth -= 1;
            }

            if left_depth > right_depth {
                right.insert(l, '[');
            }
            if right_depth > left_depth {
                left.insert(l, '[');
            }

            if left_depth < right_depth {
                right.insert(l, ']');
                failed_indexes.push(i as i32);
            }
            if right_depth < left_depth {
                left.insert(l, ']');
            }

            if left[l].is_digit(10) && right[l].is_digit(10) {
                if left[l].to_digit(10).unwrap() > right[l].to_digit(10).unwrap() {
                    failed_indexes.push(i as i32);
                }
            }
        }
    }
    let sum: i32 = failed_indexes.iter().sum();
    println!("Sum {}", sum);
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
