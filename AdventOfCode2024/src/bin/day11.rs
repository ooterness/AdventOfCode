/// Advent of Code 2024, Day 11
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;

#[derive(Clone)]
struct Stones {
    stones: HashMap<u64, u64>,
}

impl Stones {
    fn new() -> Self {
        return Stones{ stones:HashMap::new() };
    }

    fn parse(input: &str) -> Self {
        let mut result = Stones::new();
        for tok in input.trim().split(' ') {
            let val: u64 = tok.parse().unwrap();
            result.insert(val, 1);
        }
        return result;
    }

    fn insert(&mut self, val:u64, ct:u64) {
        *self.stones.entry(val).or_insert(0) += ct;
    }

    fn blink1(&self) -> Self {
        let mut next = Stones::new();
        for (&val, &ct) in self.stones.iter() {
            let dig = val.to_string().len() as u32;
            if val == 0 {
                next.insert(1, ct);
            } else if (dig % 2) == 0 {
                let split = 10u64.pow(dig/2);
                next.insert(val / split, ct);
                next.insert(val % split, ct);
            } else {
                next.insert(val * 2024, ct);
            }
        }
        return next;
    }

    fn blink(&self, n:usize) -> Self {
        let mut tmp = self.clone();
        for _ in 0..n {tmp = tmp.blink1();}
        return tmp;
    }

    fn count(&self) -> u64 {
        self.stones.values().sum()
    }
}

fn part1(input: &str) -> u64 {
    Stones::parse(input).blink(25).count()
}

fn part2(input: &str) -> u64 {
    Stones::parse(input).blink(75).count()
}


const EXAMPLE: &'static str = "125 17";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 11).unwrap();

    assert_eq!(part1(EXAMPLE), 55312);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
