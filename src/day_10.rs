use aoc_runner_derive::{aoc, aoc_generator};

use crate::input_parser::parse_line_vec;
use std::collections::VecDeque;
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone)]
enum Operation {
    Noop,
    Addx(isize),
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Self::Noop),
            s => Ok(Self::Addx(
                s.split_once(' ')
                    .ok_or_else(|| "Expected a line with a single space".to_string())?
                    .1
                    .parse()
                    .map_err(|e| format!("{e}"))?,
            )),
        }
    }
}

#[derive(Clone)]
struct Computer {
    register_value: isize,
    operations: VecDeque<Operation>,
    operation_in_progress: Option<Operation>,
}

impl Iterator for Computer {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        let previous_register_value = self.register_value;
        if let Some(operation) = &self.operation_in_progress {
            match operation {
                Operation::Addx(x) => {
                    self.register_value += x;
                    self.operation_in_progress = None;
                    return Some(previous_register_value);
                }
                _ => {
                    panic!("Only Addx can be in progress");
                }
            }
        }
        self.operations.pop_front().map(|next_operation| {
            match next_operation {
                Operation::Addx(_) => {
                    self.operation_in_progress = Some(next_operation);
                }
                Operation::Noop => { /* Do nothing, as instructed */ }
            }
            previous_register_value
        })
    }
}

impl FromStr for Computer {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            register_value: 1,
            operations: parse_line_vec(s)?.into_iter().collect(),
            operation_in_progress: None,
        })
    }
}

type Parsed = Computer;
type Input = Computer;
type OutputPart1 = isize;
type OutputPart2 = String;

#[aoc_generator(day10)]
fn parse_input(data: &str) -> Parsed {
    data.parse().unwrap()
}

#[aoc(day10, part1)]
fn part1(data: &Input) -> OutputPart1 {
    data.clone()
        .enumerate()
        .map(|(index, register_value)| (index + 1, register_value))
        .filter(move |(index, _register_value)| {
            *index == 20 || *index > 20 && (*index - 20) % 40 == 0
        })
        .take(6)
        .map(|(index, register_value)| index as isize * register_value)
        .sum()
}

const DISPLAY_WIDTH: usize = 40;
const DISPLAY_HEIGHT: usize = 6;

struct Monitor {
    pixels: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Monitor {
    fn new() -> Self {
        Self {
            pixels: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }
}

impl ToString for Monitor {
    fn to_string(&self) -> String {
        self.pixels
            .chunks(DISPLAY_WIDTH)
            .map(|line| {
                line.iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>()
            })
            .intersperse('\n'.to_string())
            .collect()
    }
}

#[aoc(day10, part2)]
fn part2(data: &Input) -> OutputPart2 {
    let mut monitor = Monitor::new();
    data.clone()
        .enumerate()
        .take(DISPLAY_WIDTH * DISPLAY_HEIGHT)
        .for_each(|(pixel_position, sprite_position)| {
            if ((sprite_position - 1)..=(sprite_position + 1))
                .contains(&((pixel_position % DISPLAY_WIDTH) as isize))
            {
                monitor.pixels[pixel_position] = true;
            }
        });
    monitor.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: OutputPart1 = 13140;
    const SOLUTION_PART1: OutputPart1 = 17840;
    const EXAMPLE_SOLUTION_PART2: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
    const SOLUTION_PART2: &str = "####..##..#.....##..#..#.#....###...##..
#....#..#.#....#..#.#..#.#....#..#.#..#.
###..#..#.#....#....#..#.#....#..#.#....
#....####.#....#.##.#..#.#....###..#.##.
#....#..#.#....#..#.#..#.#....#....#..#.
####.#..#.####..###..##..####.#.....###.";

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day10.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day10.txt"))
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
        assert_eq!(part2(&example_input()).as_str(), EXAMPLE_SOLUTION_PART2)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), SOLUTION_PART2)
    }
}
