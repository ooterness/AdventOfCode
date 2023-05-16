/// Advent of Code 2016, Day 1
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

use std::collections::HashSet;

struct Direction {
    face: u8,
}

impl Direction {
    fn new() -> Self {
        Direction {face: 0}
    }

    fn turn(&mut self, turn: char) {
        let delta = if turn == 'R' {1} else {3};
        self.face = (self.face + delta) % 4;
    }

    fn xy(&self) -> (i64,i64) {
        match self.face {
            0 => (0,1),     // North
            1 => (1,0),     // East
            2 => (0,-1),    // South
            _ => (-1,0),    // West
        }
    }
}

struct Step {
    turn: char,
    dist: i64,
}

impl Step {
    fn new(input: &str) -> Self {
        let turn:char = input.chars().nth(0).unwrap();
        let dist:i64 = input[1..].parse().unwrap();
        Step {turn:turn, dist:dist}
    }
}

fn part1(input: &str) -> i64
{
    let mut d = Direction::new();   // Initial position
    let mut x = 0i64;
    let mut y = 0i64;
    for step in input.split(", ").map(Step::new) {
        d.turn(step.turn);          // Turn left or right
        x += d.xy().0 * step.dist;  // Move N steps
        y += d.xy().1 * step.dist;
    }
    return x.abs() + y.abs();       // Manhattan distance from start
}

fn part2(input: &str) -> i64
{
    let mut visit: HashSet<(i64,i64)> = HashSet::new();
    let mut d = Direction::new();   // Initial position
    let mut x = 0i64;
    let mut y = 0i64;
    visit.insert((x,y));
    for step in input.split(", ").map(Step::new) {
        d.turn(step.turn);          // Turn left or right
        for _ in 0..step.dist {     // Move N steps...
            x += d.xy().0;
            y += d.xy().1;
            if !visit.insert((x,y)) {
                return x.abs() + y.abs();
            }
        }
    }
    return 0;                       // No match found...
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 1).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1("R2, L3"), 5);
    assert_eq!(part1("R2, R2, R2"), 2);
    assert_eq!(part1("R5, L5, R5, R3"), 12);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
