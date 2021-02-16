/// Day 23: https://adventofcode.com/2020/day/23
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

struct Cups {
    len:  usize,        // Number of cups
    curr: usize,        // Label for current cup
    next: Vec<usize>,   // Label of next cup, indexed by label
}

impl Cups {
    /// Create initial state from string.
    fn new(s: &str, len:usize) -> Cups {
        // Initialize circular linked list for positions 0 through N-1.
        let mut next: Vec<usize> = (0..len).map(|n| (n+1) % len).collect();
        // Overwrite initial state as dictated by string.
        let mut start = 0usize;
        let mut prev = len - 1;
        for (n,c) in String::from(s).chars().enumerate() {
            let d = (c.to_digit(10).unwrap() - 1) as usize;
            if n == 0 {start = d;}  // Index of first cup
            next[prev] = d;         // Link previous cup to this one
            prev = d;
        }
        // Special case for end of initial state...
        if s.len() < len {
            next[prev] = s.len();   // Link to rest of list
        } else {
            next[prev] = start;     // Wrap to beginning
        }
        // Construct the complete object.
        Cups {len:len, curr:start, next:next}
    }

    /// Given current state, find destination label.
    fn dest(&self) -> usize {
        // Initial state: "pick up" next three cups.
        let cur = self.curr;
        let up1 = self.next[cur];
        let up2 = self.next[up1];
        let up3 = self.next[up2];
        // Calculate the destination label.
        let decr = |n| if n > 0 {n-1} else {self.len-1};
        let mut dst = decr(cur);
        while (dst == up1) || (dst == up2) || (dst == up3) {
            dst = decr(dst);
        }
        dst
    }

    fn iter_1(&mut self) {
        // Find the destination label.
        let dst = self.dest();
        // Note label of the first four cups.
        let cur = self.curr;
        let up1 = self.next[cur];
        let up2 = self.next[up1];
        let up3 = self.next[up2];
        // Update the linked-list.
        self.next[cur] = self.next[up3];
        self.next[up3] = self.next[dst];
        self.next[dst] = up1;
        // Update current cup index.
        self.curr = self.next[cur];
    }

    fn iter_n(&mut self, iter:usize) {
        for _ in 0..iter {self.iter_1();}
    }

    fn product(&self) -> u64 {
        let cup1 = self.next[0];
        let cup2 = self.next[cup1];
        (cup1 as u64 + 1u64) * (cup2 as u64 + 1u64)
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut next = self.next[0];
        for _ in 1..9 {
            result += &(next+1).to_string();
            next = self.next[next];
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
