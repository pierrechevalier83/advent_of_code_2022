use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;
use vfs::{MemoryFS, VfsError, VfsPath, VfsResult};

const TOTAL_DISK_SPACE: usize = 70_000_000;
const NEEDED_FREE_SPACE: usize = 30_000_000;

fn get_path_size(path: VfsPath) -> usize {
    if path.is_file().unwrap() {
        path
    } else {
        path.join("DIR_SIZE").unwrap()
    }
    .read_to_string()
    .unwrap()
    .parse::<usize>()
    .unwrap()
}

struct Input {
    root: VfsPath,
}

impl FromStr for Input {
    type Err = VfsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = VfsPath::new(MemoryFS::new());
        let mut current_dir = root.clone();
        let commands = s.trim().split("$ ").collect::<Vec<_>>();
        // Parse all commands to get a representation of the filesystem as is
        for command in commands {
            let mut lines = command.lines();
            if let Some(command) = lines.next() {
                if command == "ls" {
                    for output_line in lines {
                        if output_line.starts_with("dir ") {
                            let dir = output_line.strip_prefix("dir ").unwrap();
                            current_dir.join(dir)?.create_dir()?;
                        } else {
                            let (size, file_name) = output_line.split_once(' ').unwrap();
                            write!(current_dir.join(file_name)?.create_file()?, "{size}").unwrap();
                        }
                    }
                } else if command.starts_with("cd ") {
                    let dir = command.strip_prefix("cd ").unwrap();
                    if dir == "/" {
                        current_dir = root.clone();
                    } else {
                        current_dir = current_dir.join(dir)?;
                    }
                }
            }
        }
        // Walk the virtual filesystem once to create a file per dir indicating its size, so we
        // don't need to re-walk each directory later
        for path in root
            .walk_dir()?
            .collect::<VfsResult<Vec<_>>>()?
            .iter()
            .rev()
        {
            if path.is_dir().unwrap() {
                let dir_size: usize = path.read_dir().unwrap().map(get_path_size).sum();
                write!(
                    path.join("DIR_SIZE").unwrap().create_file().unwrap(),
                    "{dir_size}"
                )
                .unwrap();
            }
        }
        Ok(Self { root })
    }
}

type Parsed = Input;
type Output = usize;

#[aoc_generator(day7)]
fn parse_input(data: &str) -> Parsed {
    Input::from_str(data).unwrap()
}

#[aoc(day7, part1)]
fn part1(data: &Input) -> Output {
    data.root
        .walk_dir()
        .unwrap()
        .filter_map(|path| path.ok())
        .filter(|path| path.is_dir().unwrap())
        .map(get_path_size)
        .filter(|dir_size| *dir_size < 100_000)
        .sum()
}

#[aoc(day7, part2)]
fn part2(data: &Input) -> Output {
    let used_space: usize = data.root.read_dir().unwrap().map(get_path_size).sum();
    let unused_space = dbg!(TOTAL_DISK_SPACE) - dbg!(used_space);
    let extra_needed_space = NEEDED_FREE_SPACE - unused_space;
    data.root
        .walk_dir()
        .unwrap()
        .filter_map(|path| path.ok())
        .filter(|path| path.is_dir().unwrap())
        .map(get_path_size)
        .filter(|dir_size| *dir_size > extra_needed_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SOLUTION_PART1: Output = 95437;
    const SOLUTION_PART1: Output = 1077191;
    const EXAMPLE_SOLUTION_PART2: Output = 24933642;
    const SOLUTION_PART2: Output = 5649896;

    fn input() -> Parsed {
        parse_input(include_str!("../input/2022/day7.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/2022/day7.txt"))
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
