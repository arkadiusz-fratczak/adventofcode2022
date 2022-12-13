use std::fs;

use crate::Config;

pub fn run(config: Config) {
    println!("Day {}", config.day_number);
    println!("Answer for part 1: {}", part_1(&config));
    println!("Answer for part 2: {}", part_2(&config));
}

fn part_1(config: &Config) -> i32 {
    let puzzle_input = fs::read_to_string(&config.input_path).unwrap();
    let puzzle_input = puzzle_input.trim();

    top_elf(puzzle_input)
}

fn top_elf(puzzle_input: &str) -> i32 {
    let total_calories_per_elf = total_calories_carrying_by_elves(puzzle_input);

    let max_value = total_calories_per_elf.iter().max().unwrap();
    *max_value
}

fn part_2(config: &Config) -> i32 {
    let puzzle_input = fs::read_to_string(&config.input_path).unwrap();
    let puzzle_input = puzzle_input.trim();

    top_three_elves(puzzle_input)
}

fn top_three_elves(puzzle_input: &str) -> i32 {
    let mut total_calories_per_elf = total_calories_carrying_by_elves(puzzle_input);

    total_calories_per_elf.sort();
    let top_three = total_calories_per_elf.iter().rev().take(3).sum();
    top_three
}

fn total_calories_carrying_by_elves(puzzle_input: &str) -> Vec<i32> {
    let mut total_calories_carrying: Vec<i32> = Vec::new();
    let mut current_total_for_elf = 0;

    for line in puzzle_input.lines() {
        if line.is_empty() {
            total_calories_carrying.push(current_total_for_elf);
            current_total_for_elf = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            current_total_for_elf += calories;
        }
    }
    total_calories_carrying
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_top_elf() {
        let puzzle_input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(24000, top_elf(puzzle_input));
    }

    #[test]
    fn find_sum_of_top_three_elves() {
        let puzzle_input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(45000, top_three_elves(puzzle_input));
    }
}
