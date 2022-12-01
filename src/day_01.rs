use aoc_runner_derive::{aoc, aoc_generator};

use crate::input_parser::{parse_delimited_vec, parse_line_vec};
use std::collections::BinaryHeap;
use std::num::ParseIntError;
use std::str::FromStr;

type Cal = u32;
struct Elf {
    cals: Vec<Cal>,
}

impl FromStr for Elf {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Elf {
            cals: parse_line_vec(s)?,
        })
    }
}

type Parsed = Vec<Elf>;
type Input = [Elf];
type Output = Cal;

#[aoc_generator(day1)]
fn parse_input(data: &str) -> Parsed {
    parse_delimited_vec(data, "\n\n").expect("Failed to parse input")
}

#[aoc(day1, part1)]
fn part1(data: &Input) -> Output {
    data.iter()
        .map(|elf| elf.cals.iter().sum())
        .max()
        .unwrap_or(0)
}

#[aoc(day1, part2, Naive)]
fn part2_naive(data: &Input) -> Output {
    let mut top_elves = data
        .iter()
        .map(|elf| elf.cals.iter().sum())
        .collect::<Vec<_>>();
    top_elves.sort();

    top_elves.iter().rev().take(3).sum()
}

#[aoc(day1, part2, Faster)]
fn part2(data: &Input) -> Output {
    let top_elves = data
        .iter()
        .map(|elf| elf.cals.iter().sum())
        .collect::<BinaryHeap<_>>();

    top_elves.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 24000;
    const EXAMPLE_SOLUTION_PART2: Output = 45000;
    const SOLUTION_PART1: Output = 69626;
    const SOLUTION_PART2: Output = 206780;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day1.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day1.txt"))
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
    fn test_part2_naive_given_example_input() {
        assert_eq!(part2_naive(&example_input()), EXAMPLE_SOLUTION_PART2)
    }
    #[test]
    fn test_part2_naive() {
        assert_eq!(part2_naive(&input()), SOLUTION_PART2)
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
