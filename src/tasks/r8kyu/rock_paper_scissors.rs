use std::cmp::{Ordering, PartialEq, PartialOrd};

#[derive(Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Lose,
    Draw
}

fn str_to_rock_paper_scissors(str: &str) -> RockPaperScissors {
    if str.to_ascii_lowercase() == "rock" {
        RockPaperScissors::Rock
    } else if str.to_ascii_lowercase() == "paper" {
        RockPaperScissors::Paper
    } else {
        RockPaperScissors::Scissors
    }
}

impl PartialEq for RockPaperScissors {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RockPaperScissors::Rock => {
                match other {
                    RockPaperScissors::Rock => true,
                    RockPaperScissors::Paper => false,
                    RockPaperScissors::Scissors => false,
                }
            },
            RockPaperScissors::Paper => {
                match other {
                    RockPaperScissors::Paper => true,
                    RockPaperScissors::Rock => false,
                    RockPaperScissors::Scissors => false
                }
            }
            RockPaperScissors::Scissors => {
                match other {
                    RockPaperScissors::Rock => false,
                    RockPaperScissors::Paper => false,
                    RockPaperScissors::Scissors => true
                }
            }
        }
    }
}

impl PartialOrd for RockPaperScissors {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            RockPaperScissors::Rock => {
                match other {
                    RockPaperScissors::Rock => Some(Ordering::Equal),
                    RockPaperScissors::Paper => Some(Ordering::Less),
                    RockPaperScissors::Scissors => Some(Ordering::Greater),
                }
            }
            RockPaperScissors::Paper => {
                match other {
                    RockPaperScissors::Paper => Some(Ordering::Equal),
                    RockPaperScissors::Scissors => Some(Ordering::Less),
                    RockPaperScissors::Rock => Some(Ordering::Greater),
                }
            }
            RockPaperScissors::Scissors => {
                match other {
                    RockPaperScissors::Scissors => Some(Ordering::Equal),
                    RockPaperScissors::Rock => Some(Ordering::Less),
                    RockPaperScissors::Paper => Some(Ordering::Greater),
                }
            }
        }
    }
}

fn compare(rock_paper_scissors_1: RockPaperScissors, rock_paper_scissors_2: RockPaperScissors) -> GameResult {
    if rock_paper_scissors_1 == rock_paper_scissors_2 {
        GameResult::Draw
    } else if rock_paper_scissors_1 > rock_paper_scissors_2 {
        GameResult::Win
    } else {
        GameResult::Lose
    }
}

pub fn rps (p1: &str, p2: &str) -> &'static str {
    let p1 = str_to_rock_paper_scissors(p1);
    let p2 = str_to_rock_paper_scissors(p2);
    match compare(p1, p2) {
        GameResult::Win => {
            "Player 1 won!"
        }
        GameResult::Lose => {
            "Player 2 won!"
        }
        GameResult::Draw => {
            "Draw!"
        }
    }
}