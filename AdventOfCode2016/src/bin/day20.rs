/// Advent of Code 2016, Day 20
/// Copyright 2023 by Alex Utter

use aocfetch;

#[derive(Clone, Copy)]
struct Range(u64, u64);

const ADDR_END: u64 = 1u64 << 32;

impl Range {
    fn new(line: &str) -> Self {
        let tokens: Vec<&str> = line.split('-').collect();
        Range(tokens[0].parse().unwrap(),
              tokens[1].parse().unwrap())
    }

    fn contains(&self, x: u64) -> bool {
        self.0 <= x && x <= self.1
    }
}

struct Ruleset {
    rules0: Vec<Range>,     // Rules sorted by lower bound
    rules1: Vec<Range>,     // Rules sorted by upper bound
}

impl Ruleset {
    fn new(input: &str) -> Self {
        let tmp: Vec<Range> = input.trim().lines()
            .map(Range::new).collect();
        let mut rules0 = tmp.clone();
        let mut rules1 = tmp.clone();
        rules0.sort_by(|a,b| a.0.cmp(&b.0));
        rules1.sort_by(|a,b| a.1.cmp(&b.1));
        Ruleset {rules0:rules0, rules1:rules1}
    }

    // Is a given address blocked?
    fn blocked(&self, addr: u64) -> bool {
        self.rules0.iter().any(|r| r.contains(addr))
    }

    // Find next allowed address x >= addr.
    fn next_allow(&self, addr: u64) -> u64 {
        // Sorting by upper bound ensures we only need a single pass.
        let mut guess = addr;
        for rule in self.rules1.iter() {
            if rule.contains(guess) {guess = rule.1 + 1;}
        }
        return guess;
    }

    // Find next blocked address x >= addr.
    fn next_block(&self, addr: u64) -> u64 {
        // Sorting by lower bound ensures we only need a single pass.
        if self.blocked(addr) {return addr;}
        for rule in self.rules0.iter() {
            if rule.1 >= addr {return rule.0;}
        }
        return ADDR_END;
    }
}

// Find the first valid address.
fn part1(input: &str) -> u64 {
    let rules = Ruleset::new(input);
    return rules.next_allow(0);
}

// Find the total number of valid addresses.
fn part2(input: &str) -> u64 {
    let rules = Ruleset::new(input);
    let mut count = 0u64;
    let mut lower = rules.next_allow(0);
    loop {
        let upper = rules.next_block(lower);
        count += upper - lower;
        if upper < ADDR_END {
            lower = rules.next_allow(upper);
        } else {
            return count;
        }
    }
}

const TEST: &str = "5-8\n0-2\n4-7";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 20).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), 3);
    assert_eq!(part2(TEST), 4294967288);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
