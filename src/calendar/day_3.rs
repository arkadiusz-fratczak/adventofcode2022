use std::collections::HashSet;
use std::fs;

use crate::Config;

pub fn run(config: Config) {
    println!("Day {}", config.day_number);
    println!("Answer for part 1: {}", part_1(&config));
    println!("Answer for part 2: {}", part_2(&config));
}

fn part_1(config: &Config) -> u32 {
    let puzzle_input = fs::read_to_string(&config.input_path).unwrap();
    let puzzle_input = puzzle_input.trim();

    rucksack_reorganization(puzzle_input)
}

fn rucksack_reorganization(puzzle_input: &str) -> u32 {
    puzzle_input.lines().map(rearrangement).sum()
}

fn rearrangement(rucksack: &str) -> u32 {
    let priority_vec: Vec<u8> = rucksack.chars().map(priority_fn).collect();
    let half_size = priority_vec.len() / 2;
    let first_compartment: HashSet<u8> =
        HashSet::from_iter(priority_vec[..half_size].iter().copied());
    let second_compartment: HashSet<u8> =
        HashSet::from_iter(priority_vec[half_size..].iter().copied());

    first_compartment.intersection(&second_compartment).map(|&i| i as u32).sum()
}

fn priority_fn(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        (c as u8) - 96u8
    } else if c.is_ascii_uppercase() {
        (c as u8) - 38u8
    } else {
        0
    }
}

fn part_2(config: &Config) -> u32 {
    let puzzle_input = fs::read_to_string(&config.input_path).unwrap();
    let puzzle_input = puzzle_input.trim();

    badges(puzzle_input)
}

fn badges(puzzle_input: &str) -> u32 {
    let all_lines: Vec<&str> = puzzle_input.lines().collect();
    all_lines.chunks(3)
        .map(|c| c.iter().map(|&s| HashSet::from_iter(s.chars())).collect::<Vec<_>>())
        .map(|v| v.iter()
            .map(|h| h.to_owned())
            .reduce(|h1, h2| h1.intersection(&h2).copied().collect::<HashSet<_>>())
            .unwrap_or(HashSet::new())
            .iter().next().unwrap().to_owned())
        .map(|ch| priority_fn(ch) as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksacks_part_1_test() {
        let puzzle_input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, rucksack_reorganization(puzzle_input));
    }

    #[test]
    fn rucksacks_part_2_test() {
        let puzzle_input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(70, badges(puzzle_input));
    }
}
