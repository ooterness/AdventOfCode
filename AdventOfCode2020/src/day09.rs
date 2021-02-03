/// Day 9: https://adventofcode.com/2020/day/9
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
use std::collections::VecDeque;
#[path = "common.rs"] mod common;

struct XmasCode {
    wind: usize,                // Window / history size
    hist: VecDeque<i64>,        // History of last N inputs
    next: HashMap<i64,usize>,   // Legal values for next input
}

impl XmasCode {
    fn new(wind: usize) -> XmasCode {
        XmasCode {
            wind: wind,
            hist: VecDeque::new(),
            next: HashMap::new(),
        }
    }
    
    fn hlookup(&self, x:i64) -> usize {
        if let Some(n) = self.next.get(&x) {*n} else {0}
    }

    fn legal(&self, x:i64) -> bool {
        (self.hist.len() < self.wind) || (self.hlookup(x) > 0)
    }

    fn add(&mut self, x:i64) {
        // If we're out of the amble phase, remove old pairings.
        if self.hist.len() >= self.wind {
            let x = self.hist.pop_front().unwrap();
            for y in self.hist.iter() {
                let z:i64 = x + y;
                self.next.insert(z, self.hlookup(z) - 1);
            }
        }
        // Now add all the new pairings.
        for y in self.hist.iter() {
            let z:i64 = x + y;
            self.next.insert(z, self.hlookup(z) + 1);
        }
        self.hist.push_back(x);
    }
}

fn find_badinput(wind: usize, input: &Vec<i64>) -> Option<i64> {
    let mut code = XmasCode::new(wind);
    for x in input {
        if !code.legal(*x) {return Some(*x);} else {code.add(*x);}
    }
    None
}

fn find_weakness(target: i64, input: &Vec<i64>) -> Option<i64> {
    // Use cumulative sum to quickly check every contiguous range.
    let csum = common::cumsum(&input);
    for a in 0..csum.len()-1 {
        for b in a+1..csum.len() {
            if target == csum[b] - csum[a] {
                // Found a matching set. Get min/max.
                let slice = &input[a..=b];
                return Some(slice.iter().min().unwrap()
                          + slice.iter().max().unwrap());
            }
        }
    }
    None
}

pub fn solve() {
    // Solve the 5-number example:
    let example:Vec<i64> = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
    assert_eq!(find_badinput(5, &example), Some(127));
    assert_eq!(find_weakness(127, &example), Some(62));

    // Solve the main problem:
    let input = common::read_integers("input/input09.txt");
    if let Some(x) = find_badinput(25, &input) {
        println!("Part1: BadInput = {}", x);
        if let Some(y) = find_weakness(x, &input) {
            println!("Part2: Weakness = {}", y);
        } else {
            eprintln!("Part2: No solution.");
        }
    } else {
        eprintln!("Part1: No solution.");
    }
}
