use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::{HashMap, VecDeque};
use std::iter::once;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Elevation(u8);

impl From<char> for Elevation {
    fn from(c: char) -> Self {
        assert!(c == 'S' || c == 'E' || c.is_ascii_lowercase());
        match c {
            'S' => Self(0),
            'E' => Self(25),
            c => Self(c as u8 - b'a'),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Position {
    row: u16,
    col: u16,
}

impl Position {
    fn from_index_and_num_cols(index: u16, num_cols: u16) -> Self {
        Self {
            row: index / num_cols,
            col: index % num_cols,
        }
    }
    fn neighbours(&self, num_rows: u16, num_cols: u16) -> impl Iterator<Item = Position> + '_ {
        once((-1, 0))
            .chain(once((1, 0)))
            .chain(once((0, -1)))
            .chain(once((0, 1)))
            .filter_map(|pos| {
                (self.row as i16).checked_add(pos.0).and_then(|row| {
                    (self.col as i16).checked_add(pos.1).map(|col| Self {
                        row: row as u16,
                        col: col as u16,
                    })
                })
            })
            .filter(move |pos| pos.row < num_rows && pos.col < num_cols)
    }
}

#[derive(Debug, Clone)]
struct Input {
    num_cols: u16,
    num_rows: u16,
    topology: HashMap<Position, Elevation>,
    start: Position,
    end: Position,
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_rows = s.lines().count() as u16;
        let num_cols = s
            .lines()
            .next()
            .ok_or_else(|| "No input line".to_string())?
            .chars()
            .count() as u16;
        let topology = s
            .chars()
            .filter(|c| *c != '\n')
            .enumerate()
            .map(|(i, c)| {
                (
                    Position::from_index_and_num_cols(i as u16, num_cols),
                    Elevation::from(c),
                )
            })
            .collect();
        let start = Position::from_index_and_num_cols(
            s.chars()
                .filter(|c| *c != '\n')
                .position(|c| c == 'S')
                .ok_or_else(|| "Missing start".to_string())? as u16,
            num_cols,
        );
        let end = Position::from_index_and_num_cols(
            s.chars()
                .filter(|c| *c != '\n')
                .position(|c| c == 'E')
                .ok_or_else(|| "Missing end".to_string())? as u16,
            num_cols,
        );
        Ok(Self {
            num_cols,
            num_rows,
            topology,
            start,
            end,
        })
    }
}

#[derive(Clone)]
struct PathFinder {
    input: Input,
    cost_to_dest: HashMap<Position, u64>,
    boundary: VecDeque<Position>,
}

impl PathFinder {
    fn from_input(input: Input) -> Self {
        let end = input.end;
        let boundary = end
            .neighbours(input.num_rows, input.num_cols)
            .filter(|neighbour| input.topology[neighbour].0 + 1 >= input.topology[&end].0)
            .collect();
        Self {
            input,
            cost_to_dest: once((end, 0)).collect(),
            boundary,
        }
    }
    fn unexplored_neighbours<'a>(
        &'a self,
        pos: &'a Position,
    ) -> impl Iterator<Item = Position> + 'a {
        pos.neighbours(self.input.num_rows, self.input.num_cols)
            .filter(|neighbour| !self.cost_to_dest.contains_key(neighbour))
    }
    fn explored_reachable_neighbours<'a>(
        &'a self,
        pos: &'a Position,
    ) -> impl Iterator<Item = Position> + 'a {
        pos.neighbours(self.input.num_rows, self.input.num_cols)
            .filter(|p| self.cost_to_dest.contains_key(p))
            .filter(|neighbour| self.input.topology[pos].0 + 1 >= self.input.topology[neighbour].0)
    }
    fn precompute(&mut self) {
        while !self.boundary.is_empty() {
            // transition from boundary to cost to cost_to_dest
            if let Some(pos) = self.boundary.pop_front() {
                if let Some(cost) = self
                    .explored_reachable_neighbours(&pos)
                    .map(|neighbour| self.cost_to_dest[&neighbour] + 1)
                    .min()
                {
                    self.cost_to_dest.insert(pos, cost);
                    for neighbour in self.unexplored_neighbours(&pos).collect::<Vec<_>>() {
                        if !self.boundary.contains(&neighbour) {
                            self.boundary.push_back(neighbour);
                        }
                    }
                }
            }
        }
    }
    fn shortest_path(mut self, start: Position) -> Output {
        self.precompute();
        self.cost_to_dest[&start]
    }
    fn shortest_path_from_any_a(mut self) -> Output {
        self.precompute();
        self.input
            .topology
            .iter()
            .filter(|(_, elevation)| elevation.0 == 0)
            .map(|(start, _)| self.cost_to_dest.get(start).copied().unwrap_or(u64::MAX))
            .min()
            .unwrap()
    }
}

type Parsed = Input;
type Output = u64;

#[aoc_generator(day12)]
fn parse_input(data: &str) -> Parsed {
    data.parse().unwrap()
}

#[aoc(day12, part1)]
fn part1(data: &Input) -> Output {
    let start = data.start;
    PathFinder::from_input(data.clone()).shortest_path(start)
}

#[aoc(day12, part2)]
fn part2(data: &Input) -> Output {
    PathFinder::from_input(data.clone()).shortest_path_from_any_a()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 31;
    const SOLUTION_PART1: Output = 350;
    const EXAMPLE_SOLUTION_PART2: Output = 29;
    const SOLUTION_PART2: Output = 349;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day12.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day12.txt"))
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
