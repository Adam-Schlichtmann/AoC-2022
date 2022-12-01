use std::env;
mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args:\n{:?}", args);
    if args[1].parse::<i32>().unwrap() == 1 {
      if  args[2].parse::<i32>().unwrap() == 1 {
        day01::part1::part1();

      } else if args[2].parse::<i32>().unwrap() == 2 {
        day01::part2::part2();
      }
    }
    dbg!(args);
}