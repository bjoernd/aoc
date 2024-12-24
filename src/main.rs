use std::env;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};
#[macro_use]
extern crate num_derive;
mod day1;
use day1::Day1;
mod day2;
use day2::Day2;
mod day3;
use day3::Day3;
mod day4;
use day4::Day4;
mod day5;
use day5::Day5;
mod day6;
use day6::Day6;
mod day7;
use day7::Day7;
mod day8;
use day8::Day8;
mod day9;
use day9::Day9;
mod day10;
use day10::Day10;
mod day11;
use day11::Day11;
mod day12;
use day12::Day12;
mod day13;
use day13::Day13;
mod day14;
use day14::Day14;
mod day15;
use day15::Day15;
mod day16;
use day16::Day16;
mod day17;
use day17::Day17;
// MOD_MARKER

/// Reads the lines from the input file into a relevant
/// model of the data for the day's solution.
trait FromInput {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self;
}

/// Solutions for a day of Advent of Code.
trait DaySolution {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

/// Reads the input for a day from the `.input` directory.
fn load_input(in_file: &String) -> impl Iterator<Item = String> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(in_file)
        .expect("Failed to access data in in_file");
    let buffered_file = BufReader::new(file);

    buffered_file
        .lines()
        .map(|line| line.expect("Failed to read line from data file"))
}

/// Gets the solution for the given day as a trait object.
fn get_day_solution(day: usize, lines: impl Iterator<Item = String>) -> Box<dyn DaySolution> {
    match day {
        1 => Box::new(Day1::from_lines(lines)),
        2 => Box::new(Day2::from_lines(lines)),
        3 => Box::new(Day3::from_lines(lines)),
        4 => Box::new(Day4::from_lines(lines)),
        5 => Box::new(Day5::from_lines(lines)),
        6 => Box::new(Day6::from_lines(lines)),
        7 => Box::new(Day7::from_lines(lines)),
        8 => Box::new(Day8::from_lines(lines)),
        9 => Box::new(Day9::from_lines(lines)),
        10 => Box::new(Day10::from_lines(lines)),
        11 => Box::new(Day11::from_lines(lines)),
        12 => Box::new(Day12::from_lines(lines)),
        13 => Box::new(Day13::from_lines(lines)),
        14 => Box::new(Day14::from_lines(lines)),
        15 => Box::new(Day15::from_lines(lines)),
        16 => Box::new(Day16::from_lines(lines)),
        17 => Box::new(Day17::from_lines(lines)),
        // DAY_MARKER
        _other => panic!("Day hasn't been solved yet"),
    }
}

/// Times the execution of a function.
fn time_execution(work: impl FnOnce() -> String) -> (String, Duration) {
    let start = Instant::now();
    let result = work();
    let duration = start.elapsed();

    (result, duration)
}

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Must provide a day to solve")
        .parse::<usize>()
        .expect("Provided day wasn't a valid integer");

    let in_file = match env::args().nth(2) {
        None => {
            format!(".input/{day}.txt")
        }
        Some(s) => s,
    };

    println!("Reading input data from {in_file}");
    let input = load_input(&in_file);
    let solution = get_day_solution(day, input);
    println!("Solving day {day}...");

    let (part_one, duration) = time_execution(|| solution.part_one());
    println!("Part 1: {part_one} ({} seconds)", duration.as_secs_f32());

    let (part_two, duration) = time_execution(|| solution.part_two());
    println!("Part 2: {part_two} ({} seconds)", duration.as_secs_f32());
}
