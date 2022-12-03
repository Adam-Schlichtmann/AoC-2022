use std::fs;

fn score_item(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        return u32::from(item) - 96;
    } else if item.is_ascii_uppercase() {
        return u32::from(item) - 38;
    }
    panic!("Can't score char")
}

fn split_sack_in_half(item: &str) -> Vec<&str> {
    let middle = item.chars().count() / 2;
    let mut result: Vec<&str> = Vec::new();
    let first = &item[..middle];
    let second = &item[middle..];
    if first.len() + second.len() != item.len() {
        println!("{:?} | {:?} | {:?}", item.len(), first.len(), second.len());
        panic!("SPLIT FAILED")
    }
    result.push(first);
    result.push(second);
    return result;
}

fn get_item_in_both_sacks(left: &str, right: &str) -> char {
    for lc in left.chars() {
        if right.contains(lc) {
            return lc;
        }
    }
    println!("{left} | {right}");
    panic!("NO MATCHING CHARS");
}

fn part1() {
    println!("Day 3 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let bags: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let bag_scores: Vec<u32> = bags
        .iter()
        .map(|bag| {
            let parts = split_sack_in_half(bag);
            let item = get_item_in_both_sacks(parts[0], parts[1]);
            return score_item(item);
        })
        .collect();
    let sum: u32 = bag_scores.iter().sum();
    println!("Priority Totals: {sum}") // Answer: 8053
}

fn get_badge_for_elf_group(group: Vec<&str>) -> char {
    if group.len() == 0 {
        panic!("empty group");
    }
    let common_item: Option<char> = group[0].chars().find(|item| {
        if group.iter().all(|bag| {
            return bag.contains(*item);
        }) {
            return true;
        }
        return false;
    });
    if common_item.is_none() {
        println!("{:?}", group);
        panic!("UNABLE TO FIND ITEM IN GROUP");
    }
    return common_item.unwrap();
}

fn part2() {
    println!("Day 3 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let bags: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let mut group_priorities: Vec<u32> = Vec::new();
    for group in bags.chunks(3) {
        let item = get_badge_for_elf_group(group.to_vec());
        group_priorities.push(score_item(item));
    }

    let sum: u32 = group_priorities.iter().sum();
    println!("Priority Totals: {sum}") // Answer: 2425
}

fn main() {
    part1();
    part2();
}
