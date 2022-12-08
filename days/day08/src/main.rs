use std::fs;

fn parse_char(c: char) -> i32 {
    return (c.to_string()).parse::<i32>().unwrap();
}

fn part1() {
    println!("Day 8 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let rows: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for row in rows {
        grid.push(row.chars().collect());
    }

    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            let tree = parse_char(*item);
            if i == 0 || j == 0 || i == grid.len() - 1 || j == row.len() - 1 {
                // If edge tree
                count += 1;
            } else if (&row[..j]).iter().all(|x| parse_char(*x) < tree) {
                // Check left
                count += 1;
            } else if (&row[(j + 1)..]).iter().all(|x| parse_char(*x) < tree) {
                // Check right
                count += 1;
            } else {
                let mut visible: bool = true;
                // Check top
                for k in 0..i {
                    if parse_char(grid[k][j]) >= tree {
                        visible = false;
                    }
                }
                if visible {
                    count += 1
                } else {
                    visible = true;
                    // Check bottom
                    for k in (i + 1)..grid.len() {
                        if parse_char(grid[k][j]) >= tree {
                            visible = false;
                        }
                    }
                    if visible {
                        count += 1
                    }
                }
            }
        }
    }
    println!("Number of visible trees: {count}"); // Answer: 1538
}

fn count_trees(row: Vec<char>, tree: i32) -> i32 {
    let mut count = 0;
    for r in row {
        let compare_tree = parse_char(r);
        if compare_tree < tree {
            count += 1;
        } else {
            count += 1;
            break;
        }
    }
    return count;
}

fn part2() {
    println!("Day 8 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let rows: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for row in rows {
        grid.push(row.chars().collect());
    }

    let mut best_score = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            let tree = parse_char(*item);
            let left = count_trees(
                (0..j).map(|k| grid[i][k]).rev().collect::<Vec<char>>(),
                tree,
            );
            let right = count_trees(row[(j + 1)..].to_vec(), tree);
            let top = count_trees((0..i).map(|k| grid[k][j]).rev().collect(), tree);
            let bottom = count_trees(((i + 1)..grid.len()).map(|k| grid[k][j]).collect(), tree);
            let score = left * right * top * bottom;

            if score > best_score {
                best_score = score;
            }
        }
    }
    println!("Number tree score: {best_score}") // Answer: 496125
}

fn main() {
    part1();
    part2();
}
