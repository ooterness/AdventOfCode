/// Advent of Code 2015, Day 4
/// Copyright 2023 by Alex Utter

use md5;
#[path = "fetch.rs"] mod fetch;

fn md5_prefix(salt: &str, idx: usize) -> u32
{
    let blk = format!("{}{}", salt, idx);
    let hash = md5::compute(blk.as_bytes());
    return 65536 * hash[0] as u32   // Digits 1 & 2
         +   256 * hash[1] as u32   // Digits 3 & 4
         +         hash[2] as u32;  // Digits 5 & 6
}

fn solve(salt: &str, mask: u32) -> usize
{
    let mut guess = 1usize;
    while md5_prefix(salt, guess) & mask > 0 {guess += 1;}
    return guess;
}

fn part1(input: &str) -> usize
{
    solve(input, 0xFFFFF0u32)
}

fn part2(input: &str) -> usize
{
    solve(input, 0xFFFFFFu32)
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 4).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1("abcdef"), 609043);
    assert_eq!(part1("pqrstuv"), 1048970);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
