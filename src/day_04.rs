use aoc_runner_derive::{aoc, aoc_generator};

use std::ops::RangeInclusive;
use std::str::FromStr;

struct Elf(RangeInclusive<u64>);

impl FromStr for Elf {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (begin, end) = s.split_once('-').expect("Missing dash");
        Ok(Self(
            begin.parse().map_err(|e| format!("{e:?}"))?
                ..=end.parse().map_err(|e| format!("{e:?}"))?,
        ))
    }
}

type Parsed = Vec<(Elf, Elf)>;
type Input = [(Elf, Elf)];
type Output = usize;

#[aoc_generator(day4)]
fn parse_input(data: &str) -> Parsed {
    data.trim()
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            (first.parse().unwrap(), second.parse().unwrap())
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(data: &Input) -> Output {
    data.iter()
        .filter(|(x, y)| {
            (x.0.contains(y.0.start()) && x.0.contains(y.0.end()))
                || (y.0.contains(x.0.start()) && y.0.contains(x.0.end()))
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(data: &Input) -> Output {
    data.iter()
        .filter(|(x, y)| {
            (x.0.contains(y.0.start()) || x.0.contains(y.0.end()))
                || (y.0.contains(x.0.start()) || y.0.contains(x.0.end()))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 2;
    const SOLUTION_PART1: Output = 464;
    const EXAMPLE_SOLUTION_PART2: Output = 4;
    const SOLUTION_PART2: Output = 770;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day4.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day4.txt"))
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
