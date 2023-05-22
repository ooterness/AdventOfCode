/// Advent of Code 2016, Day 15
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

#[derive(Clone)]
struct DiskState {
    depth: usize,
    period: usize,
    phase: usize,
}

const INIT: DiskState = DiskState {depth: 0, period: 1, phase: 0};

impl DiskState {
    fn new(line: &str) -> Self {
        let tokens: Vec<&str> = line.trim().split([' ', '#', '.']).collect();
        DiskState {
            depth:  tokens[2].parse().unwrap(),
            period: tokens[4].parse().unwrap(),
            phase:  tokens[12].parse().unwrap(),
        }
    }

    fn add(&mut self, next: &Self) {
        // Advance time in steps until we find a solution.
        while (self.phase + next.phase + next.depth) % next.period != 0 {
            self.phase += self.period;
        }
        // Update the least-common multiple of the aggregate solution.
        // Note: Input periods are always mutually prime, no reduce LCM.
        self.period *= next.period;
        self.depth   = next.depth;
    }
}

fn solve(input: &str) -> DiskState {
    let mut state = INIT.clone();
    for line in input.trim().lines() {
        state.add(&DiskState::new(line));
    }
    return state;
}

fn part1(input: &str) -> usize {
    solve(input).phase
}

fn part2(input: &str) -> usize {
    let mut soln = solve(input);
    soln.add(&DiskState {depth: soln.depth+1, period: 11, phase: 0});
    return soln.phase
}

const TEST: &str = "\
Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 15).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), 5);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
