use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::iter::once;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn to_index(self, num_cols: usize) -> usize {
        (self.row * num_cols + self.col) as usize
    }
    fn line_to(self, other: Self) -> Vec<Point> {
        if self.row == other.row {
            ((self.col.min(other.col))..=(self.col.max(other.col)))
                .map(|col| Point { row: self.row, col })
                .collect::<Vec<_>>()
        } else if self.col == other.col {
            ((self.row.min(other.row))..=(self.row.max(other.row)))
                .map(|row| Point { row, col: self.col })
                .collect::<Vec<_>>()
        } else {
            panic!("Only works for segments");
        }
    }
}

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (col, row) = s
            .split_once(',')
            .ok_or(format!("Failed to parse point: {s}"))?;
        Ok(Self {
            row: row.parse().map_err(|e| format!("{e}"))?,
            col: col.parse().map_err(|e| format!("{e}"))?,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Self::Air => ' ',
            Self::Rock => '#',
            Self::Sand => '.',
        }
    }
}

#[derive(Clone)]
struct Map {
    num_rows: usize,
    num_cols: usize,
    min_col: usize,
    tiles: Vec<Tile>,
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .trim_end()
            .split('\n')
            .flat_map(|line| {
                let points = line
                    .split(" -> ")
                    .map(|p| Point::from_str(p).unwrap())
                    .collect::<Vec<_>>();
                points
                    .windows(2)
                    .flat_map(|window| window[0].line_to(window[1]))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Point>>();
        let min_col = points
            .iter()
            .map(|point| point.col)
            .min()
            .ok_or_else(|| "No cols".to_string())?;
        let max_col = points.iter().map(|point| point.col).max().unwrap();
        let num_cols = max_col - min_col + 1;
        let num_rows = points
            .iter()
            .map(|point| point.row)
            .max()
            .ok_or_else(|| "No rows".to_string())?
            + 1;
        let mut tiles = Vec::new();
        tiles.resize_with(num_rows * num_cols, || Tile::Air);
        points.iter().for_each(|point| {
            let point = Point {
                row: point.row,
                col: point.col - min_col,
            };
            tiles[point.to_index(num_cols)] = Tile::Rock;
        });

        Ok(Self {
            num_rows,
            num_cols,
            min_col,
            tiles,
        })
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.tiles
                .chunks(self.num_cols)
                .map(|row| row
                    .iter()
                    .map(|tile| Into::<char>::into(*tile))
                    .chain(once('\n'))
                    .collect::<String>())
                .collect::<String>()
        )
    }
}

fn find_next_sand_position(caves: &Map, mut position: Point) -> Option<Point> {
    loop {
        if position.row == caves.num_rows {
            return None;
        }
        let next_position = Point {
            row: position.row + 1,
            col: position.col,
        };
        if caves.tiles[next_position.to_index(caves.num_cols)] == Tile::Air {
            position = next_position;
            continue;
        }
        if position.col == 0 {
            return None;
        }
        let next_position = Point {
            row: position.row + 1,
            col: position.col - 1,
        };
        if caves.tiles[next_position.to_index(caves.num_cols)] == Tile::Air {
            position = next_position;
            continue;
        }
        if position.col + 1 == caves.num_cols {
            return None;
        }
        let next_position = Point {
            row: position.row + 1,
            col: position.col + 1,
        };
        if caves.tiles[next_position.to_index(caves.num_cols)] == Tile::Air {
            position = next_position;
            continue;
        }
        return Some(position);
    }
}

fn simulate_sandfall(caves: &mut Map) -> usize {
    let mut count = 0;
    while let Some(sand_position) = find_next_sand_position(
        caves,
        Point {
            row: 0,
            col: 500 - caves.min_col,
        },
    ) {
        caves.tiles[sand_position.to_index(caves.num_cols)] = Tile::Sand;
        count += 1;
    }
    count
}

type Parsed = Map;
type Input = Map;
type Output = usize;

#[aoc_generator(day14)]
fn parse_input(data: &str) -> Parsed {
    data.parse().unwrap()
}

#[aoc(day14, part1)]
fn part1(data: &Input) -> Output {
    let mut data = data.clone();
    simulate_sandfall(&mut data)
}

#[aoc(day14, part2)]
fn part2(data: &Input) -> Output {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 24;
    const SOLUTION_PART1: Output = 1513;
    const EXAMPLE_SOLUTION_PART2: Output = 0;
    const SOLUTION_PART2: Output = 0;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day14.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day14.txt"))
    }
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), EXAMPLE_SOLUTION_PART1)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), SOLUTION_PART1)
    }
    /*
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), EXAMPLE_SOLUTION_PART2)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), SOLUTION_PART2)
    }
    */
}
