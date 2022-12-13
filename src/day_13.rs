use aoc_runner_derive::{aoc, aoc_generator};

use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;

type Parsed = Vec<(Token, Token)>;
type Input = [(Token, Token)];
type Output = usize;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
#[serde(untagged)]
enum Token {
    Scalar(i8),
    List(Vec<Token>),
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Token::Scalar(left), Token::Scalar(right)) => left.cmp(right),
            (Token::List(left), Token::List(right)) => {
                let longest = left.len().max(right.len());
                for (l, r) in left.iter().zip(right.iter()).take(longest) {
                    let cmp = l.cmp(r);
                    if cmp.is_ne() {
                        return cmp;
                    }
                }
                left.len().cmp(&right.len())
            }
            (Token::Scalar(left), Token::List(_)) => {
                Token::List(vec![Token::Scalar(*left)]).cmp(other)
            }
            (Token::List(_), Token::Scalar(right)) => {
                self.cmp(&Token::List(vec![Token::Scalar(*right)]))
            }
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day13)]
fn parse_input(data: &str) -> Parsed {
    data.split("\n\n")
        .map(|s| {
            let (left, right) = s.split_once('\n').unwrap();
            (
                serde_json::from_str::<Token>(left).unwrap(),
                serde_json::from_str::<Token>(right).unwrap(),
            )
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(data: &Input) -> Output {
    data.iter()
        .enumerate()
        .filter(|(_, tokens)| tokens.0 < tokens.1)
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
fn part2(data: &Input) -> Output {
    let mut all_packets = data
        .iter()
        .flat_map(|(left, right)| vec![left, right])
        .cloned()
        .collect::<Vec<_>>();
    let delims = vec![
        Token::List(vec![Token::List(vec![Token::Scalar(2)])]),
        Token::List(vec![Token::List(vec![Token::Scalar(6)])]),
    ];
    all_packets.extend(delims.clone());
    all_packets.sort();
    all_packets
        .iter()
        .enumerate()
        .filter(|(_, t)| delims.contains(t))
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 13;
    const SOLUTION_PART1: Output = 5393;
    const EXAMPLE_SOLUTION_PART2: Output = 140;
    const SOLUTION_PART2: Output = 26712;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day13.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day13.txt"))
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
