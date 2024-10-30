use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::graph::{Graph, NodeIndex};
use std::fs;

struct Coords {
    x: i32,
    y: i32,
    distance: i32,
    symbol: char,
}

impl PartialEq for Coords {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn parse_char(c: char) -> i32 {
    return c as i32;
}

fn get_char(c: char) -> char {
    if c == 'S' {
        return 'a';
    }
    if c == 'E' {
        return 'z';
    }
    return c;
}

fn is_within_one_letter(current: char, next: char) -> bool {
    if current == 'z' && next == 'E' {
        return true;
    }
    if current == 'S' && next == 'a' {
        return true;
    }
    let c = parse_char(get_char(current));
    let n = parse_char(get_char(next));
    return c - n >= -1 && c - n <= 1;
}

// Gets path char for the destination
fn get_path_char(current_x: i32, current_y: i32, destination_x: i32, destination_y: i32) -> char {
    if current_x != destination_x {
        if current_x > destination_x {
            return '<';
        } else {
            return '>';
        }
    }
    if current_y != destination_y {
        if current_y > destination_y {
            return '^';
        } else {
            return 'v';
        }
    }
    panic!("NO CHAR")
}

fn to_node_id(x: usize, y: usize, width: usize) -> i32 {
    return (y * width + x) as i32;
}

fn part1() {
    println!("Day 12 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let num_lines = lines.len();

    let mut map: Vec<Vec<char>> = Vec::new();

    for y in 0..num_lines {
        map.push(Vec::new());

        for (x, c) in lines[y].chars().enumerate() {
            map[y].push(c);
        }
    }

    // Create an undirected graph with `i32` nodes and edges with `()` associated data.
    // let edges: Vec<(i32, i32)> = Vec::new();

    let mut g = Graph::<i32, i32>::new();
    // for y in 0..map.len() {
    //     for x in 0..map[y].len() {
    //         if y > 0 && is_within_one_letter(map[y][x], map[y - 1][x]) {
    //             edges.push((
    //                 to_node_id(y, x, map[y].len()),
    //                 to_node_id(y - 1, x, map[y].len()),
    //             ));
    //         }
    //         // Up
    //         if y < map.len() - 1 && is_within_one_letter(map[y][x], map[y + 1][x]) {
    //             edges.push((
    //                 to_node_id(y, x, map[y].len()),
    //                 to_node_id(y + 1, x, map[y].len()),
    //             ));
    //         }
    //         // Right
    //         if x < map[y].len() - 1 && is_within_one_letter(map[y][x], map[y][x + 1]) {
    //             edges.push((
    //                 to_node_id(y, x, map[y].len()),
    //                 to_node_id(y, x + 1, map[y].len()),
    //             ));
    //         }
    //         // Left
    //         if x > 0 && is_within_one_letter(map[y][x], map[y][x - 1]) {
    //             edges.push((
    //                 to_node_id(y, x, map[y].len()),
    //                 to_node_id(y, x - 1, map[y].len()),
    //             ));
    //         }
    //     }
    // }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if y > 0 && is_within_one_letter(map[y][x], map[y - 1][x]) {
                edges.push((
                    to_node_id(y, x, map[y].len()),
                    to_node_id(y - 1, x, map[y].len()),
                ));
            }
            // Up
            if y < map.len() - 1 && is_within_one_letter(map[y][x], map[y + 1][x]) {
                edges.push((
                    to_node_id(y, x, map[y].len()),
                    to_node_id(y + 1, x, map[y].len()),
                ));
            }
            // Right
            if x < map[y].len() - 1 && is_within_one_letter(map[y][x], map[y][x + 1]) {
                edges.push((
                    to_node_id(y, x, map[y].len()),
                    to_node_id(y, x + 1, map[y].len()),
                ));
            }
            // Left
            if x > 0 && is_within_one_letter(map[y][x], map[y][x - 1]) {
                edges.push((
                    to_node_id(y, x, map[y].len()),
                    to_node_id(y, x - 1, map[y].len()),
                ));
            }
        }
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        print!("\n",);
    }
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
