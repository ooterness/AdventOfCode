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
    static ref COMPLETE2POINTS: HashMap<char, u64> = vec!
        [(')',1), (']',2), ('}',3), ('>',4)]
        .into_iter().collect();
}

// Return points value for the first incorrect closing character.
fn syntax_score(line: &str) -> u64 {
    // For each opener, push matching closer onto stack.
    // Otherwise, pop from stack and see if it matches.
    let mut stack = Vec::<char>::new();
    for ch in line.chars() {
        if let Some(close) = OPEN2CLOSE.get(&ch) {
            stack.push(*close);
        } else if let Some(pts) = CLOSE2POINTS.get(&ch) {
            let expect = stack.pop().unwrap_or('*');
            if !(ch == expect) {return *pts;}
        }
    }
    return 0    // Reached end without a corrupted character
}

// Return score for completing an incomplete line.
fn complete_score(line: &str) -> Option<u64> {
    // For each opener, push matching closer onto stack.
    // Otherwise, pop from stack and see if it matches.
    let mut stack = Vec::<char>::new();
    for ch in line.chars() {
        if let Some(close) = OPEN2CLOSE.get(&ch) {
            stack.push(*close);
        } else {
            let expect = stack.pop().unwrap_or('*');
            if !(ch == expect) {return None;}
        }
    }
    // Anything leftover in the stack is our autocomplete sequence.
    let mut score = 0u64;
    while let Some(ch) = stack.pop() {
        score = 5*score + COMPLETE2POINTS.get(&ch).unwrap_or(&0);
    }
    Some(score)
}

// Part-1 score is the sum syntax scores for each corrupt line.
fn part1(lines: &Vec<String>) -> u64 {
    lines.iter().map(|x| syntax_score(&x)).sum()
}

// Part-2 score is the median of completion scores for each line.
fn part2(lines: &Vec<String>) -> u64 {
    let mut scores: Vec<u64> = lines.iter()
        .filter_map(|x| complete_score(&x)).collect();
    scores.sort();
    scores[(scores.len()-1)/2]
}

pub fn solve() {
    let test = common::read_lines("input/test10.txt");
    let data = common::read_lines("input/input10.txt");

    assert_eq!(part1(&test), 26397);
    println!("Part1: {}", part1(&data));

    assert_eq!(complete_score(&test[0]), Some(288957));
    assert_eq!(complete_score(&test[1]), Some(5566));
    assert_eq!(complete_score(&test[2]), None);
    assert_eq!(complete_score(&test[3]), Some(1480781));
    assert_eq!(complete_score(&test[6]), Some(995444));
    assert_eq!(complete_score(&test[9]), Some(294));
    assert_eq!(part2(&test), 288957);
    println!("Part2: {}", part2(&data));
}
