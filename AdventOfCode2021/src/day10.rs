/// Day 10: https://adventofcode.com/2021/day/10
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::collections::HashMap;

// Using library for on-startup runtime initialization...
lazy_static! {
    static ref OPEN2CLOSE: HashMap<char, char> = vec!
        [('(',')'), ('[',']'), ('{','}'), ('<','>')]
        .into_iter().collect();
    static ref CLOSE2POINTS: HashMap<char, u64> = vec!
        [(')',3), (']',57), ('}',1197), ('>',25137)]
        .into_iter().collect();
}

// Return points value for the first incorrect closing character.
fn syntax_score(line: &str) -> u64 {
    let mut stack = Vec::<char>::new();
    for ch in line.chars() {
        // For each opener, push matching closer onto stack.
        // Otherwise, pop from stack and see if it matches.
        if let Some(close) = OPEN2CLOSE.get(&ch) {
            stack.push(*close);
        } else if let Some(pts) = CLOSE2POINTS.get(&ch) {
            let expect = stack.pop().unwrap_or('*');
            if !(ch == expect) {return *pts;}
        }
    }
    return 0    // Reached end without a corrupted character
}

// Part-1 score is the sum of all syntax scores.
fn part1(lines: &Vec<String>) -> u64 {
    lines.iter().map(|x| syntax_score(&x)).sum()
}

pub fn solve() {
    let test = common::read_lines("input/test10.txt");
    let data = common::read_lines("input/input10.txt");

    assert_eq!(part1(&test), 26397);
    println!("Part1: {}", part1(&data));
}
