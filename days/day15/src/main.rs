use regex::Regex;
use std::collections::HashSet;
use std::fs;

struct Sensor {
    sens_x: i32,
    sens_y: i32,
    beac_x: i32,
    beac_y: i32,
    distance: i32,
}

const DESIRED_Y: i32 = 2000000;
fn part1() {
    println!("Day 15 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let mut sensors: Vec<Sensor> = Vec::new();
    let mut max_sens_x = 0;
    let mut min_sens_x = 99999999;
    let re = Regex::new(r"[^0-9-]").unwrap();
    for i in 0..lines.len() {
        let parts: Vec<&str> = lines[i]
            .split(" ")
            .filter(|x| x.contains("x=") || x.contains("y="))
            .collect();

        let cleaned: Vec<String> = parts
            .iter()
            .map(|x| re.replace_all(x, "").into())
            .collect::<Vec<String>>();

        let sens_x = cleaned[0].parse::<i32>().unwrap();
        let sens_y = cleaned[1].parse::<i32>().unwrap();
        let beac_x = cleaned[2].parse::<i32>().unwrap();
        let beac_y = cleaned[3].parse::<i32>().unwrap();
        let x_distance = (sens_x - beac_x).abs();
        let y_distance = (sens_y - beac_y).abs();
        if max_sens_x < (sens_x + x_distance + y_distance) {
            max_sens_x = sens_x + x_distance + y_distance;
        }
        if min_sens_x > (sens_x - x_distance - y_distance) {
            min_sens_x = sens_x - x_distance - y_distance;
        }
        sensors.push(Sensor {
            sens_x,
            sens_y,
            beac_x,
            beac_y,
            distance: x_distance + y_distance,
        });
    }

    let mut row: HashSet<i32> = HashSet::new();
    let mut count = 0;
    for i in 0..sensors.len() {
        // If sensor overlaps in the y value we care about
        let y_offset = (DESIRED_Y - sensors[i].sens_y).abs();
        if (sensors[i].sens_y + sensors[i].distance) >= DESIRED_Y
            && (sensors[i].sens_y - sensors[i].distance) <= DESIRED_Y
        {
            let start = sensors[i].sens_x - sensors[i].distance + y_offset - min_sens_x;
            let end = sensors[i].sens_x + sensors[i].distance - y_offset - min_sens_x;
            for x in start..end {
                if !row.contains(&x) {
                    row.insert(x);
                    count += 1;
                }
            }
        }
    }

    println!("Unavailable locations in row: {DESIRED_Y}: {count}"); // Answer: 5335787
}

const MAX_Y: i32 = 4000000;
// const MAX_Y: i32 = 20;
fn part2() {
    println!("Day 15 Part 2");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let mut sensors: Vec<Sensor> = Vec::new();

    let re = Regex::new(r"[^0-9-]").unwrap();
    for i in 0..lines.len() {
        let parts: Vec<&str> = lines[i]
            .split(" ")
            .filter(|x| x.contains("x=") || x.contains("y="))
            .collect();

        let cleaned: Vec<String> = parts
            .iter()
            .map(|x| re.replace_all(x, "").into())
            .collect::<Vec<String>>();

        let sens_x = cleaned[0].parse::<i32>().unwrap();
        let sens_y = cleaned[1].parse::<i32>().unwrap();
        let beac_x = cleaned[2].parse::<i32>().unwrap();
        let beac_y = cleaned[3].parse::<i32>().unwrap();
        let x_distance = (sens_x - beac_x).abs();
        let y_distance = (sens_y - beac_y).abs();

        sensors.push(Sensor {
            sens_x,
            sens_y,
            beac_x,
            beac_y,
            distance: x_distance + y_distance,
        });
    }
    let rotations = [(-1, 1), (1, -1), (-1, -1), (1, 1)];

    for i in 0..sensors.len() {
        println!("Sensor: {i}");

        // Iterate over every possibility that is one tile out of the distance for the sensor
        for dx in 0..(sensors[i].distance + 2) {
            let dy = sensors[i].distance + 1 - dx;
            // Check each rotation around the sensor
            for r in 0..rotations.len() {
                let x = sensors[i].sens_x + (dx * rotations[r].0);
                let y = sensors[i].sens_y + (dy * rotations[r].1);

                // Is in the valid range
                if x <= MAX_Y && x >= 0 && y <= MAX_Y && y >= 0 {
                    // Is out of range of all sensors
                    if sensors
                        .iter()
                        .all(|sen| (sen.sens_x - x).abs() + (sen.sens_y - y).abs() > sen.distance)
                    {
                        println!("FOUND {},{}", x, y);
                        println!("ANSWER: {}", (x) as i128 * (4000000) as i128 + (y) as i128);
                        // Answer: 13673971349056
                    }
                }
            }
        }
    }
}

fn main() {
    part1();
    part2();
}

// Print the board
// let mut matrix = [['.'; 20]; 20];
// for i in 0..sensors.len() {
//     println!("Sensor: {i} {} {}", sensors[i].beac_x, sensors[i].beac_y);
//     if sensors[i].sens_x < 20
//         && sensors[i].sens_y < 20
//         && sensors[i].sens_x >= 0
//         && sensors[i].sens_y >= 0
//     {
//         matrix[sensors[i].sens_y as usize][sensors[i].sens_x as usize] = 'S';
//     }

//     if sensors[i].beac_x < 20
//         && sensors[i].beac_y < 20
//         && sensors[i].beac_x >= 0
//         && sensors[i].beac_y >= 0
//     {
//         matrix[sensors[i].beac_y as usize][sensors[i].beac_x as usize] = 'B';
//     }
//     let mut sub = sensors[i].distance;
//     while sub >= 0 {
//         for dx in 0..sensors[i].distance + 1 - sub {
//             let dy = sensors[i].distance - sub - dx;
//             for r in 0..rotations.len() {
//                 let x = sensors[i].sens_x + (dx * rotations[r].0);
//                 let y = sensors[i].sens_y + (dy * rotations[r].1);
//                 println!("Setting {x},{y}");
//                 if x >= 0 && x < 20 && y >= 0 && y < 20 {
//                     if matrix[y as usize][x as usize] == '.' {
//                         matrix[y as usize][x as usize] = '#';
//                     }
//                 }
//             }
//         }
//         sub -= 1;
//     }
// }

// for y in 0..matrix.len() {
//     for x in 0..matrix[y].len() {
//         print!("{}", matrix[y][x]);
//     }
//     print!("\n");
// }
