use std::fs;

fn parse_board(rows: Vec<&str>) -> Vec<Vec<char>> {
    let mut board = Vec::new();

    for _x in 0..9 {
        board.push(Vec::new());
    }

    for row in rows {
        let mut i = 0;
        for c in row.chars() {
            if c != '[' && c != ']' && c != ' ' {
                let j = i / 4;

                board[j].insert(0, c);
            }
            i += 1
        }
    }
    println!("{:?}", board);
    return board;
}

fn part1() {
    println!("Day 5 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let start_state: Vec<&str> = contents.split("\n").filter(|s| s.contains("[")).collect();
    let mut board: Vec<Vec<char>> = parse_board(start_state);

    let moves: Vec<&str> = contents
        .split("\n")
        .filter(|s| s.starts_with("move"))
        .collect();

    for m in moves {
        let parts: Vec<&str> = m.split(" ").collect();
        let amount = parts[1].parse::<usize>().unwrap();
        let start = parts[3].parse::<usize>().unwrap();
        let destination = parts[5].parse::<usize>().unwrap();

        for _n in 0..amount {
            let c: char = board[start - 1].pop().unwrap();
            // println!("MOVING {c} from {start} to {destination}");
            board[destination - 1].push(c)
        }
    }
    println!("{:?}", board);

    for col in board {
        let c: char = col[col.len() - 1];
        print!("{c}"); // Answer: HNSNMTLHQ
    }
    print!("\n")
}

fn part2() {
    println!("Day 5 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let start_state: Vec<&str> = contents.split("\n").filter(|s| s.contains("[")).collect();
    let mut board: Vec<Vec<char>> = parse_board(start_state);

    let moves: Vec<&str> = contents
        .split("\n")
        .filter(|s| s.starts_with("move"))
        .collect();

    for m in moves {
        let parts: Vec<&str> = m.split(" ").collect();
        let amount = parts[1].parse::<usize>().unwrap();
        let start = parts[3].parse::<usize>().unwrap();
        let destination = parts[5].parse::<usize>().unwrap();
        let split = board[start - 1].len() - amount;
        let moved_chunk = board[start - 1].split_off(split);
        // println!(
        //     "MOVING CHUNK {:?} from {start} to {destination}",
        //     moved_chunk
        // );

        for cr in moved_chunk {
            board[destination - 1].push(cr);
        }
    }
    println!("{:?}", board);

    for col in board {
        let c: char = col[col.len() - 1];
        print!("{c}"); // Answer: RNLFDJMCT
    }
    print!("\n")
}

fn main() {
    part1();
    part2();
}
