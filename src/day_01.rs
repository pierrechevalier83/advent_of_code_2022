use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = ();
type Input = ();
type Output = u64;

#[aoc_generator(day1)]
fn parse_input(data: &str) -> Parsed {
    unimplemented!();
}

#[aoc(day1, part1)]
fn part1(data: &Input) -> Output {
    unimplemented!();
}

#[aoc(day1, part2)]
fn part2(data: &Input) -> Output {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 0;
    const EXAMPLE_SOLUTION_PART2: Output = 0;
    const SOLUTION_PART1: Output = 0;
    const SOLUTION_PART2: Output = 0;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day1.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day1.txt"))
    }
    use super::*;
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
