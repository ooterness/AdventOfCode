/// Top-level dispatcher for my Advent Of Code 2021 solutions.
/// See also: https://adventofcode.com/2021/
///     To run a given day's solution: cargo run [day#]
/// Copyright 2021 by Alex Utter

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::env;
use std::time;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

// List of functions, one for each day's problem.
lazy_static! {
    static ref SOLUTIONS: HashMap<usize, fn()> = vec![
        (1,  day01::solve as fn()),
        (2,  day02::solve as fn()),
        (3,  day03::solve as fn()),
        (4,  day04::solve as fn()),
        (5,  day05::solve as fn()),
        (6,  day06::solve as fn()),
        (7,  day07::solve as fn()),
        (8,  day08::solve as fn()),
        (9,  day09::solve as fn()),
        (10, day10::solve as fn()),
        (11, day11::solve as fn()),
        (12, day12::solve as fn()),
    ].into_iter().collect();

    static ref EMPTY_STRING: String = String::from("");
}

// Wrapper that adds elapsed-time telemetry.
fn solve_timer(idx: &usize) -> u32 {
    if let Some(uut) = SOLUTIONS.get(idx) {
        println!("Starting Day {}", idx);
        let timer = time::Instant::now();
        uut();  // Run designated function
        let elapsed = timer.elapsed().as_millis();
        println!("Elapsed: {} msec.", elapsed);
        println!("");
        elapsed as u32
    } else {
        eprintln!("No solution for Day {}.", idx);
        0
    }
}

// Main entry point looks at command-line arguments.
fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1).unwrap_or(&EMPTY_STRING);

    if arg == "all" {
        // Solve every problem in the list.
        let mut elapsed = 0u32;
        for idx in SOLUTIONS.keys() {
            elapsed += solve_timer(idx);
        };
        println!("Total elapsed time {} msec.", elapsed);
    } else if let Result::Ok(idx) = arg.parse::<usize>() {
        solve_timer(&idx);
    } else {
        eprintln!("Usage: 'cargo run [day#]' or 'cargo run all'");
        eprintln!("  Where day# is the problem to be solved (1-25)");
    }
}
