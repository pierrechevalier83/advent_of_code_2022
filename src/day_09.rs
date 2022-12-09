use aoc_runner_derive::{aoc, aoc_generator};

use crate::input_parser::parse_line_vec;
use generic_array::{typenum::U1, typenum::U10, ArrayLength, GenericArray};
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::str::FromStr;

type Position = (i32, i32);

#[derive(Debug, Default)]
struct RopeLink {
    head: Position,
    tail: Position,
}

impl RopeLink {
    fn move_head(&mut self, d: Direction) {
        let m = match d {
            Direction::Up => (1, 0),
            Direction::Down => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        };
        self.head.0 += m.0;
        self.head.1 += m.1;
    }
    fn move_tail(&mut self) {
        let distance = (
            (self.head.0 - self.tail.0).abs(),
            (self.head.1 - self.tail.1).abs(),
        );
        if distance.0 > 1 && self.head.1 == self.tail.1 {
            if self.head.0 > self.tail.0 {
                assert!(self.tail.0 + 1 == self.head.0 - 1);
                self.tail.0 = self.head.0 - 1;
            } else {
                assert!(self.tail.0 - 1 == self.head.0 + 1);
                self.tail.0 = self.head.0 + 1;
            }
        } else if self.head.0 == self.tail.0 && distance.1 > 1 {
            if self.head.1 > self.tail.1 {
                assert!(self.tail.1 + 1 == self.head.1 - 1);
                self.tail.1 = self.head.1 - 1;
            } else {
                assert!(self.tail.1 - 1 == self.head.1 + 1);
                self.tail.1 = self.head.1 + 1;
            }
        } else if distance.0 > 1 {
            if self.head.0 > self.tail.0 {
                self.tail.0 += distance.0 - 1;
            } else {
                self.tail.0 -= distance.0 - 1;
            }
            if self.head.1 > self.tail.1 {
                self.tail.1 += 1;
            } else {
                self.tail.1 -= 1;
            }
        } else if distance.1 > 1 {
            if self.head.0 > self.tail.0 {
                self.tail.0 += 1;
            } else {
                self.tail.0 -= 1;
            }
            if self.head.1 > self.tail.1 {
                self.tail.1 += distance.1 - 1;
            } else {
                self.tail.1 -= distance.1 - 1;
            }
        }
        let distance = (
            (self.head.0 - self.tail.0).abs(),
            (self.head.1 - self.tail.1).abs(),
        );
        assert!(distance.0 <= 1);
        assert!(distance.1 <= 1);
        println!("After move: Head: {:?}, Tail: {:?}", self.head, self.tail);
    }
}

#[derive(Debug, Default)]
struct Rope<N: ArrayLength<RopeLink>> {
    links: GenericArray<RopeLink, N>,
}

impl<N> Rope<N>
where
    N: ArrayLength<RopeLink>,
{
    // Move one step and return the tail position
    fn move_one_step(&mut self, d: Direction) -> Position {
        self.links[0].move_head(d);
        self.links[0].move_tail();
        for i in 1..self.links.len() {
            self.links[i].head = self.links[i - 1].tail;
            self.links[i].move_tail();
        }
        println!("===");
        self.links[self.links.len() - 1].tail
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => Result::Err(format!("Invalid direction: {s}"))?,
        })
    }
}

#[derive(Debug, Clone)]
struct Move {
    dir: Direction,
    num: i32,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, n) = s
            .split_once(' ')
            .ok_or_else(|| format!("Invalid move: {s}"))?;
        Ok(Self {
            dir: d.parse()?,
            num: n.parse().map_err(|e| format!("{e}"))?,
        })
    }
}

#[derive(Debug, Default, Clone)]
struct Moves(VecDeque<Move>);

impl FromStr for Moves {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_line_vec(s)?.into_iter().collect()))
    }
}

#[derive(Default)]
struct Simulation<N: ArrayLength<RopeLink>> {
    rope: Rope<N>,
    remaining_moves: Moves,
}

impl<N> Simulation<N>
where
    N: ArrayLength<RopeLink>,
{
    fn from_moves(moves: Moves) -> Self {
        Self {
            remaining_moves: moves,
            ..Default::default()
        }
    }
}

impl<N> Iterator for Simulation<N>
where
    N: ArrayLength<RopeLink>,
{
    // the tail positon
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_moves.0.is_empty() {
            None
        } else {
            let mut next_move = self.remaining_moves.0.get_mut(0).unwrap();
            let tail = Some(self.rope.move_one_step(next_move.dir));
            if next_move.num == 1 {
                self.remaining_moves.0.pop_front();
            } else {
                next_move.num -= 1
            }
            tail
        }
    }
}

type Input = Moves;
type Parsed = Moves;
type Output = usize;

#[aoc_generator(day9)]
fn parse_input(data: &str) -> Parsed {
    data.parse().unwrap()
}

#[aoc(day9, part1)]
fn part1(data: &Input) -> Output {
    Simulation::<U1>::from_moves(data.clone())
        .collect::<BTreeSet<_>>()
        .len()
}

#[aoc(day9, part2)]
fn part2(data: &Input) -> Output {
    Simulation::<U10>::from_moves(data.clone())
        .collect::<BTreeSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 13;
    const SOLUTION_PART1: Output = 6081;
    const EXAMPLE_SOLUTION_PART2: Output = 36;
    const SOLUTION_PART2: Output = 0;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day9.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day9.txt"))
    }
    fn example_input_part2() -> Parsed {
        parse_input(include_str!("../example/2022/day9_part2.txt"))
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
        assert_eq!(part2(&example_input_part2()), EXAMPLE_SOLUTION_PART2)
    }
    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), SOLUTION_PART2)
    }
    */
}
