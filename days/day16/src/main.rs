use regex::Regex;
use std::fs;

struct Valve {
    name: String,
    flow: i32,
    connections: Vec<String>,
}

impl Valve {
    fn get_flow(&self) -> i32 {
        return self.flow;
    }
}

struct Volcano {
    valves: Vec<Valve>,
    release_rate: i32,
    total_released: i32,
    opened_valves: Vec<String>,
}

fn get_valve<'a>(valves: &'a mut Vec<Valve>, valve: String) -> Option<&'a mut Valve> {
    for v in valves {
        if *v.name == valve {
            return Some(v);
        }
    }
    None
}

impl Volcano {
    fn new() -> Volcano {
        Volcano {
            valves: Vec::new(),
            release_rate: 0,
            total_released: 0,
            opened_valves: Vec::new(),
        }
    }

    fn add_valve(&mut self, valve: Valve) {
        self.valves.push(valve);
    }

    fn open_valve(&mut self, valve: String) {
        self.opened_valves.push(valve);
    }

    fn get_valve_flow(&self, valve: String) -> i32 {
        let current = self.valves.iter().find(|x| x.name == valve).unwrap();
        return current.get_flow();
    }

    fn get_valve_greedy_score(&self, valve: String, rounds_remaining: i32) -> i32 {
        let current = self.valves.iter().find(|x| x.name == valve).unwrap();
        return current.get_flow() * rounds_remaining;
    }

    fn get_connections(&self, valve: String) -> Vec<String> {
        let current = self.valves.iter().find(|x| x.name == valve).unwrap();
        return (*current.connections).to_vec();
    }

    fn is_open(&self, valve: String) -> bool {
        return self.opened_valves.contains(&valve);
    }

    fn update_release_rate(&mut self, valve: String) {
        self.release_rate += self.get_valve_flow(valve);
    }

    fn update_total_released(&mut self) {
        self.total_released += self.release_rate;
    }
}

const ROUNDS: i32 = 30;
fn part1() {
    println!("Day X Part X");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let number_regex = Regex::new(r"[^0-9-]").unwrap();
    let letter_regex = Regex::new(r"[^A-Z]").unwrap();
    let mut volcano = Volcano::new();
    for i in 0..lines.len() {
        let parts: Vec<&str> = lines[i].split(" ").collect();
        let name = parts[1];
        let flow_parts: String = number_regex.replace_all(parts[4], "").into();
        let flow = flow_parts.parse::<i32>().unwrap();
        // Anything after 8
        let next: Vec<String> = parts[9..]
            .iter()
            .map(|s| letter_regex.replace_all(s, "").into())
            .collect();
        volcano.add_valve(Valve {
            name: String::from(name),
            flow,
            connections: next,
        });
    }
    for valve in &volcano.valves {
        println!(
            "Name: {} | Flow: {:2} | Connections {:?}",
            valve.name, valve.flow, valve.connections
        );
    }

    let mut current = String::from("AA");

    for round in 0..ROUNDS {
        println!("Round {round} {current}");

        let current_open = volcano.is_open((*current).to_string());
        let mut best_score = if current_open {
            -1
        } else {
            volcano.get_valve_greedy_score((*current).to_string(), ROUNDS - round)
        };
        let mut best_node: String = if current_open {
            String::from("")
        } else {
            (*current).to_string()
        };
        let connections = volcano.get_connections((*current).to_string());
        for (i, connection) in connections.iter().enumerate() {
            let connection_open = volcano.is_open((*connection).to_string());
            let score =
                volcano.get_valve_greedy_score((*connection).to_string(), ROUNDS - round - 1);
            println!("New Score: {score} | Old Score: {best_score}");
            if score > best_score && !connection_open {
                best_score = score;
                best_node = (*connection).to_string();
            }
            if best_node == current && i == connections.len() && !connection_open {
                best_node = (*connection).to_string();
                best_score = score;
            }
        }

        if best_node != current {
            println!("Moving to {best_node}");
            current = best_node;
        } else {
            println!("Opening {best_node}");
            volcano.open_valve((*best_node).to_string());
            volcano.update_release_rate((*best_node).to_string())
        }
        volcano.update_total_released();
    }
    println!("Total Pressure Released: {}", volcano.total_released);
}

// Answer:

fn part2() {
    println!("Day X Part X");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
}

// Answer:

fn main() {
    part1();
    part2();
}
