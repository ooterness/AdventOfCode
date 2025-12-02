/// Advent of Code 2025, Day 1
/// Copyright 2025 by Alex Utter

use aocfetch;

type CmdVec = Vec<i64>;

// Convert a single command "L68" or "R48" to a number.
fn parse_one(word: &str) -> Result<i64, &str> {
    let cmd = word.chars().nth(0).ok_or("Empty")?;
    let num = word[1..].parse::<i64>().or(Err("Bad distance"))?;
    if cmd == 'L' { return Ok(-num); }  // Left = Negative
    if cmd == 'R' { return Ok( num); }  // Right = Positive
    return Err("Bad direction");
}

fn parse(input: &str) -> CmdVec {
    return input.trim().split_whitespace()
        .filter_map(|w| parse_one(w).ok())
        .collect();
}

fn run(cmds: &CmdVec) -> (usize, usize) {
    let mut count1 = 0usize;    // Stop at zero?
    let mut count2 = 0usize;    // Pass zero?
    let mut dial = 50i64;
    for cmd in cmds.iter() {
        let dist = if dial == 0 {100}
            else if *cmd < 0 {dial} else {100-dial};
        if cmd.abs() >= dist {
            let extra = (cmd.abs() - dist) / 100;
            count2 += 1usize + extra as usize;
        }
        dial = (dial + cmd).rem_euclid(100);
        if dial == 0 { count1 += 1; }
    }
    return (count1, count2);
}

fn part1(cmds: &CmdVec) -> usize {
    run(cmds).0
}

fn part2(cmds: &CmdVec) -> usize {
    run(cmds).1
}

const EXAMPLE1: &'static str = "\
    L68 L30 R48 L5 R60 L55 L1 L99 R14 L82";
const EXAMPLE2: &'static str = "\
    R1000 L50 R100 L200 R200 R1 R99 L1 L99 R1 R100 L100 L102 L99";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 1).unwrap();

    assert_eq!(part1(&parse(EXAMPLE1)), 3);
    assert_eq!(part1(&parse(EXAMPLE2)), 7);
    assert_eq!(part2(&parse(EXAMPLE1)), 6);
    assert_eq!(part2(&parse(EXAMPLE2)), 23);

    let data = parse(&input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
