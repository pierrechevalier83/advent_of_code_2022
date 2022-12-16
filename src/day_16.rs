use aoc_runner_derive::{aoc, aoc_generator};

use crate::input_parser::parse_delimited_vec;

use std::collections::VecDeque;
use std::fmt::Debug;
use std::str::FromStr;

const LABEL_SPACE: usize = 26 * 26;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Label(u16);

impl FromStr for Label {
    type Err = String;
    fn from_str(tag: &str) -> Result<Self, Self::Err> {
        assert!(tag.len() == 2);
        let mut chars = tag.chars();
        let index = (chars.next().unwrap() as u8 - b'A') as u16
            + 26 * (chars.next().unwrap() as u8 - b'A') as u16;
        println!("{tag}, {index}");
        Ok(Self(index))
    }
}

impl From<Label> for usize {
    fn from(label: Label) -> Self {
        label.0 as usize
    }
}

impl From<usize> for Label {
    fn from(x: usize) -> Self {
        Self(x as u16)
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ((self.0 / 26) as u8 + b'A') as char)?;
        write!(f, "{}", ((self.0 % 26) as u8 + b'A') as char)
    }
}

struct LabelSet([bool; LABEL_SPACE]);

impl LabelSet {
    fn empty() -> Self {
        Self([false; LABEL_SPACE])
    }
    fn from_labels(labels: &[Label]) -> Self {
        let mut set = Self::empty();
        for label in labels {
            set.insert(*label);
        }
        set
    }
    fn insert(&mut self, label: Label) {
        self.0[usize::from(label.0)] = true;
    }
    fn contains(&self, label: Label) -> bool {
        self.0[usize::from(label)]
    }
    fn labels(&self) -> impl Iterator<Item = Label> + '_ {
        self.0.iter().enumerate().filter_map(
            |(index, is_set)| {
                if *is_set {
                    Some(index.into())
                } else {
                    None
                }
            },
        )
    }
}

impl Debug for LabelSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.labels() {
            write!(f, "{l:?}, ")?;
        }
        Ok(())
    }
}

impl Default for LabelSet {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone)]
// Conceptually, [T; LABEL_SPACE], but I don't want to restrict T to Copy types
struct LabelMap<T>(Vec<T>)
where
    T: Default + Clone;

impl<T> LabelMap<T>
where
    T: Default + Clone,
{
    fn get(&self, label: Label) -> &T {
        &self.0[usize::from(label)]
    }
    fn get_mut(&mut self, label: Label) -> &mut T {
        &mut self.0[usize::from(label)]
    }
    fn set(&mut self, label: Label, x: T) {
        self.0[usize::from(label)] = x;
    }
}

impl<T> Default for LabelMap<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        let mut vec = Vec::with_capacity(LABEL_SPACE);
        vec.resize(LABEL_SPACE, Default::default());
        Self(vec)
    }
}

#[derive(Default, Debug)]
struct CavesNetwork {
    caves: LabelSet,
    // Because it's undirected, we'll need to put a connection at origin and destination
    // If tunnels['AA'] == 'BB', 'AA' and 'BB' are connected.
    // Also, we can expect that tunnels['BB'].contains('AA')
    tunnels: LabelMap<Vec<Label>>,
    flow_rates: LabelMap<u8>,
}

impl FromStr for CavesNetwork {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut net = CavesNetwork::default();
        for line in s.lines() {
            let (label, line) = line
                .strip_prefix("Valve ")
                .unwrap()
                .split_once(' ')
                .unwrap();
            let label = Label::from_str(label)?;
            let (flow_rate, line) = line
                .strip_prefix("has flow rate=")
                .unwrap()
                .split_once(';')
                .unwrap();
            let flow_rate = flow_rate.parse().unwrap();
            let (_, connections) = line
                .split_once("to valves ")
                .unwrap_or_else(|| line.split_once("to valve ").unwrap());
            let connections = parse_delimited_vec::<Label>(connections, ", ")?;
            net.caves.insert(label);
            net.flow_rates.set(label, flow_rate);
            for conn in connections {
                net.caves.insert(conn);
                net.tunnels.get_mut(label).push(conn);
            }
        }
        Ok(net)
    }
}

type Parsed = CavesNetwork;
type Input = CavesNetwork;
type Output = u64;

#[aoc_generator(day16)]
fn parse_input(data: &str) -> Parsed {
    data.parse().unwrap()
}

fn shortest_paths_from_cave(origin: Label, tunnels: &LabelMap<Vec<Label>>) -> LabelMap<Option<u8>> {
    let mut shortest_paths = LabelMap::<Option<u8>>::default();
    let mut boundary = VecDeque::new();
    boundary.push_back(origin);
    let mut shortest_path = 0;
    while !boundary.is_empty() {
        let mut new_boundary = VecDeque::new();
        for label in boundary {
            if shortest_paths.get(label).is_none() {
                shortest_paths.set(label, Some(shortest_path));
                for conn in tunnels.get(label) {
                    new_boundary.push_back(*conn);
                }
            }
        }
        boundary = new_boundary;
        shortest_path += 1;
    }
    shortest_paths
}

fn precompute_all_shortest_paths(data: &CavesNetwork) -> LabelMap<LabelMap<Option<u8>>> {
    let mut shortest_paths: LabelMap<LabelMap<Option<u8>>> = Default::default();
    for cave in data.caves.labels() {
        shortest_paths.set(cave, shortest_paths_from_cave(cave, &data.tunnels));
    }
    shortest_paths
}

const MAX_TIME: u64 = 30;
// TODO: this + memoization should work
fn expected_value_at_time(time: u64, cave: Label, caves: &CavesNetwork) -> (u64, Vec<Label>) {
    if time >= MAX_TIME {
        return (0, Vec::new());
    }
    let remaining_time = if time == MAX_TIME {
        0
    } else {
        MAX_TIME - time - 1
    };
    // Open this valve and move on
    let max_neighbour_value_in_two_minutes = caves
        .tunnels
        .get(cave)
        .iter()
        .map(|neighbour| expected_value_at_time(time + 2, *neighbour, caves))
        // Filter out paths that have already opened our valve as in that case, it would be a
        // waste of time to open it again.
        .filter(|neighbour| !neighbour.1.contains(&cave))
        .max_by(|l, r| l.0.cmp(&r.0))
        .unwrap_or((0, Vec::new()));
    let value_if_open_valve =
        *caves.flow_rates.get(cave) as u64 * remaining_time + max_neighbour_value_in_two_minutes.0;
    let max_neighbour_value_in_one_minute = caves
        .tunnels
        .get(cave)
        .iter()
        .map(|neighbour| expected_value_at_time(time + 1, *neighbour, caves))
        .max_by(|l, r| l.0.cmp(&r.0))
        .unwrap_or((0, Vec::new()));
    let value_if_skip_valve = max_neighbour_value_in_one_minute.0;
    if value_if_open_valve > value_if_skip_valve {
        let mut path = max_neighbour_value_in_two_minutes.1;
        path.push(cave);
        (value_if_open_valve, path)
    } else {
        max_neighbour_value_in_one_minute
    }
}

fn expected_values_at_time(time: u64, caves: &CavesNetwork) -> LabelMap<(u64, Vec<Label>)> {
    println!("At time: {time}");
    let mut expected_values: LabelMap<(u64, Vec<Label>)> = Default::default();
    for label in caves.caves.labels() {
        let (value, path) = expected_value_at_time(time, label, caves);
        println!("({label:?} -> {value:?}: {path:?})");
        expected_values.set(label, (value, path));
    }
    expected_values
}
/*
 * This approach is suboptimal : If we pick the best value later in time, we don't maximize value overall

fn expected_value_at_time(
    time: u64,
    cave: Label,
    caves: &CavesNetwork,
    expected_values_in_one_minute: &LabelMap<(u64, Vec<Label>)>,
    expected_values_in_two_minutes: &LabelMap<(u64, Vec<Label>)>,
) -> (u64, Vec<Label>) {
    let remaining_time = if time == MAX_TIME {
        0
    } else {
        MAX_TIME - time - 1
    };
    // Open this valve and move on
    let max_neighbour_value_in_two_minutes = caves
        .tunnels
        .get(cave)
        .iter()
        .map(|neighbour| expected_values_in_two_minutes.get(*neighbour))
        // Filter out paths that have already opened our valve as in that case, it would be a
        // waste of time to open it again.
        .filter(|neighbour| !neighbour.1.contains(&cave))
        .max_by(|l, r| l.0.cmp(&r.0))
        .cloned()
        .unwrap_or((0, Vec::new()));
    let value_if_open_valve =
        *caves.flow_rates.get(cave) as u64 * remaining_time + max_neighbour_value_in_two_minutes.0;
    let max_neighbour_value_in_one_minute = caves
        .tunnels
        .get(cave)
        .iter()
        .map(|neighbour| expected_values_in_one_minute.get(*neighbour))
        .max_by(|l, r| l.0.cmp(&r.0))
        .cloned()
        .unwrap_or((0, Vec::new()));
    let value_if_skip_valve = max_neighbour_value_in_one_minute.0;
    if value_if_open_valve > value_if_skip_valve {
        let mut path = max_neighbour_value_in_two_minutes.1;
        path.push(cave);
        (value_if_open_valve, path)
    } else {
        max_neighbour_value_in_one_minute
    }
}

fn expected_values_at_time(
    time: u64,
    caves: &CavesNetwork,
    expected_values_in_one_minute: &LabelMap<(u64, Vec<Label>)>,
    expected_values_in_two_minutes: &LabelMap<(u64, Vec<Label>)>,
) -> LabelMap<(u64, Vec<Label>)> {
    println!("At time: {time}");
    let mut expected_values: LabelMap<(u64, Vec<Label>)> = Default::default();
    for label in caves.caves.labels() {
        let (value, path) = expected_value_at_time(
            time,
            label,
            caves,
            expected_values_in_one_minute,
            expected_values_in_two_minutes,
        );
        println!("({label:?} -> {value:?}: {path:?})");
        expected_values.set(label, (value, path));
    }
    expected_values
}
*/
#[aoc(day16, part1)]
fn part1(data: &Input) -> Output {
    //let mut expected_values_in_one_minute = LabelMap::<(u64, Vec<Label>)>::default();
    //let mut expected_values_in_two_minutes = LabelMap::<(u64, Vec<Label>)>::default();
    let mut expected_values = LabelMap::<(u64, Vec<Label>)>::default();
    for time in 1..=30 {
        expected_values = expected_values_at_time(time, data)
    }
    /*
    // Backtrack (incorrect)
    for time in (1..=30).rev() {
        expected_values = expected_values_at_time(
            time,
            data,
            &expected_values_in_one_minute,
            &expected_values_in_two_minutes,
        );
        expected_values_in_two_minutes = expected_values_in_one_minute;
        expected_values_in_one_minute = expected_values.clone();
    }
    */
    expected_values.get(Label::from_str("AA").unwrap()).0
}

#[aoc(day16, part2)]
fn part2(data: &Input) -> Output {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 1651;
    const SOLUTION_PART1: Output = 0;
    const EXAMPLE_SOLUTION_PART2: Output = 0;
    const SOLUTION_PART2: Output = 0;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day16.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day16.txt"))
    }
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), EXAMPLE_SOLUTION_PART1)
    }
    /*
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

    */
}
