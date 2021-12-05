/// Top-level dispatcher for my Advent Of Code 2021 solutions.
/// See also: https://adventofcode.com/2021/
///     To run a given day's solution: cargo run [day#]
/// Copyright 2021 by Alex Utter

use std::env;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(daynum) = args[1].parse::<usize>() {
        match daynum {
            1 => day01::solve(),
            2 => day02::solve(),
            3 => day03::solve(),
            4 => day04::solve(),
            5 => day05::solve(),
            _ => println!("No solution for Day {} yet.", daynum),
        }
    } else {
        eprintln!("Usage: cargo run [day#]");
        eprintln!("  Where day# is the problem to be solved (1-25)");
    }
}
