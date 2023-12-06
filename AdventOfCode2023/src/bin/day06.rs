/// Advent of Code 2023, Day 6
/// Copyright 2023 by Alex Utter

use aocfetch;

fn parse_single(line: &str) -> Vec<usize> {
    let mut total = 0usize;
    for ch in line.trim().chars() {
        if let Some(d) = ch.to_digit(10) {total = 10*total + d as usize;}
    }
    return vec![total];
}

fn parse_many(line: &str) -> Vec<usize> {
    line.trim().split(' ')              // Split on spaces
        .filter(|s| !s.is_empty())      // Ignore consecutive spaces
        .skip(1)                        // Skip the header/label
        .map(|s| s.parse().unwrap())    // Parse each number
        .collect()
}

fn parse(input: &str, part2: bool) -> Vec<(usize, usize)> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let myfun = if part2 {parse_single} else {parse_many};
    let line0 = myfun(lines[0]);
    let line1 = myfun(lines[1]);
    line0.into_iter().zip(line1.into_iter()).collect()
}

// Find the number of discrete solutions to "(t-x)*x > d", where:
//  * d is the minimum distance to travel.
//  * t is the duration of the race.
//  * x is the duration to hold the button.
fn count(t: usize, d: usize) -> usize {
    // Quadratic equation solving "x^2 - kx + d+1 = 0":
    let xmin = 0.5 * (t as f64 - ((t*t - 4*d - 4) as f64).sqrt());
    let xmax = 0.5 * (t as f64 + ((t*t - 4*d - 4) as f64).sqrt());
    return 1 + (xmax.floor() as usize) - (xmin.ceil() as usize);
}

fn part1(input: &str) -> usize {
    parse(input, false).into_iter().map(|(k,d)| count(k,d)).product()
}

fn part2(input: &str) -> usize {
    parse(input, true).into_iter().map(|(k,d)| count(k,d)).product()
}

const EXAMPLE: &'static str = "\
    Time:      7  15   30
    Distance:  9  40  200";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 6).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 288);
    assert_eq!(part2(EXAMPLE), 71503);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
