/// Day 23: https://adventofcode.com/2020/day/23
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

#[derive(Clone)]
struct Cups(Vec<usize>);

impl Cups {
    /// Create initial state from string.
    fn new(s: &str) -> Cups {
        let s = String::from(s);
        Cups(s.chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as usize).collect())
    }

    /// Find cup index with the given label.
    fn find(&self, lbl:&usize) -> usize {
        self.0.iter().position(|x| x == lbl).unwrap()
    }

    /// Given current state, find destination index.
    fn dest(&self) -> usize {
        // Initial state: "pick up" next three cups.
        let len = self.0.len();
        let cur = self.0[0];
        let up0 = self.0[1];
        let up1 = self.0[2];
        let up2 = self.0[3];
        // Calculate the destination label.
        let decr = |n| if n > 1 {n - 1} else {len};
        let mut dst = decr(cur);
        while (dst == up0) || (dst == up1) || (dst == up2) {
            dst = decr(dst);
        }
        // Return index matching that label.
        self.find(&dst)
    }

    fn iter_1(&self) -> Cups {
        // Find the destination cup-index.
        let dst = self.dest();
        // Pick up three cups and move them just after destination.
        let mut cups = Vec::new();
        for n in self.0[4..=dst].iter() {cups.push(*n);}
        cups.push(self.0[1]);
        cups.push(self.0[2]);
        cups.push(self.0[3]);
        for n in self.0[dst+1..].iter() {cups.push(*n);}
        // Finally, the original starting cup.
        cups.push(self.0[0]);
        Cups(cups)
    }

    fn iter_n(&self, iter:usize) -> Cups {
        let mut cups = self.clone();
        for _ in 0..iter {cups = cups.iter_1()}
        cups
    }

    fn to_string(&self) -> String {
        let start = self.find(&1);
        let mut result = String::new();
        for n in 1..self.0.len() {
            let idx = (start + n) % self.0.len();
            result += &self.0[idx].to_string();
        }
        result
    }
}

pub fn solve() {
    let example = Cups::new("389125467");
    let input   = Cups::new("716892543");

    assert_eq!(example.iter_n(0).to_string(), "25467389");
    assert_eq!(example.iter_n(1).to_string(), "54673289");
    assert_eq!(example.iter_n(2).to_string(), "32546789");
    assert_eq!(example.iter_n(3).to_string(), "34672589");
    assert_eq!(example.iter_n(4).to_string(), "32584679");
    assert_eq!(example.iter_n(5).to_string(), "36792584");
    assert_eq!(example.iter_n(6).to_string(), "93672584");
    assert_eq!(example.iter_n(7).to_string(), "92583674");
    assert_eq!(example.iter_n(8).to_string(), "58392674");
    assert_eq!(example.iter_n(9).to_string(), "83926574");
    assert_eq!(example.iter_n(10).to_string(), "92658374");
    assert_eq!(example.iter_n(100).to_string(), "67384529");

    println!("Part1: {}", input.iter_n(100).to_string());
}
