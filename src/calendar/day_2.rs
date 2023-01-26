use std::fs;

use crate::Config;

pub fn run(config: Config) {
    println!("Day {}", config.day_number);
    println!("Answer for part 1: {}", part_1(&config));
    println!("Answer for part 2: {}", part_2(&config));
}

#[derive(Debug, PartialEq)]
enum PlayerMove {
    Rock,
    Paper,
    Scissors,
}

impl PlayerMove {
    fn from(s: &str) -> PlayerMove {
        match s {
            "A" => PlayerMove::Rock,
            "B" => PlayerMove::Paper,
            "C" => PlayerMove::Scissors,
            "X" => PlayerMove::Rock,
            "Y" => PlayerMove::Paper,
            "Z" => PlayerMove::Scissors,
            _ => panic!("unable to parse game move")
        }
    }

    fn move_score(&self) -> u32 {
        match self {
            PlayerMove::Rock => 1,
            PlayerMove::Paper => 2,
            PlayerMove::Scissors => 3,
        }
    }
    
    fn play_round(&self, p2: &PlayerMove) -> u32 {
        let round_result = if *self == *p2 {
            RoundResult::Draw
        } else if *self == PlayerMove::Rock && *p2 == PlayerMove::Scissors {
            RoundResult::Win
        } else if *self == PlayerMove::Paper && *p2 == PlayerMove::Rock {
            RoundResult::Win
        } else if *self == PlayerMove::Scissors && *p2 == PlayerMove::Paper {
            RoundResult::Win
        } else {
            RoundResult::Lose
        };
        self.move_score() + round_result.round_score()
    }
}

#[derive(Debug, PartialEq)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl RoundResult {
    fn from(s: &str) -> RoundResult {
        match s {
            "X" => RoundResult::Lose,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => panic!("unable to parse game move")
        }
    }
    
    fn round_score(&self) -> u32 {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
    
    fn play_round(&self, p: &PlayerMove) -> u32 {
        let player_move = match self {
            RoundResult::Draw => p,
            RoundResult::Win => match p {
                PlayerMove::Paper => &PlayerMove::Scissors,
                PlayerMove::Rock => &PlayerMove::Paper,
                PlayerMove::Scissors => &PlayerMove::Rock,
            },
            RoundResult::Lose => match p {
                PlayerMove::Paper => &PlayerMove::Rock,
                PlayerMove::Rock => &PlayerMove::Scissors,
                PlayerMove::Scissors => &PlayerMove::Paper,
            },
        };
        self.round_score() + player_move.move_score()
    }
}

fn part_1(config: &Config) -> u32 {
    let puzzle_input = fs::read_to_string(&config.input_path).unwrap();
    let puzzle_input = puzzle_input.trim();

    strategy_guide(puzzle_input)
}

fn strategy_guide(puzzle_input: &str) -> u32 {
    let mut rounds: Vec<(PlayerMove, PlayerMove)> = vec![];
    for line in puzzle_input.lines() {
        let v: Vec<&str> = line.split(' ').collect();
        rounds.push((PlayerMove::from(v[0]), PlayerMove::from(v[1])));
    }
    rounds.iter().map(|(p1, p2)| p2.play_round(p1)).sum()
}

fn part_2(config: &Config) -> u32 {
    let puzzle_input = fs::read_to_string(&config.input_path).unwrap();
    let puzzle_input = puzzle_input.trim();

    strategy_guide_part_2(puzzle_input)
}

fn strategy_guide_part_2(puzzle_input: &str) -> u32 {
    let mut rounds: Vec<(PlayerMove, RoundResult)> = vec![];
    for line in puzzle_input.lines() {
        let v: Vec<&str> = line.split(' ').collect();
        rounds.push((PlayerMove::from(v[0]), RoundResult::from(v[1])));
    }
    rounds.iter().map(|(p1, r)| r.play_round(p1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_guide_test() {
        let puzzle_input = "\
A Y
B X
C Z";

        assert_eq!(15, strategy_guide(puzzle_input));
    }

    #[test]
    fn strategy_guide_test_part_2() {
        let puzzle_input = "\
A Y
B X
C Z";

        assert_eq!(12, strategy_guide_part_2(puzzle_input));
    }
}