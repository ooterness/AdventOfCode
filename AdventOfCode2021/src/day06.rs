/// Day 6: https://adventofcode.com/2021/day/6
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

// Reproductive timing:
const DAYS_SPAWN0: usize = 9;   // Days from birth to first spawning
const DAYS_SPAWN1: usize = 7;   // Days between subsequent spawnings

// State vector counts fish with each possible timer state.
struct FishCount {
    count: Vec<u64>,
}

impl FishCount {
    // Read a comma-delimited string of fish-timers.
    fn new(line: &str) -> FishCount {
        let mut count = vec![0u64; DAYS_SPAWN0];
        for idx in common::split_str_as::<usize>(line, ',').into_iter() {
            assert!(idx < DAYS_SPAWN0); // Invalid input?
            count[idx] += 1;            // Increment fish-count
        }
        FishCount {count:count}
    }

    // Increment state by one day.
    fn next(&self) -> FishCount {
        let mut count = vec![0u64; DAYS_SPAWN0];
        for n in 0..DAYS_SPAWN0-1 {
            count[n] = self.count[n+1];
        }
        count[DAYS_SPAWN0-1] += self.count[0];
        count[DAYS_SPAWN1-1] += self.count[0];
        FishCount {count:count}
    }

    // Increment state by N days.
    fn advance(&self, n: usize) -> FishCount {
        if n == 1 {self.next()} else {self.advance(n-1).next()}
    }

    // Total fish this day?
    fn total(&self) -> u64 {
        self.count.iter().sum()
    }
}

fn read_file(filename: &str) -> Vec<FishCount> {
    let lines = common::read_lines(filename);
    lines.iter().map(|line| FishCount::new(line)).collect()
}

pub fn solve() {
    // Test reference has expected state for Day 0, Day 1, ...
    let test = read_file("input/test06.txt");
    for n in 1..test.len() {
        assert_eq!(test[n].total(), test[n-1].next().total());
        assert_eq!(test[n].total(), test[0].advance(n).total());
    }
    assert_eq!(test[0].advance(80).total(), 5934);
    assert_eq!(test[0].advance(256).total(), 26984457539);

    // Real input.
    let data = read_file("input/input06.txt");
    println!("Part1: {}", data[0].advance(80).total());
    println!("Part2: {}", data[0].advance(256).total());
}
