/// Top-level dispatcher for my Advent Of Code 2021 solutions.
/// See also: https://adventofcode.com/2021/
///     To run a given day's solution: cargo run [day#]
/// Copyright 2021 by Alex Utter

#[macro_use]
extern crate lazy_static;

use std::env;
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(daynum) = args[1].parse::<usize>() {
        match daynum {
            1 => day01::solve(),
            2 => day02::solve(),
            3 => day03::solve(),
            4 => day04::solve(),
            5 => day05::solve(),
            6 => day06::solve(),
            7 => day07::solve(),
            8 => day08::solve(),
            9 => day09::solve(),
            10 => day10::solve(),
            11 => day11::solve(),
            _ => println!("No solution for Day {} yet.", daynum),
        }
    } else {
        eprintln!("Usage: cargo run [day#]");
        eprintln!("  Where day# is the problem to be solved (1-25)");
    }
}
