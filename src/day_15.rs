use aoc_runner_derive::{aoc, aoc_generator};

use scan_rules::scan;
use std::collections::{BTreeSet, HashSet};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    closest_beacon: Pos,
}

impl FromStr for Sensor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        scan!(s;
            ("Sensor at x=", let sx: i64, ", y=", let sy:i64, ": closest beacon is at x=", let bx: i64, ", y=", let by: i64) =>  Self{pos: Pos{x: sx, y: sy}, closest_beacon: Pos {x: bx, y: by}},
        )
        .map_err(|e| format!("{e}"))
    }
}

impl Sensor {
    fn manhattan_distance(&self) -> i64 {
        (self.pos.x - self.closest_beacon.x).abs() + (self.pos.y - self.closest_beacon.y).abs()
    }
    fn is_in_range(&self, y: i64) -> bool {
        ((self.pos.y - self.manhattan_distance())..=(self.pos.y + self.manhattan_distance()))
            .contains(&y)
    }
    fn exclusion_range_at_line(&self, y: i64) -> RangeInclusive<i64> {
        let start = self.pos.x - self.manhattan_distance() + (self.pos.y - y).abs();
        let end = self.pos.x + self.manhattan_distance() - (self.pos.y - y).abs();
        start..=end
    }
}

struct Map {
    sensors: Vec<Sensor>,
}

impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sensors: s
                .lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Default, Debug)]
struct CoveredRange {
    // Invariant: the ranges are non-overlapping
    ranges: Vec<RangeInclusive<i64>>,
}

impl CoveredRange {
    fn contains(&self, x: i64) -> bool {
        self.ranges.iter().any(|r| r.contains(&x))
    }
    fn next_overlapping_range(&self, range: &RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
        self.ranges
            .iter()
            .find(|existing| {
                existing.contains(range.start())
                    || existing.contains(range.end())
                    || range.contains(existing.start())
                    || range.contains(existing.end())
            })
            .cloned()
    }
    fn insert(&mut self, range: RangeInclusive<i64>) {
        if let Some(existing) = self.next_overlapping_range(&range) {
            if existing.contains(range.start()) && existing.contains(range.end()) {
                // No need to insert: already covered
                return;
            }
            if existing.contains(range.start())
                || range.contains(existing.end()) && existing.end() < range.end()
            {
                self.insert((existing.end() + 1)..=*range.end());
            }
            if existing.contains(range.end())
                || range.contains(existing.start()) && range.start() < existing.start()
            {
                self.insert(*range.start()..=(existing.start() - 1));
            }
        } else {
            // No remaining overlap situation. Actually insert
            self.ranges.push(range);
        }
    }
    fn num_covered_positions(&self) -> i64 {
        self.ranges.iter().map(|r| r.end() - r.start() + 1).sum()
    }
}

type Parsed = Map;
type Input = Map;
type Output = i64;

#[aoc_generator(day15)]
fn parse_input(data: &str) -> Parsed {
    data.parse().unwrap()
}

fn part1(data: &Input, line: i64) -> Output {
    let mut covered_range = CoveredRange::default();
    for range in data.sensors.iter().filter_map(|sensor| {
        if sensor.is_in_range(line) {
            Some(sensor.exclusion_range_at_line(line))
        } else {
            None
        }
    }) {
        covered_range.insert(range);
    }
    covered_range.num_covered_positions()
        - data
            .sensors
            .iter()
            .filter_map(|sensor| {
                if sensor.closest_beacon.y == line
                    && covered_range.contains(sensor.closest_beacon.x)
                {
                    Some(sensor.closest_beacon.x)
                } else if sensor.pos.y == line && covered_range.contains(sensor.pos.x) {
                    Some(sensor.pos.x)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>()
            .len() as i64
}

// A range that we guarantee we'll only compare with non overlapping ranges, so we can order it
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct OrdRange {
    start: i64,
    end: i64,
}

#[aoc(day15, part1, Example)]
fn part1_example(data: &Input) -> Output {
    part1(data, 10)
}

#[aoc(day15, part1, Big)]
fn part1_big(data: &Input) -> Output {
    part1(data, 2_000_000)
}

fn tuning_frequency(pos: Pos) -> i64 {
    4_000_000 * pos.x + pos.y
}

fn part2(data: &Input, max_line: i64) -> Output {
    let busy_positions = data
        .sensors
        .iter()
        .flat_map(|sensor| vec![sensor.pos, sensor.closest_beacon])
        .collect::<HashSet<_>>();
    for line in 0..=max_line {
        let mut covered_range = CoveredRange::default();
        for range in data.sensors.iter().filter_map(|sensor| {
            if sensor.is_in_range(line) {
                Some(sensor.exclusion_range_at_line(line))
            } else {
                None
            }
        }) {
            covered_range.insert(range);
        }
        let mut ranges = covered_range
            .ranges
            .iter()
            .map(|range| OrdRange {
                start: *range.start(),
                end: *range.end(),
            })
            .collect::<Vec<_>>();
        ranges.sort();
        for window in ranges.windows(2) {
            if window[0].end + 1 < window[1].start {
                let x = window[0].end + 1;
                let candidate = Pos { x, y: line };
                if !busy_positions.contains(&candidate) {
                    return tuning_frequency(Pos { x, y: line });
                }
            }
        }
    }
    0
}

#[aoc(day15, part2, Example)]
fn part2_example(data: &Input) -> Output {
    part2(data, 20)
}

#[aoc(day15, part2, Big)]
fn part2_big(data: &Input) -> Output {
    part2(data, 4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 26;
    const SOLUTION_PART1: Output = 4985193;
    const EXAMPLE_SOLUTION_PART2: Output = 56000011;
    const SOLUTION_PART2: Output = 11583882601918;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day15.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day15.txt"))
    }
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1_example(&example_input()), EXAMPLE_SOLUTION_PART1)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1_big(&input()), SOLUTION_PART1)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2_example(&example_input()), EXAMPLE_SOLUTION_PART2)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2_big(&input()), SOLUTION_PART2)
    }
}
