Advent of Code
==============

My solutions for [Advent of Code][aoc], written in [Rust][rust].

This repository provides a good template for anyone interested in writing
their solutions in Rust. Follow the instructions below to get started!

(And I stole this template originally from [smores56](https://github.com/smores56/aoc-2022)).

## Setup

### Prerequisites

If you don't already have [Rust][rust] installed, you'll want to
[install it here][install rust]. Otherwise, the only things you'll need
are a terminal/shell and a text editor.

### Clobbering my Solutions (For Your Integrity!)

To get started, you'll want to fork this repository and then delete any
of my solutions I've submitted so far to keep yourself from being spoiled:

```bash
rm src/day*.rs
```

Also, in the `src/main.rs` file, you'll need to delete all `mod dayN;`
declarations and `use dayN::DayN;` statements, as well as reset the
`get_day_solution` implementation to the following:

```rust
fn get_day_solution(day: usize, lines: impl Iterator<Item = String>) -> Box<dyn DaySolution> {
    match day {
        // 1 => Box::new(Day1::from_lines(input)),
        _other => panic!("Day hasn't been solved yet"),
    }
}
```

Now you're ready to go!. You'll want to log in to [Advent of Code][aoc]
so that you can download inputs automatically.

### Logging In to Advent of Code

Go to the [Advent of Code][aoc] site and log in at the top of the
page. Once you're signed in, you should open the Developer Tools and head
to the "Network" tab, and then reload the page. Look through your requests
until you find one that has your "session" cookie. Copy the contents of
that cookie (the alphanumeric string after the equals sign) sans quotes to
a file called `.session` in this repository. Now you're ready to download
inputs from the terminal!

_Note: the `.session` file is .gitignored, so you don't accidentally upload_
_your login token to GitHub._

### Setting Up For a Day

The `prep-day.sh` script in the root of this repository will download your
input using `curl` to a .gitignored `.input` directory and then copy a
boilerplate module to `src/dayN.rs`, so you can start working (almost)
right away! Here's how to run it:

```bash
sh prep-day.sh <day>
```

The script will also update `main.rs` with the boilerplate needed to
actually run your solution.

## Running

To run your solutions for a day, run the following in the repo:

```bash
cargo run <day>
```

You should see something like the following:

```bash
$ cargo run 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target\debug\aoc-2024.exe 1`
Reading input data from .input/1.txt
Solving day 1...
Part 1: 2164381 (0.0002336 seconds)
Part 2: 20719933 (0.0089312 seconds)
```

## Test inputs

For development it often makes sense to work with smaller sample inputs
such as the ones showin in the AoC task descriptions. For that, you can
create a test input file in any location (e.g., `.input/day1_test.txt`)
and then pass it as a second parameter to the run command:

```bash
$ cargo run 1 .input/1_test.txt
   Compiling aoc-2024 v0.1.0 (C:\Users\bjoer\src\aoc)
    Finished dev [unoptimized + debuginfo] target(s) in 0.65s
     Running `target\debug\aoc-2024.exe 1 .input/1_test.txt`
Reading input data from .input/1_test.txt
Solving day 1...
Part 1: 11 (0.0000087 seconds)
Part 2: 31 (0.0000027 seconds)
```

## Questions

If you have any issues getting this up and running, you can make an
[issue on GitHub][make issue].


[aoc]: https://adventofcode.com/
[rust]: https://rust-lang.org
[install rust]: https://www.rust-lang.org/tools/install
[make issue]: https://github.com/bjoernd/aoc-2023/issues/new
