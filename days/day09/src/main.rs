use std::collections::HashSet;
use std::fs;
use std::{thread, time};

fn parse_move(mv: &str) -> Vec<&str> {
    let parts: Vec<&str> = mv.split(" ").collect();
    return parts;
}

fn move_head_y(current_y: i32, direction: &str) -> i32 {
    if direction == "U" {
        return current_y + 1;
    }
    if direction == "D" {
        return current_y - 1;
    }
    return current_y;
}

fn move_head_x(current_x: i32, direction: &str) -> i32 {
    if direction == "R" {
        return current_x + 1;
    }
    if direction == "L" {
        return current_x - 1;
    }
    return current_x;
}

fn move_tail(head: i32, tail: i32) -> i32 {
    if is_dimension_safe(head, tail) {
        return tail;
    }

    if head > tail {
        return tail + 1;
    }

    return tail - 1;
}

fn is_dimension_safe(head: i32, tail: i32) -> bool {
    return head - tail >= -1 && head - tail <= 1;
}

fn is_tail_safe(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> bool {
    return is_dimension_safe(head_x, tail_x) && is_dimension_safe(head_y, tail_y);
}

fn part1() {
    println!("Day 9 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let moves: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let mut head_x = 0;
    let mut head_y = 0;
    let mut tail_x = 0;
    let mut tail_y = 0;
    let mut tail_locations: HashSet<String> = HashSet::new();
    tail_locations.insert(String::from("0x0"));
    for mv in moves {
        let parts = parse_move(mv);
        let direction = parts[0];
        let distance = parts[1].parse::<i32>().unwrap();
        for _d in 0..distance {
            head_x = move_head_x(head_x, direction);
            head_y = move_head_y(head_y, direction);
            if !is_tail_safe(head_x, head_y, tail_x, tail_y) {
                if head_x != tail_x && head_y != tail_y {
                    // DIAGONAL MOVE BOTH DIMENSIONS MUST STEP
                    if head_x > tail_x {
                        tail_x = tail_x + 1;
                    } else {
                        tail_x = tail_x - 1;
                    }
                    if head_y > tail_y {
                        tail_y = tail_y + 1;
                    } else {
                        tail_y = tail_y - 1;
                    }
                } else {
                    // Vertical or Horizontal move
                    tail_x = move_tail(head_x, tail_x);
                    tail_y = move_tail(head_y, tail_y);
                }

                let key = format!("{tail_x}x{tail_y}");
                tail_locations.insert(key);
            }
        }
    }

    println!("TAIL LOCATIONS: {:?}", tail_locations.len()); // Answer: 6311
}

struct Coords {
    x: i32,
    y: i32,
}
const grid_size_x: i32 = 80;
const grid_size_y: i32 = 40;
fn build_output(snake: &[Coords]) {
    // Clear
    print!("{}[2J", 27 as char);
    let base_y = snake[0].y;
    let base_x = snake[0].x;
    for y in 0..grid_size_y {
        for x in 0..grid_size_x {
            match snake.iter().position(|seg| {
                seg.x == (x + base_x - (grid_size_x / 2))
                    && seg.y == (y + base_y - (grid_size_y / 2))
            }) {
                Some(0) => print!("H"),
                Some(_) => print!("T"),
                None => print!("."),
            }
        }
        print!("\n")
    }
}

fn part2() {
    println!("Day 9 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let moves: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let mut tails: Vec<Coords> = vec![
        Coords { x: 0, y: 0 },
        Coords { x: 0, y: 0 },
        Coords { x: 0, y: 0 },
        Coords { x: 0, y: 0 },
        Coords { x: 0, y: 0 },
        Coords { x: 0, y: 0 },
        Coords { x: 0, y: 0 },
        Coords { x: 0, y: 0 },
        Coords { x: 0, y: 0 },
        Coords { x: 0, y: 0 },
    ];
    let mut tail_locations: HashSet<String> = HashSet::new();
    tail_locations.insert(String::from("0x0"));
    for mv in moves {
        let parts = parse_move(mv);
        let direction = parts[0];
        let distance = parts[1].parse::<i32>().unwrap();
        for _d in 0..distance {
            tails[0].x = move_head_x(tails[0].x, direction);
            tails[0].y = move_head_y(tails[0].y, direction);

            for i in 1..tails.len() {
                if !is_tail_safe(tails[i - 1].x, tails[i - 1].y, tails[i].x, tails[i].y) {
                    if tails[i - 1].x != tails[i].x && tails[i - 1].y != tails[i].y {
                        // DIAGONAL MOVE BOTH DIMENSIONS MUST STEP
                        if tails[i - 1].x > tails[i].x {
                            tails[i].x = tails[i].x + 1;
                        } else {
                            tails[i].x = tails[i].x - 1;
                        }
                        if tails[i - 1].y > tails[i].y {
                            tails[i].y = tails[i].y + 1;
                        } else {
                            tails[i].y = tails[i].y - 1;
                        }
                    } else {
                        // Vertical or Horizontal move
                        tails[i].x = move_tail(tails[i - 1].x, tails[i].x);
                        tails[i].y = move_tail(tails[i - 1].y, tails[i].y);
                    }
                    // Only tracking the last piece of tail
                    if i == tails.len() - 1 {
                        let key = format!("{}x{}", tails[i].x, tails[i].y);
                        tail_locations.insert(key);
                    }
                    let delay = time::Duration::from_millis(100);

                    thread::sleep(delay);
                    build_output(tails.as_slice());
                }
            }
        }
    }
    println!("TAIL LOCATIONS: {:?}", tail_locations.len()); // Answer: 6311
}

fn main() {
    part1();
    part2();
}
