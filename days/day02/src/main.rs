use std::fs;

fn convert_to_elf(mine: &str) -> &str {
    if mine == "A" || mine == "B" || mine == "C" {
        return mine;
    }
    if mine == "X" {
        return "A";
    }
    if mine == "Y" {
        return "B";
    }
    if mine == "Z" {
        return "C";
    }
    panic!("UNKNOWN OPERATOR #{mine}")
}

fn score_outcome(elf: &str, mine: &str) -> i32 {
    // Draw
    if elf == convert_to_elf(mine) {
        return 3;
    }
    // Mine win
    if (elf == "A" && convert_to_elf(mine) == "B")
        || (elf == "B" && convert_to_elf(mine) == "C")
        || (elf == "C" && convert_to_elf(mine) == "A")
    {
        return 6;
    }
    // Elf wins
    if (elf == "A" && convert_to_elf(mine) == "C")
        || (elf == "B" && convert_to_elf(mine) == "A")
        || (elf == "C" && convert_to_elf(mine) == "B")
    {
        return 0;
    }
    panic!("UNKNOWN OPERATOR #{mine} OR #{elf}")
}

fn score_shape(shape: &str) -> i32 {
    if shape == "A" || shape == "X" {
        return 1;
    }
    if shape == "B" || shape == "Y" {
        return 2;
    }
    if shape == "C" || shape == "Z" {
        return 3;
    }
    panic!("UNKNOWN OPERATOR #{shape}")
}

fn score_game(elf: &str, mine: &str) -> i32 {
    return score_outcome(elf, mine) + score_shape(mine);
}

fn part1() {
    println!("Day 2 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let games: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    // println!("LINES{:?}", games);
    let game_scores: Vec<i32> = games
        .iter()
        .map(|input| {
            let inputs: Vec<&str> = input.split(" ").collect();
            return score_game(inputs[0], inputs[1]);
        })
        .collect();
    let sum: i32 = game_scores.iter().sum();
    println!("MY SCORE: {sum}") // Answer: 15422
}

fn get_losing_shape(elf: &str) -> &str {
    // Rock
    if elf == "A" {
        return "C"; // Scissors
    }
    // Paper
    if elf == "B" {
        return "A"; // Rock
    }
    // Scissors
    if elf == "C" {
        return "B"; // Paper
    }
    panic!("UNKNOWN OPERATOR #{elf}")
}

fn get_winning_shape(elf: &str) -> &str {
    // Rock
    if elf == "A" {
        return "B"; // Paper
    }
    // Paper
    if elf == "B" {
        return "C"; // Scissors
    }
    // Scissors
    if elf == "C" {
        return "A"; // Rock
    }
    panic!("UNKNOWN OPERATOR #{elf}")
}

fn get_shape_to_play<'a>(elf: &'a str, outcome: &'a str) -> &'a str {
    if outcome == "X" {
        return get_losing_shape(elf);
    }
    if outcome == "Y" {
        return elf;
    }
    if outcome == "Z" {
        return get_winning_shape(elf);
    }
    panic!("UNKNOWN OPERATOR #{outcome}")
}

fn get_outcome(elf: &str, outcome: &str) -> i32 {
    let my_shape: &str = get_shape_to_play(elf, outcome);
    return score_game(elf, my_shape);
}
fn part2() {
    println!("Day 2 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let games: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();
    // println!("LINES{:?}", games);
    let game_scores: Vec<i32> = games
        .iter()
        .map(|input| {
            let inputs: Vec<&str> = input.split(" ").collect();
            return get_outcome(inputs[0], inputs[1]);
        })
        .collect();
    let sum: i32 = game_scores.iter().sum();
    println!("MY ACTUAL: {sum}") // Answer:
}

// Answer:

fn main() {
    part1();
    part2();
}
