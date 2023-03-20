use std::fs;

use crate::Config;

pub fn run(config: Config) {
    println!("Day {}", config.day_number);
    println!("Answer for part 1: {}", part_1(&config));
    println!("Answer for part 2: {}", part_2(&config));
}

struct Range {
    lower_bound: u8,
    upper_bound: u8,
}

impl Range {
    fn new(lower_bound: u8, upper_bound: u8) -> Range {
        Range {
            lower_bound,
            upper_bound,
        }
    }
    
    fn from(range_str: &str) -> Range {
        let bounds: Vec<&str> = range_str.split('-').collect();
        Range::new(bounds[0].parse().unwrap(), bounds[1].parse().unwrap())
    }
    
    fn fully_contain(&self, other_range: &Range) -> bool {
        self.lower_bound <= other_range.lower_bound && self.upper_bound >= other_range.upper_bound
    }
    
    fn overlaps(&self, other_range: &Range) -> bool {
        (self.lower_bound >= other_range.lower_bound && self.lower_bound <= other_range.upper_bound)
        || (other_range.lower_bound >= self.lower_bound && other_range.lower_bound <= self.upper_bound)
    }
}

fn part_1(config: &Config) -> u32 {
    let puzzle_input = fs::read_to_string(&config.input_path).unwrap();
    let puzzle_input = puzzle_input.trim();

    count_pairs_with_fully_contained_ranges(puzzle_input)
}

fn count_pairs_with_fully_contained_ranges(input: &str) -> u32 {
    let section_assignment_pairs: Vec<Vec<Range>> = 
        input.lines().map(|l| parse_line(l)).collect();
    section_assignment_pairs.iter()
        .filter(|&v| v[0].fully_contain(&v[1]) || v[1].fully_contain(&v[0]))
        .count() as u32
}

fn parse_line(line: &str) -> Vec<Range> {
    line.split(',').map(|r| Range::from(r)).collect()
}

fn part_2(config: &Config) -> u32 {
    let puzzle_input = fs::read_to_string(&config.input_path).unwrap();
    let puzzle_input = puzzle_input.trim();

    count_overlapping_pairs(puzzle_input)
}

fn count_overlapping_pairs(input: &str) -> u32 {
    let section_assignment_pairs: Vec<Vec<Range>> =
        input.lines().map(|l| parse_line(l)).collect();
    section_assignment_pairs.iter()
        .filter(|&v| v[0].overlaps(&v[1]))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camp_cleanup_part_1_test() {
        let puzzle_input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(2, count_pairs_with_fully_contained_ranges(puzzle_input));
    }

    #[test]
    fn camp_cleanup_part_2_test() {
        let puzzle_input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(4, count_overlapping_pairs(puzzle_input));
    }

}