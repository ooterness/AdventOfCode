/// Day 15: https://adventofcode.com/2020/day/15
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
#[path = "common.rs"] mod common;

struct History {
    hist: HashMap<usize, usize>,
    last: usize,
    step: usize,
}

impl History {
    /// Initialize from a comma-delimited history string.
    fn new(start: &str) -> History {
        let mut hist:HashMap<usize,usize> = HashMap::new();
        let mut last:usize = 0;
        let mut step:usize = 0;
        for x in start.split(',') {
            if let Ok(x) = x.parse::<usize>() {
                if step > 0 {hist.insert(last, step);}
                last  = x;
                step += 1;
            }
        }
        History {hist:hist, last:last, step:step}
    }

    /// Calculate the next spoken number.
    fn next(&mut self) -> usize {
        // Have we seen the previous number before?
        let mut next:usize = 0;
        if let Some(prev) = self.hist.get(&self.last) {
            next = self.step - prev;
        }
        // Update stored state.
        self.hist.insert(self.last, self.step);
        self.last  = next;
        self.step += 1;
        next
    }

    /// Iterate to Nth step return the last spoken number.
    fn iter(&mut self, step:usize) -> usize {
        while self.step < step {
            self.next();
        }
        self.last
    }
}

pub fn solve() {
    // Test each of the examples.
    let mut example1 = History::new("0,3,6");
    assert_eq!(example1.next(), 0); // Turn 4
    assert_eq!(example1.next(), 3); // Turn 5
    assert_eq!(example1.next(), 3); // Turn 6
    assert_eq!(example1.next(), 1); // Turn 7
    assert_eq!(example1.next(), 0); // Turn 8
    assert_eq!(example1.next(), 4); // Turn 9
    assert_eq!(example1.next(), 0); // Turn 10
    assert_eq!(example1.iter(2020), 436);

    let mut example2 = History::new("1,3,2");
    assert_eq!(example2.iter(2020), 1);

    let mut example3 = History::new("2,1,3");
    assert_eq!(example3.iter(2020), 10);

    let mut example4 = History::new("1,2,3");
    assert_eq!(example4.iter(2020), 27);

    let mut example5 = History::new("2,3,1");
    assert_eq!(example5.iter(2020), 78);

    let mut example6 = History::new("3,2,1");
    assert_eq!(example6.iter(2020), 438);

    let mut example7 = History::new("3,1,2");
    assert_eq!(example7.iter(2020), 1836);

    // Part-1 solution.
    let mut input = History::new("19,0,5,1,10,13");
    println!("Part1: {}", input.iter(2020));

    // Extended tests.
    assert_eq!(example1.iter(30000000), 175594);
    assert_eq!(example2.iter(30000000), 2578);
    assert_eq!(example3.iter(30000000), 3544142);
    assert_eq!(example4.iter(30000000), 261214);
    assert_eq!(example5.iter(30000000), 6895259);
    assert_eq!(example6.iter(30000000), 18);
    assert_eq!(example7.iter(30000000), 362);

    // Part-2 solution.
    println!("Part2: {}", input.iter(30000000));
}
