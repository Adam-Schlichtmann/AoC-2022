use std::fs;

pub fn part2() {
    println!("Day 1 Part 2");
    let file_path = "./src/day01/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let elves: Vec<&str> = contents.split("\n\n").collect();
    let elves_iter = elves.iter();
    let mut elves_mapped_to_nums = elves_iter
        .map(|elf| {
            let foods: Vec<&str> = elf.split("\n").collect();
            let foods_iter = foods.iter();
            let parsed_foods: Vec<i32> = foods_iter
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            let total = parsed_foods.iter().sum();
            println!("Foods:\n{total}",);
            return total;
        })
        .collect::<Vec<i32>>();
    elves_mapped_to_nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("Sorted Totals:\n{:?}", elves_mapped_to_nums);
    let top_three_total = elves_mapped_to_nums[elves_mapped_to_nums.len() - 1]
        + elves_mapped_to_nums[elves_mapped_to_nums.len() - 2]
        + elves_mapped_to_nums[elves_mapped_to_nums.len() - 3];
    println!("Top 3 Total: {top_three_total}")
}
// Answer: 210367
