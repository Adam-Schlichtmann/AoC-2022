use std::collections::VecDeque;
use std::fs;

struct Coords {
    x: usize,
    y: usize,
    distance: i32,
    symbol: char,
    letter: char,
    visited: bool,
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
fn get_path_char(
    current_x: usize,
    current_y: usize,
    destination_x: usize,
    destination_y: usize,
) -> char {
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

fn part1() {
    println!("Day 12 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let num_lines = lines.len();
    let mut queue: VecDeque<&Coords> = VecDeque::new();

    let mut map: Vec<Vec<&Coords>> = Vec::new();
    for y in 0..num_lines {
        map.push(Vec::new());
        for (x, c) in lines[y].chars().enumerate() {
            if c == 'S' {
                let start = Coords {
                    x: x,
                    y: y,
                    distance: 0,
                    letter: c,
                    symbol: 'S',
                    visited: false,
                };
                queue.push_back(&start);
                map[y].push(&start);
            } else {
                let node = Coords {
                    x: x,
                    y: y,
                    distance: 10000,
                    symbol: '.',
                    letter: c,
                    visited: false,
                };
                map[y].push(&node);
            }
        }
    }

    // for y in 0..map.len() {
    //     for x in 0..map[y].len() {
    //         print!("{}", map[y][x]);
    //     }
    //     print!("\n",);
    // }

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        println!(
            "STARTING ITERATION FOR {}, {}, NEW QUEUE: {}",
            current.x,
            current.y,
            queue.len()
        );
        let mut possible_nodes: VecDeque<Coords> = VecDeque::new();
        //   Down
        if current.y > 0
            && !map[current.y - 1][current.x].visited
            && is_within_one_letter(
                map[current.y][current.x].letter,
                map[current.y - 1][current.x].letter,
            )
        {
            possible_nodes.push_back(map[current.y - 1][current.x]);
        }
        // Up
        if current.y < map.len() - 1
            && !map[current.y + 1][current.x].visited
            && is_within_one_letter(
                map[current.y][current.x].letter,
                map[current.y + 1][current.x].letter,
            )
        {
            possible_nodes.push_back(map[current.y + 1][current.x]);
        }
        // Right
        if current.x < map[current.y].len() - 1
            && !map[current.y][current.x + 1].visited
            && is_within_one_letter(
                map[current.y][current.x].letter,
                map[current.y][current.x + 1].letter,
            )
        {
            possible_nodes.push_back(map[current.y][current.x + 1]);
        }
        // Left
        if current.x > 0
            && !map[current.y][current.x - 1].visited
            && is_within_one_letter(
                map[current.y][current.x].letter,
                map[current.y][current.x - 1].letter,
            )
        {
            possible_nodes.push_back(map[current.y][current.x - 1]);
        }

        for i in 0..possible_nodes.len() {
            // Check if neighbor
            let neighbor = possible_nodes.pop_front().unwrap();
            queue.push_back(&map[neighbor.y][neighbor.x]);
            if neighbor.distance > current.distance + 1 {
                map[neighbor.y][neighbor.x].distance = current.distance + 1;
                map[neighbor.y][neighbor.x].symbol =
                    get_path_char(current.x, current.y, neighbor.x, neighbor.y);
            }
        }
        map[current.y][current.x].visited = true;
        queue
            .make_contiguous()
            .sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x].letter);
        }
        print!("\n",);
    }
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x].symbol);
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
