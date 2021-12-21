/// Day 21: https://adventofcode.com/2021/day/21
/// Copyright 2021 by Alex Utter

use std::cmp::min;

// Given starting position, play game with deterministic dice.
fn part1(start: &(u64,u64)) -> u64 {
    // Set initial game state.
    let mut die = 0u64;
    let mut pos = (start.0-1, start.1-1);
    let mut pts = (0u64, 0u64);
    // Keep playing until one player reaches target score.
    while pts.0 < 1000 && pts.1 < 1000 {
        // Roll the die three times.
        let incr = (3*die+1) + (3*die+2) + (3*die+3);
        // Move pieces and update points.
        if die % 2 == 0 {
            pos.0 = (pos.0 + incr) % 10;
            pts.0 += pos.0 + 1;
        } else {
            pos.1 = (pos.1 + incr) % 10;
            pts.1 += pos.1 + 1;
        }
        // Increment state of deterministic die.
        die += 1;
    }
    3 * die * min(pts.0, pts.1)
}

pub fn solve() {
    assert_eq!(part1(&(4,8)), 739785);
    println!("Part1: {}", part1(&(7,4)));
}
