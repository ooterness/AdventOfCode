/// Day 23: https://adventofcode.com/2020/day/23
/// Copyright 2021 by Alex Utter

use std::cmp::min;
#[path = "common.rs"] mod common;

struct Cups {
    curr: usize,        // Current index
    cups: Vec<usize>,   // Label for cup by index
    posn: Vec<usize>,   // Position of cup by label
}

impl Cups {
    /// Create initial state from string.
    fn new(s: &str, len:usize) -> Cups {
        // Initialize labels and positions 0 through N-1.
        let mut cups: Vec<usize> = (0..len).collect();
        let mut posn: Vec<usize> = (0..len).collect();
        // Overwrite initial state as dictated by string.
        for (n,c) in String::from(s).chars().enumerate() {
            let d = (c.to_digit(10).unwrap() - 1) as usize;
            cups[n] = d;
            posn[d] = n;
        }
        Cups {curr:0, cups:cups, posn:posn}
    }

    /// Fetch number of cups.
    fn len(&self) -> usize {
        self.cups.len()
    }

    /// Fetch index of Nth cup from current position.
    fn idx(&self, offset:usize) -> usize {
        (self.curr + offset) % self.len()
    }

    /// Fetch value of Nth cup from current position.
    fn val(&self, offset:usize) -> usize {
        self.cups[self.idx(offset)]
    }

    /// Given current state, find destination index.
    fn dest(&self) -> usize {
        // Initial state: "pick up" next three cups.
        let cur = self.val(0);
        let up1 = self.val(1);
        let up2 = self.val(2);
        let up3 = self.val(3);
        // Calculate the destination label.
        let decr = |n| if n > 0 {n-1} else {self.len()-1};
        let mut dst = decr(cur);
        while (dst == up1) || (dst == up2) || (dst == up3) {
            dst = decr(dst);
        }
        // Return index relative to current position.
        (self.posn[dst] + self.len() - self.curr) % self.len()
    }

    fn iter_1(&mut self) {
        // Find the destination position.
        let rel = self.dest();
        // Note value of the three cups being picked up.
        let val1 = self.val(1);
        let val2 = self.val(2);
        let val3 = self.val(3);
        let pos1 = self.idx(rel-2);
        let pos2 = self.idx(rel-1);
        let pos3 = self.idx(rel-0);
        // From that point to destination, move cups foward three steps.
        for n in 4..=rel {
            let pos = self.idx(n-3);    // New position
            let val = self.val(n);      // Label / value
            self.cups[pos] = val;
            self.posn[val] = pos;
        }
        // Deposit the cups that were picked up.
        self.cups[pos1] = val1;
        self.cups[pos2] = val2;
        self.cups[pos3] = val3;
        self.posn[val1] = pos1;
        self.posn[val2] = pos2;
        self.posn[val3] = pos3;
        // Update current cup index.
        self.curr = self.idx(1);
    }

    fn iter_n(&mut self, iter:usize) {
        for _ in 0..iter {self.iter_1();}
    }

    fn product(&self) -> u64 {
        let start = self.posn[0];
        let cup1 = self.cups[(start+1) % self.len()] as u64;
        let cup2 = self.cups[(start+2) % self.len()] as u64;
        (cup1+1) * (cup2+1)
    }

    fn to_string(&self) -> String {
        let start = self.posn[0];
        let maxln = min(self.len(), 9usize);
        let mut result = String::new();
        for n in 1..maxln {
            let idx = (start + n) % self.cups.len();
            let val = self.cups[idx] + 1usize;
            result += &val.to_string();
        }
        result
    }
}

pub fn solve() {
    let mut test1  = Cups::new("389125467", 9);
    assert_eq!(test1.to_string(), "25467389");                      // Step 0
    test1.iter_1(); assert_eq!(test1.to_string(), "54673289");      // Step 1
    test1.iter_1(); assert_eq!(test1.to_string(), "32546789");      // Step 2
    test1.iter_1(); assert_eq!(test1.to_string(), "34672589");      // Step 3
    test1.iter_1(); assert_eq!(test1.to_string(), "32584679");      // Step 4
    test1.iter_1(); assert_eq!(test1.to_string(), "36792584");      // Step 5
    test1.iter_1(); assert_eq!(test1.to_string(), "93672584");      // Step 6
    test1.iter_1(); assert_eq!(test1.to_string(), "92583674");      // Step 7
    test1.iter_1(); assert_eq!(test1.to_string(), "58392674");      // Step 8
    test1.iter_1(); assert_eq!(test1.to_string(), "83926574");      // Step 9
    test1.iter_1(); assert_eq!(test1.to_string(), "92658374");      // Step 10
    assert_eq!(test1.product(), 18);
    test1.iter_n(90); assert_eq!(test1.to_string(), "67384529");    // Step 100
    assert_eq!(test1.product(), 42);

    let mut test2  = Cups::new("389125467", 1_000_000);
    test2.iter_n(10_000_000);
    assert_eq!(test2.product(), 149245887792u64);

    let mut input1 = Cups::new("716892543", 9);
    input1.iter_n(100);
    println!("Part1: {}", input1.to_string());

    let mut input2 = Cups::new("716892543", 1_000_000);
    input2.iter_n(10_000_000);
    println!("Part2: {}", input2.product());
}
