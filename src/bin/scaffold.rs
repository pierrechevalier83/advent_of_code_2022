/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

const YEAR: u16 = 2022;

const MODULE_TEMPLATE: &str = r###"use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = ();
type Input = ();
type Output = u64;

#[aoc_generator(dayDAY)]
fn parse_input(data: &str) -> Parsed {
    unimplemented!();
}

#[aoc(dayDAY, part1)]
fn part1(data: &Input) -> Output {
    unimplemented!();
}

#[aoc(dayDAY, part2)]
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
        parse_input(include_str!("../input/YEAR/dayDAY.txt"))
    }
    fn example_input() -> Parsed {
        parse_input(include_str!("../example/YEAR/dayDAY.txt"))
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
"###;

fn parse_args() -> Result<u8, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    args.free_from_str()
}

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

fn main() {
    let day = match parse_args() {
        Ok(day) => day,
        Err(_) => {
            eprintln!("Need to specify a day (as integer). example: `cargo scaffold 7`");
            process::exit(1);
        }
    };

    let day_padded = format!("{:02}", day);

    let input_path = format!("input/{}/day{}.txt", YEAR, day);
    let example_path = format!("example/{}/day{}.txt", YEAR, day);
    let module_path = format!("src/day_{}.rs", day_padded);

    let mut file = match safe_create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {}", e);
            process::exit(1);
        }
    };

    match file.write_all(
        MODULE_TEMPLATE
            .replace("YEAR", &YEAR.to_string())
            .replace("PADDED_DAY", &day_padded)
            .replace("DAY", &day.to_string())
            .as_bytes(),
    ) {
        Ok(_) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {}", e);
            process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {}", e);
            process::exit(1);
        }
    }

    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {}", e);
            process::exit(1);
        }
    }

    println!("---");
    println!(
        "🎄 Type `cargo aoc input -d{}` to download your input.",
        &day
    );
    println!("🎄 Type `cargo aoc -d{}` to run your solution.", &day);
}
