use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::BinaryHeap;

type Parsed = Vec<Vec<u32>>;
type Input = [Vec<u32>];
type Output = u32;

#[aoc_generator(day1)]
fn parse_input(data: &str) -> Parsed {
    data.trim()
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|cal| cal.parse().expect("failed to parse cal"))
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(data: &Input) -> Output {
    data.iter()
        .map(|elf| elf.iter().sum::<u32>())
        .max()
        .unwrap_or(0)
}

#[aoc(day1, part2, Naive)]
fn part2_naive(data: &Input) -> Output {
    let mut top_elves = data
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .collect::<Vec<_>>();
    top_elves.sort();

    top_elves.iter().rev().take(3).sum::<u32>()
}

#[aoc(day1, part2, Faster)]
fn part2(data: &Input) -> Output {
    let top_elves = data
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .collect::<BinaryHeap<_>>();

    top_elves.iter().take(3).sum::<u32>()
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
