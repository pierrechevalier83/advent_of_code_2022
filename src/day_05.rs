use aoc_runner_derive::{aoc, aoc_generator};

use crate::input_parser::parse_line_vec;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Stacks(Vec<Vec<char>>);

impl FromStr for Stacks {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rev_lines = s.lines().rev();
        let num_stacks = rev_lines
            .next()
            .ok_or("no num lines")?
            .trim()
            .chars()
            .last()
            .ok_or("no last char")?
            .to_digit(10)
            .ok_or("no last num")? as usize;
        let mut stacks = Vec::new();
        stacks.resize(num_stacks, Vec::new());
        rev_lines.for_each(|line| {
            line.chars()
                .chain(std::iter::once(' '))
                .array_chunks::<4>()
                .enumerate()
                .for_each(|(i, chunk)| {
                    let c = chunk[1];
                    if !c.is_ascii_whitespace() {
                        stacks[i].push(c);
                    }
                })
        });
        Ok(Self(stacks))
    }
}

#[derive(Clone, Debug)]
struct Op {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once("move ").ok_or("no move")?;
        let (q, s) = s.split_once(" from ").ok_or("no from")?;
        let (f, s) = s.split_once(" to ").ok_or("no to")?;
        let t = s.trim_end();
        Ok(Self {
            quantity: q.parse().expect("incorrect quantity"),
            from: f.parse::<usize>().expect("incorrect from") - 1,
            to: t.parse::<usize>().expect("incorrect to") - 1,
        })
    }
}

#[derive(Clone, Debug)]
struct Input {
    stacks: Stacks,
    ops: Vec<Op>,
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks, ops) = s.split_once("\n\n").ok_or("no separation")?;
        Ok(Self {
            stacks: Stacks::from_str(stacks)?,
            ops: parse_line_vec(ops)?,
        })
    }
}

type Parsed = Input;
type Output = String;

#[aoc_generator(day5)]
fn parse_input(data: &str) -> Parsed {
    data.parse().unwrap()
}

#[aoc(day5, part1)]
fn part1(data: &Input) -> Output {
    let mut data = data.clone();
    for op in data.ops.iter() {
        for _ in 0..op.quantity {
            let popped = data.stacks.0[op.from].pop().expect("empty stack");
            data.stacks.0[op.to].push(popped);
        }
    }
    data.stacks
        .0
        .iter()
        .filter_map(|stack| stack.last())
        .collect()
}

#[aoc(day5, part2)]
fn part2(data: &Input) -> Output {
    let mut data = data.clone();
    for op in data.ops.iter() {
        let from_len = data.stacks.0[op.from].len();
        let mut chunk = data.stacks.0[op.from].split_off(from_len - op.quantity);
        data.stacks.0[op.to].append(&mut chunk);
    }
    data.stacks
        .0
        .iter()
        .filter_map(|stack| stack.last())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: &str = "CMZ";
    const SOLUTION_PART1: &str = "BZLVHBWQF";
    const EXAMPLE_SOLUTION_PART2: &str = "MCD";
    const SOLUTION_PART2: &str = "TDGJQTZSL";

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day5.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day5.txt"))
    }
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(&part1(&example_input()), EXAMPLE_SOLUTION_PART1)
    }
    #[test]
    fn test_part1() {
        assert_eq!(&part1(&input()), SOLUTION_PART1)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(&part2(&example_input()), EXAMPLE_SOLUTION_PART2)
    }
    #[test]
    fn test_part2() {
        assert_eq!(&part2(&input()), SOLUTION_PART2)
    }
}
