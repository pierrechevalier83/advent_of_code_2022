use aoc_runner_derive::{aoc, aoc_generator};

use crate::input_parser::parse_delimited_vec;
use scan_rules::scan;
use std::collections::{BinaryHeap, VecDeque};
use std::str::FromStr;

#[derive(Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Default for Operation {
    fn default() -> Self {
        Self::Add(0)
    }
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        scan!(s;
            ("new = old * old") =>  Self::Square ,
            ("new = old + ", let x: u64) =>  Self::Add(x as u64),
            ("new = old * ", let x: u64) => Self::Multiply(x as u64),
        )
        .map_err(|e| format!("{e}"))
    }
}

#[derive(Default, Clone)]
struct Destination {
    if_true: usize,
    if_false: usize,
}

struct Items(VecDeque<u64>);

impl FromStr for Items {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            parse_delimited_vec(s, ", ")
                .map_err(|e| format!("{e}"))?
                .into_iter()
                .collect(),
        ))
    }
}

#[derive(Clone, Copy)]
enum OverflowProtection {
    Divide(u64),
    Modulus(u64),
}

#[derive(Default, Clone)]
struct Monkey {
    index: usize,
    items: VecDeque<u64>,
    operation: Operation,
    test_divisibility: u64,
    destination: Destination,
    num_items_inspected: usize,
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let mut monkey = Monkey::default();
        for line in lines {
            scan!(line;
                ("Monkey ", let index: usize, .._) => {monkey.index = index},
                ("  Starting items: ", ..items) => {monkey.items = items.parse::<Items>()?.0},
                ("  Operation: ", ..operation) => {monkey.operation = operation.parse()?},
                ("  Test: divisible by ", let test_divisibility: u64) => {monkey.test_divisibility = test_divisibility as u64},
                ("    If true: throw to monkey ", let i: usize) => {monkey.destination.if_true = i},
                ("    If false: throw to monkey ", let i: usize) => { monkey.destination.if_false = i },
            )
            .map_err(|e| e.to_string())?;
        }
        Ok(monkey)
    }
}

impl Monkey {
    fn throw(&mut self, protection: OverflowProtection) -> Option<(u64, usize)> {
        let item = self.items.pop_front();
        item.map(|item| {
            let mut new_item = match self.operation {
                Operation::Add(x) => item + x,
                Operation::Multiply(x) => item * x,
                Operation::Square => item * item,
            };
            new_item = match protection {
                OverflowProtection::Divide(x) => new_item / x,
                OverflowProtection::Modulus(x) => new_item % x,
            };
            let dest = match new_item % self.test_divisibility {
                0 => self.destination.if_true,
                _ => self.destination.if_false,
            };
            self.num_items_inspected += 1;
            (new_item, dest)
        })
    }
    fn catch(&mut self, item: u64) {
        self.items.push_back(item)
    }
}

#[derive(Clone)]
struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl FromStr for Monkeys {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Monkeys {
            monkeys: s
                .split("\n\n")
                .map(|m| m.parse())
                .collect::<Result<Vec<Monkey>, _>>()?,
        })
    }
}

impl Monkeys {
    fn play_one_round(&mut self, protection: OverflowProtection) {
        for origin in 0..self.monkeys.len() {
            let mut tmp_monkey = self.monkeys[origin].clone();
            while let Some((item, dest)) = tmp_monkey.throw(protection) {
                self.monkeys[dest].catch(item);
            }
            self.monkeys[origin] = tmp_monkey;
        }
    }
    fn play_n_rounds(&mut self, n: usize, protection: OverflowProtection) -> Output {
        for _ in 0..n {
            self.play_one_round(protection);
        }
        let mut top_inspectors = self
            .monkeys
            .iter()
            .map(|monkey| monkey.num_items_inspected)
            .collect::<BinaryHeap<_>>();
        let (alpha, beta) = (top_inspectors.pop().unwrap(), top_inspectors.pop().unwrap());
        alpha * beta
    }
}

type Parsed = Monkeys;
type Input = Monkeys;
type Output = usize;

#[aoc_generator(day11)]
fn parse_input(data: &str) -> Parsed {
    data.parse().unwrap()
}

#[aoc(day11, part1)]
fn part1(data: &Input) -> Output {
    let mut data = data.clone();
    data.play_n_rounds(20, OverflowProtection::Divide(3))
}

#[aoc(day11, part2)]
fn part2(data: &Input) -> Output {
    let mut data = data.clone();
    let modulus = data
        .monkeys
        .iter()
        .map(|monkey| monkey.test_divisibility)
        .product();
    data.play_n_rounds(10_000, OverflowProtection::Modulus(modulus))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 10605;
    const SOLUTION_PART1: Output = 55458;
    const EXAMPLE_SOLUTION_PART2: Output = 2713310158;
    const SOLUTION_PART2: Output = 14508081294;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day11.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day11.txt"))
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
