/// Day 1: https://adventofcode.com/2021/day/1
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

// Three-element sliding window.
struct SlidingFilter {
    p1: Option<u64>,
    p2: Option<u64>,
}

impl SlidingFilter {
    fn new() -> SlidingFilter {
        SlidingFilter {p1:None, p2:None}
    }

    fn next(&mut self, x: &u64) -> Option<u64> {
        // Do we have enough inputs to emit a new result?
        let mut result : Option<u64> = None;
        if let (Some(y), Some(z)) = (self.p1, self.p2) {
            result = Some(*x + y + z);
        }
        // Update internal state before returning.
        self.p2 = self.p1;
        self.p1 = Some(*x);
        return result
    }
}


fn filter(invec: &Vec<u64>) -> Vec<u64> {
    let mut filt = SlidingFilter::new();
    return invec.iter().filter_map(|x| filt.next(x)).collect()
}

// Count the number of times the depth increases.
fn count_increase(x: &Vec<u64>) -> u64 {
    let mut count = 0u64;
    let mut prev : Option<u64> = None;
    for depth in x.iter() {
        if let Some(pdepth) = prev {
            if depth > &pdepth {
                count += 1;
            }
        }
        prev = Some(*depth);
    }
    return count
}

// Part-1 solution (raw data)
fn part1(filename: &str) -> u64 {
    // Load input from file (one integer per line)
    let x = common::read_lines_as::<u64>(filename);
    return count_increase(&x)
}

// Part-2 solution (filtered data)
fn part2(filename: &str) -> u64 {
    // Load input from file (one integer per line)
    let x = common::read_lines_as::<u64>(filename);
    return count_increase(&filter(&x))
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(part1("input/test01.txt"), 7);
    println!("Part 1: {}", part1("input/input01.txt"));

    // Test part-2 solver, then apply to real input.
    assert_eq!(part2("input/test01.txt"), 5);
    println!("Part 2: {}", part2("input/input01.txt"));
}
