/// Advent of Code 2017, Day 13
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;

struct Firewall {
    scanners: HashMap<u64, u64>,
    max_depth: u64,
}

impl Firewall {
    fn new(input: &str) -> Firewall {
        let mut scanners = HashMap::new();
        for line in input.trim().lines() {
            let words: Vec<&str> = line.split(':').collect();
            let depth = words[0].trim().parse().unwrap();
            let range = words[1].trim().parse().unwrap();
            scanners.insert(depth, range);
        }
        let max_depth = scanners.keys().max().unwrap() + 1;
        return Firewall { scanners, max_depth }
    }

    fn patrol(&self, depth: u64, time: u64) -> Option<u64> {
        if let Some(range) = self.scanners.get(&depth) {
            let tau = 2 * range - 2; // Back-and-forth period
            if time % tau == 0 { return Some(*range); }
        }
        return None
    }

    fn detected(&self, t0: u64) -> bool {
        for depth in 0..self.max_depth {
            if let Some(_) = self.patrol(depth, t0 + depth) {
                return true;
            }
        }
        return false;
    }

    fn severity(&self, t0: u64) -> u64 {
        let mut severity = 0u64;
        for depth in 0..self.max_depth {
            if let Some(range) = self.patrol(depth, t0 + depth) {
                severity += depth * range;
            }
        }
        return severity;
    }
}

fn part1(input: &str) -> u64 {
    let wall = Firewall::new(input);
    return wall.severity(0);
}

fn part2(input: &str) -> u64 {
    let wall = Firewall::new(input);
    for t in 0..10000000 {
        if !wall.detected(t) { return t }
    }
    return 0;
}

const TEST: &str = "0: 3\n1: 2\n4: 4\n6: 4";

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 13).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(TEST), 24);
    assert_eq!(part2(TEST), 10);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
