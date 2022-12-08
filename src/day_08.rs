use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

struct Forest {
    tree_heights: Vec<Vec<u8>>,
}

impl FromStr for Forest {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Forest {
            tree_heights: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            c.to_digit(10)
                                .map(|d| d as u8)
                                .ok_or_else(|| format!("not a digit: {c}"))
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn list_visible_trees(
    line: impl Iterator<Item = (u8, (usize, usize))>,
) -> impl Iterator<Item = (usize, usize)> {
    let mut max_height = None;
    line.filter(move |(height, _pos)| {
        if max_height.map(|m| *height > m).unwrap_or(true) {
            max_height = Some(*height);
            true
        } else {
            false
        }
    })
    .map(|(_h, pos)| pos)
}

type Parsed = Forest;
type Input = Forest;
type Output = usize;

#[aoc_generator(day8)]
fn parse_input(data: &str) -> Parsed {
    data.parse().unwrap()
}

#[aoc(day8, part1)]
fn part1(data: &Input) -> Output {
    let n_cols = data.tree_heights[0].len();
    let n_rows = data.tree_heights.len();
    let visible_from_left = data
        .tree_heights
        .iter()
        .enumerate()
        .flat_map(|(row, heights)| {
            list_visible_trees(
                heights
                    .iter()
                    .enumerate()
                    .map(move |(col, height)| (*height, (row, col))),
            )
        });
    let visible_from_right = data
        .tree_heights
        .iter()
        .enumerate()
        .flat_map(|(row, heights)| {
            list_visible_trees(
                heights
                    .iter()
                    .enumerate()
                    .rev()
                    .map(move |(col, height)| (*height, (row, col))),
            )
        });
    let visible_from_top = (0..n_cols).flat_map(move |col| {
        list_visible_trees((0..n_rows).map(move |row| (data.tree_heights[row][col], (row, col))))
    });
    let visible_from_bottom = (0..n_cols).flat_map(move |col| {
        list_visible_trees(
            (0..n_rows)
                .map(move |row| (data.tree_heights[row][col], (row, col)))
                .rev(),
        )
    });
    let mut all = visible_from_left
        .chain(visible_from_right)
        .chain(visible_from_top)
        .chain(visible_from_bottom)
        .collect::<Vec<_>>();
    all.sort();
    all.dedup();
    all.len()
}

fn count_visible_trees_from_pos(this_height: u8, next_trees: impl Iterator<Item = u8>) -> usize {
    let mut count = 0;
    for height in next_trees {
        count += 1;
        if height >= this_height {
            break;
        }
    }
    count
}

fn scenic_score((row, col): (usize, usize), data: &Input) -> usize {
    let n_cols = data.tree_heights[0].len();
    let n_rows = data.tree_heights.len();
    if row == 0 || col == 0 || row == n_rows - 1 || col == n_cols - 1 {
        0
    } else {
        let this_height = data.tree_heights[row][col];
        let right = count_visible_trees_from_pos(
            this_height,
            data.tree_heights[row].iter().skip(col + 1).cloned(),
        );
        let left = count_visible_trees_from_pos(
            this_height,
            data.tree_heights[row].iter().take(col).rev().cloned(),
        );
        let down = count_visible_trees_from_pos(
            this_height,
            data.tree_heights.iter().map(|line| line[col]).skip(row + 1),
        );
        let up = count_visible_trees_from_pos(
            this_height,
            data.tree_heights
                .iter()
                .map(|line| line[col])
                .take(row)
                .rev(),
        );
        right * left * down * up
    }
}

#[aoc(day8, part2)]
fn part2(data: &Input) -> Output {
    let n_cols = data.tree_heights[0].len();
    let n_rows = data.tree_heights.len();
    (0..n_rows)
        .flat_map(|row| (0..n_cols).map(move |col| scenic_score((row, col), data)))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 21;
    const SOLUTION_PART1: Output = 1835;
    const EXAMPLE_SOLUTION_PART2: Output = 8;
    const SOLUTION_PART2: Output = 263670;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day8.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day8.txt"))
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
