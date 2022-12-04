use std::fs;

fn do_assignments_fully_overlap(a1: &str, a2: &str) -> bool {
    let assignment1 = parse_sections(a1);
    let assignment2 = parse_sections(a2);
    return assignment1
        .iter()
        .all(|x| assignment2.iter().any(|y| x == y))
        || assignment2
            .iter()
            .all(|x| assignment1.iter().any(|y| x == y));
}

fn do_assignments_partially_overlap(a1: &str, a2: &str) -> bool {
    let assignment1 = parse_sections(a1);
    let assignment2 = parse_sections(a2);
    return assignment1
        .iter()
        .any(|x| assignment2.iter().any(|y| x == y));
}

fn parse_sections(assignment: &str) -> Vec<i32> {
    let parts: Vec<&str> = assignment.split("-").collect();

    if parts.iter().len() != 2 {
        panic!("MORE THAN 2 numbers in assignment");
    }

    let start = parts[0].parse::<i32>().unwrap();
    let end = parts[1].parse::<i32>().unwrap();

    let range: Vec<i32> = Vec::from_iter(start..(end + 1));
    return range;
}

fn part1() {
    println!("Day 4 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let rows: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let overlapped_assignments: Vec<i32> = rows
        .iter()
        .map(|row| {
            let assignments: Vec<&str> = row.split(",").collect();
            if assignments.len() != 2 {
                panic!("ROW DOES NOT HAVE 2 ASSIGNMENTS");
            }
            if do_assignments_fully_overlap(assignments[0], assignments[1]) {
                return 1;
            }
            return 0;
        })
        .collect();
    let sum: i32 = overlapped_assignments.iter().sum();
    println!("FULLY OVERLAPPED ASSIGNMENTS: {sum}"); // Answer: 518
}

fn part2() {
    println!("Day 4 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let rows: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let overlapped_assignments: Vec<i32> = rows
        .iter()
        .map(|row| {
            let assignments: Vec<&str> = row.split(",").collect();
            if assignments.len() != 2 {
                panic!("ROW DOES NOT HAVE 2 ASSIGNMENTS");
            }
            if do_assignments_partially_overlap(assignments[0], assignments[1]) {
                return 1;
            }
            return 0;
        })
        .collect();
    let sum: i32 = overlapped_assignments.iter().sum();
    println!("PARTIALLY OVERLAPPED ASSIGNMENTS: {sum}"); // Answer: 909
}

fn main() {
    part1();
    part2();
}
