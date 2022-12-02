use aoc_runner_derive::{aoc, aoc_generator};

use std::fmt::Debug;
use std::str::FromStr;

const DRAW: Output = 3;
const LOSS: Output = 0;
const WIN: Output = 6;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Sicssor,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Sicssor),
            other => Err(format!("Unexpected move: {other}")),
        }
    }
}

impl Move {
    fn value(self) -> Output {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Sicssor => 3,
        }
    }
}

trait Round {
    fn move_value(self) -> Output;
    fn outcome_value(self) -> Output;
}

#[derive(Debug, Clone, Copy)]
struct RoundPart1 {
    them: Move,
    me: Move,
}

impl FromStr for RoundPart1 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (them, me) = s.split_once(' ').ok_or(format!("Unexpected round: {s}"))?;
        Ok(Self {
            them: them.parse()?,
            me: me.parse()?,
        })
    }
}

impl Round for RoundPart1 {
    fn outcome_value(self) -> Output {
        match self.me {
            Move::Rock => match self.them {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Loss,
                Move::Sicssor => Outcome::Win,
            },
            Move::Paper => match self.them {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Draw,
                Move::Sicssor => Outcome::Loss,
            },
            Move::Sicssor => match self.them {
                Move::Rock => Outcome::Loss,
                Move::Paper => Outcome::Win,
                Move::Sicssor => Outcome::Draw,
            },
        }
        .value()
    }
    fn move_value(self) -> Output {
        self.me.value()
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            other => Err(format!("Unexpected outcome: {other}")),
        }
    }
}

impl Outcome {
    fn value(self) -> Output {
        match self {
            Self::Loss => LOSS,
            Self::Draw => DRAW,
            Self::Win => WIN,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RoundPart2 {
    them: Move,
    me: Outcome,
}

impl FromStr for RoundPart2 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (them, me) = s.split_once(' ').ok_or(format!("Unexpected round: {s}"))?;
        Ok(Self {
            them: them.parse()?,
            me: me.parse()?,
        })
    }
}

impl Round for RoundPart2 {
    fn move_value(self) -> Output {
        match self.me {
            Outcome::Loss => match self.them {
                Move::Rock => Move::Sicssor,
                Move::Paper => Move::Rock,
                Move::Sicssor => Move::Paper,
            },
            Outcome::Draw => self.them,
            Outcome::Win => match self.them {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Sicssor,
                Move::Sicssor => Move::Rock,
            },
        }
        .value()
    }
    fn outcome_value(self) -> Output {
        self.me.value()
    }
}

fn calculate_game_score<T>(data: &Input) -> Output
where
    T: Round + FromStr + Copy,
    <T as FromStr>::Err: Debug,
{
    data.iter()
        .map(|line| line.parse::<T>().unwrap())
        .map(|round| round.move_value() + round.outcome_value())
        .sum()
}

type Parsed = Vec<String>;
type Input = [String];
type Output = u64;

#[aoc_generator(day2)]
fn parse_input(data: &str) -> Parsed {
    data.lines().map(|l| l.to_string()).collect()
}

#[aoc(day2, part1)]
fn part1(data: &Input) -> Output {
    calculate_game_score::<RoundPart1>(data)
}

#[aoc(day2, part2)]
fn part2(data: &Input) -> Output {
    calculate_game_score::<RoundPart2>(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 15;
    const SOLUTION_PART1: Output = 11841;
    const EXAMPLE_SOLUTION_PART2: Output = 12;
    const SOLUTION_PART2: Output = 13022;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day2.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day2.txt"))
    }
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), EXAMPLE_SOLUTION_PART1)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), SOLUTION_PART1)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), EXAMPLE_SOLUTION_PART2)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), SOLUTION_PART2)
    }
}
