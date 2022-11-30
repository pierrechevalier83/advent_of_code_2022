My solutions for the "advent of code" challenge of 2022.
===

[Advent of code 2022](https://adventofcode.com/2022)

[![Unit tests](https://github.com/pierrechevalier83/advent_of_code_2022/actions/workflows/rust.yml/badge.svg)](https://github.com/pierrechevalier83/advent_of_code_2022/actions/workflows/rust.yml)

This crate uses `cargo-aoc` for automating the boilerplate.

To install it, run
```
cargo install cargo-aoc
```

# Preparing a new solution
To prepare the boilerplate for a given day (e.g. day 1), run
```
cargo scaffold 1
```

To download the input for today, run
```
cargo aoc input
```

To download the input for a previous day (e.g. day 1), run
```
cargo aoc input -d1
```

Code the generator to parse this day's input and the solutions to each part in `src/day_01.rs`.
Make your solution visible at the top level by uncommenting `pub mod day_01` in `src/lib.rs`.

# Running existing solutions

To run the current day, use
```
cargo aoc
```
To benchmark the current day, use
```
cargo aoc bench
```

To run a specific day (e.g. day 1), use
```
cargo aoc -d1
```
To run a specific day and part (e.g. day 1, part 2), use
```
cargo aoc -d1 -p2
```

To run all solutions, use
```
cargo run --release
```
To run all unit tests, use
```
cargo test --release
```

# Updating the session id 
If the session id expire, log in to the advent of code website, and obtain the cookie id (In Chrome: Shift+F9, Cookies tab, and copy the "Value" for the "session" field).
Then run
```
cargo aoc credentials -s <session id>
```

# Credits

* This project is powered by [cargo-aoc](https://github.com/gobanos/cargo-aoc)
* The scaffold code is strongly inspired from [advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust)

