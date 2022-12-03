use aoc_runner_derive::{aoc, aoc_generator};

use crate::input_parser::parse_line_vec;
use std::str::FromStr;

type Output = u32;

#[derive(Debug, Clone, Copy)]
struct LettersBits {
    bits: u64,
}

impl FromStr for LettersBits {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = 0;
        for c in s.chars() {
            if c.is_lowercase() {
                bits |= 1 << (1 + c as u8 - b'a');
            } else if c.is_uppercase() {
                bits |= 1 << (27 + c as u8 - b'A');
            } else {
                return Err(format!("Expected ascii letter, got {c}"));
            };
        }
        Ok(Self { bits })
    }
}

#[derive(Debug, Clone, Copy)]
struct Rucksack {
    first_compartment: LettersBits,
    second_compartment: LettersBits,
}

impl FromStr for Rucksack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mid = s.len() / 2;
        Ok(Self {
            first_compartment: LettersBits::from_str(&s[..mid])?,
            second_compartment: LettersBits::from_str(&s[mid..])?,
        })
    }
}

impl Rucksack {
    fn common_letter_value(self) -> u32 {
        (self.first_compartment.bits & self.second_compartment.bits).ilog2()
    }
}

#[derive(Debug, Clone, Copy)]
struct ElfGroup {
    elves: (LettersBits, LettersBits, LettersBits),
}

impl ElfGroup {
    fn common_letter_value(self) -> u32 {
        (self.elves.0.bits & self.elves.1.bits & self.elves.2.bits).ilog2()
    }
}

#[aoc_generator(day3, part1)]
fn parse_input_part1(data: &str) -> Vec<Rucksack> {
    parse_line_vec(data).unwrap()
}

#[aoc(day3, part1)]
fn part1(data: &[Rucksack]) -> Output {
    data.iter()
        .map(|rucksack| rucksack.common_letter_value())
        .sum()
}

#[aoc_generator(day3, part2)]
fn parse_input_part2(data: &str) -> Vec<ElfGroup> {
    data.lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|chunk| ElfGroup {
            elves: (
                LettersBits::from_str(chunk[0]).unwrap(),
                LettersBits::from_str(chunk[1]).unwrap(),
                LettersBits::from_str(chunk[2]).unwrap(),
            ),
        })
        .collect()
}

#[aoc(day3, part2)]
fn part2(data: &[ElfGroup]) -> Output {
    data.iter().map(|elfs| elfs.common_letter_value()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 157;
    const SOLUTION_PART1: Output = 7795;
    const EXAMPLE_SOLUTION_PART2: Output = 70;
    const SOLUTION_PART2: Output = 2703;

    fn input_part1() -> Vec<Rucksack> {
        parse_input_part1(include_str!("../input/2022/day3.txt"))
    }
    fn example_input_part1() -> Vec<Rucksack> {
        parse_input_part1(include_str!("../example/2022/day3.txt"))
    }
    fn input_part2() -> Vec<ElfGroup> {
        parse_input_part2(include_str!("../input/2022/day3.txt"))
    }
    fn example_input_part2() -> Vec<ElfGroup> {
        parse_input_part2(include_str!("../example/2022/day3.txt"))
    }
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input_part1()), EXAMPLE_SOLUTION_PART1)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_part1()), SOLUTION_PART1)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input_part2()), EXAMPLE_SOLUTION_PART2)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_part2()), SOLUTION_PART2)
    }
}
