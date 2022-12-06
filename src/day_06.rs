use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<char>;
type Input = [char];
type Output = usize;

#[aoc_generator(day6)]
fn parse_input(data: &str) -> Parsed {
    data.chars().collect()
}

fn start_packet(size: usize, data: &[char]) -> usize {
    data.windows(size)
        .position(|window| {
            window
                .iter()
                .all(|c| window.iter().filter(|d| *d != c).count() == size - 1)
        })
        .map(|pos| pos + size)
        .unwrap_or(data.len())
}

#[aoc(day6, part1)]
fn part1(data: &Input) -> Output {
    start_packet(4, data)
}

#[aoc(day6, part2)]
fn part2(data: &Input) -> Output {
    start_packet(14, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOLUTION_PART1: Output = 1929;
    const SOLUTION_PART2: Output = 3298;
    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day6.txt"))
    }
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
        assert_eq!(part1(&parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(part1(&parse_input("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(part1(&parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(part1(&parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), SOLUTION_PART1)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(part2(&parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(part2(&parse_input("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(part2(&parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(part2(&parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), SOLUTION_PART2)
    }
}
