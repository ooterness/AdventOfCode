/// Advent of Code 2023, Day 12
/// Copyright 2023 by Alex Utter

use aocfetch;

struct Puzzle {
    wmask: u64,
    mask0: u64,
    mask1: u64,
    runs: Vec<usize>,
}

impl Puzzle {
    fn new(line: &str) -> Self {
        let tok: Vec<&str> = line.trim().split(' ').collect();
        // First, parse the ".?#" mask, left = LSB.
        let width = tok[0].trim().chars().count();
        assert!(width < 64);
        let mut mask0 = 0u64;
        let mut mask1 = 0u64;
        for (n,ch) in tok[0].trim().chars().enumerate() {
            if ch == '.' {mask0 |= 1 << n;}
            if ch == '#' {mask1 |= 1 << n;}
        }
        // Parse the comma-delimited list of contiguous runs.
        let runs: Vec<usize> = tok[1].split(',')
            .map(|s| s.parse().unwrap()).collect();
        return Puzzle {wmask:1<<width, mask0:mask0, mask1:mask1, runs:runs};
    }

    fn consistent(&self, guess:u64) -> bool {
        // Bit-masks are easy, so check those first.
        if guess & self.mask0 != 0 {return false;}
        if guess & self.mask1 != self.mask1 {return false;}
        // State-machine to cross-check each contiguous run.
        let mut mask = 1u64; // Starting from LSB...
        for expected in self.runs.iter() {
            if mask >= self.wmask {return false;}
            // Skip any leading 0's.
            while (mask < self.wmask) && (guess & mask == 0) {mask <<= 1;}
            // Count contiguous 1's.
            let mut run = 0;
            while (mask < self.wmask) && (guess & mask != 0) {mask <<= 1; run += 1;}
            if run != *expected {return false;}
            // Must leave a gap between each run.
            mask <<= 1;
        }
        let rmask = !(mask-1);  // Remaining MSBs must all be zero.
        return (guess & rmask) == 0;
    }

    fn count(&self) -> usize {
        // Try each possible combination of inputs...
        return (0..self.wmask).filter(|g| self.consistent(*g)).count();
    }
}

fn part1(input: &str) -> usize {
    input.trim().lines().map(|s| Puzzle::new(s).count()).sum()
}

fn part2(input: &str) -> usize {
    0   // TODO
}

const EXAMPLE: &'static str = "\
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 12).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 21);
    assert_eq!(part2(EXAMPLE), 0);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
