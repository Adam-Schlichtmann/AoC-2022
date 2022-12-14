use std::collections::VecDeque;
use std::fs;

struct Monkey {
    name: String,
    items: VecDeque<i128>,
    worry_op: String,
    worry_by: i128,
    test_by: i128,
    true_target: i128,
    false_target: i128,
    inspections: i128,
}

fn part1() {
    println!("Day 11 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let monkeys_start_state: Vec<&str> = contents.split("\n\n").filter(|s| !s.is_empty()).collect();

    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in monkeys_start_state {
        let lines: Vec<&str> = monkey.split("\n").filter(|s| !s.is_empty()).collect();
        let items_line: Vec<&str> = lines[1].trim().split(":").collect();
        let items: Vec<&str> = items_line[1].trim().split(" ").collect();
        let numeric_items: VecDeque<i128> = items
            .iter()
            .map(|x| x.replace(",", "").parse::<i128>().unwrap())
            .collect();

        let worry: Vec<&str> = lines[2].trim().split(" ").collect();
        // I know this is bad but I don't have time
        let worry_by = if lines[2].contains("old * old") {
            0
        } else {
            worry[5].parse::<i128>().unwrap()
        };
        // I know this is bad but I don't have time
        let worry_op = if lines[2].contains("old * old") {
            String::from("^")
        } else {
            String::from(worry[4])
        };
        let test: Vec<&str> = lines[3].trim().split(" ").collect();
        let true_line: Vec<&str> = lines[4].trim().split(" ").collect();
        let false_line: Vec<&str> = lines[5].trim().split(" ").collect();
        monkeys.push(Monkey {
            name: String::from(lines[0]),
            items: numeric_items,
            worry_op: worry_op,
            worry_by: worry_by,
            test_by: test[3].parse::<i128>().unwrap(),
            true_target: true_line[5].parse::<i128>().unwrap(),
            false_target: false_line[5].parse::<i128>().unwrap(),
            inspections: 0,
        })
    }

    for _round in 0..20 {
        let monkey_len = monkeys.len();
        for i in 0..monkey_len {
            let items_len = monkeys[i].items.len();
            for j in 0..items_len {
                monkeys[i].inspections += 1;
                // Get new worry level
                let mut new_worry = monkeys[i].items[j];
                if monkeys[i].worry_op == "+" {
                    new_worry += monkeys[i].worry_by;
                } else if monkeys[i].worry_op == "*" {
                    new_worry *= monkeys[i].worry_by;
                } else if monkeys[i].worry_op == "^" {
                    new_worry *= monkeys[i].items[j];
                } else {
                    panic!("Invalid Worry Operation")
                }
                // Automatically floors the number
                new_worry /= 3;

                // Test worry level
                if new_worry % monkeys[i].test_by == 0 {
                    // Throw
                    let throw_to = monkeys[i].true_target as usize;
                    monkeys[throw_to].items.push_back(new_worry)
                } else {
                    let throw_to = monkeys[i].false_target as usize;
                    monkeys[throw_to].items.push_back(new_worry)
                }
            }
            monkeys[i].items = VecDeque::new();
        }
    }
    monkeys.sort_by(|a, b| a.inspections.partial_cmp(&b.inspections).unwrap());

    println!(
        "Monkey Business: {}",
        monkeys[monkeys.len() - 1].inspections * monkeys[monkeys.len() - 2].inspections
    ); // Answer: 72884
}

fn part2() {
    println!("Day 11 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let monkeys_start_state: Vec<&str> = contents.split("\n\n").filter(|s| !s.is_empty()).collect();

    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in monkeys_start_state {
        let lines: Vec<&str> = monkey.split("\n").filter(|s| !s.is_empty()).collect();
        let items_line: Vec<&str> = lines[1].trim().split(":").collect();
        let items: Vec<&str> = items_line[1].trim().split(" ").collect();
        let numeric_items: VecDeque<i128> = items
            .iter()
            .map(|x| x.replace(",", "").parse::<i128>().unwrap())
            .collect();

        let worry: Vec<&str> = lines[2].trim().split(" ").collect();
        // I know this is bad but I don't have time
        let worry_by = if lines[2].contains("old * old") {
            0
        } else {
            worry[5].parse::<i128>().unwrap()
        };
        // I know this is bad but I don't have time
        let worry_op = if lines[2].contains("old * old") {
            String::from("^")
        } else {
            String::from(worry[4])
        };
        let test: Vec<&str> = lines[3].trim().split(" ").collect();
        let true_line: Vec<&str> = lines[4].trim().split(" ").collect();
        let false_line: Vec<&str> = lines[5].trim().split(" ").collect();
        monkeys.push(Monkey {
            name: String::from(lines[0]),
            items: numeric_items,
            worry_op: worry_op,
            worry_by: worry_by,
            test_by: test[3].parse::<i128>().unwrap(),
            true_target: true_line[5].parse::<i128>().unwrap(),
            false_target: false_line[5].parse::<i128>().unwrap(),
            inspections: 0,
        })
    }

    let mut monkey_cycle: i128 = monkeys
        .iter()
        .map(|m| m.test_by)
        .reduce(|a, b| a * b)
        .unwrap();

    for round in 0..10000 {
        let monkey_len = monkeys.len();
        for i in 0..monkey_len {
            let items_len = monkeys[i].items.len();
            for j in 0..items_len {
                monkeys[i].inspections += 1;
                // Get new worry level
                let mut new_worry = monkeys[i].items[j];

                if monkeys[i].worry_op == "+" {
                    new_worry += monkeys[i].worry_by
                } else if monkeys[i].worry_op == "*" {
                    new_worry *= monkeys[i].worry_by
                } else if monkeys[i].worry_op == "^" {
                    new_worry *= monkeys[i].items[j]
                } else {
                    panic!("Invalid Worry Operation")
                }
                // new_worry /= 3;
                new_worry %= monkey_cycle;
                // Test worry level
                if new_worry % monkeys[i].test_by == 0 {
                    // Throw
                    let throw_to = monkeys[i].true_target as usize;
                    monkeys[throw_to].items.push_back(new_worry)
                } else {
                    let throw_to = monkeys[i].false_target as usize;
                    monkeys[throw_to].items.push_back(new_worry)
                }
            }
            monkeys[i].items = VecDeque::new();
        }
    }
    monkeys.sort_by(|a, b| a.inspections.partial_cmp(&b.inspections).unwrap());

    println!(
        "Monkey Business: {}",
        monkeys[monkeys.len() - 1].inspections * monkeys[monkeys.len() - 2].inspections,
    ); // Answer: 15310845153
}

fn main() {
    part1();
    part2();
}
