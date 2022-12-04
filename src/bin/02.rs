use std::{error::Error, fmt::Display, str::FromStr};
use crate::Outcome::{Draw, Lose, Win};
use crate::Symbol::{Paper, Rock, Scissor};

#[derive(Debug)]
struct ParseMoveError {}

impl Error for ParseMoveError {}

const PARSE_MOVE_ERROR: &'static str = "invalid move key provided";

impl Display for ParseMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PARSE_MOVE_ERROR)
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Symbol {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Symbol {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissor),
            _ => Err(ParseMoveError {}),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}


impl FromStr for Outcome {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(ParseMoveError {}),
        }
    }
}

fn score_s(s: Symbol) -> u32 {
    return match s {
        Rock => 1,
        Paper => 2,
        Scissor => 3,
    };
}

fn score_d(d: Outcome) -> u32 {
    return match d {
        Win => 6,
        Draw => 3,
        Lose => 0,
    };
}

fn play(opp: Symbol, play: Symbol) -> Option<Outcome> {
    if opp == play {
        return Some(Draw);
    }

    if opp == Rock && play == Paper {
        return Some(Win);
    }

    if opp == Rock && play == Scissor {
        return Some(Lose);
    }

    if opp == Paper && play == Rock {
        return Some(Lose);
    }

    if opp == Paper && play == Scissor {
        return Some(Win);
    }

    if opp == Scissor && play == Rock {
        return Some(Win);
    }

    if opp == Scissor && play == Paper {
        return Some(Lose);
    }

    return None;
}


fn expect(opp: Symbol, desired: Outcome) -> Symbol {
    return match desired {
        Win => {
            match opp {
                Rock => Paper,
                Paper => Scissor,
                Scissor => Rock,
            }
        }
        Lose => {
            match opp {
                Rock => Scissor,
                Paper => Rock,
                Scissor => Paper,
            }
        }
        Draw => {
            match opp {
                Rock => Rock,
                Paper => Paper,
                Scissor => Scissor,
            }
        }
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines() {
        let (t, u) = line.split_once(' ').unwrap();
        let them = t.parse().unwrap();
        let us = u.parse().unwrap();

        match play(them, us).unwrap() {
            Win => result += score_s(us) + score_d(Win),
            Draw => result += score_s(us) + score_d(Draw),
            Lose => result += score_s(us) + score_d(Lose),
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines() {
        let (t, d) = line.split_once(' ').unwrap();
        let them: Symbol = t.parse().unwrap();
        let desired: Outcome = d.parse().unwrap();

        match expect(them, desired) {
            Rock => result += score_s(Rock) + score_d(desired),
            Paper => result += score_s(Paper) + score_d(desired),
            Scissor => result += score_s(Scissor) + score_d(desired),
        }
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
