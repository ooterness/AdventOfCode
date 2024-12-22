/// Advent of Code 2024, Day 22
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;

type Sequence = (i64, i64, i64, i64);
type ProfitMap = HashMap<Sequence, i64>;

struct Prng {
    x: i64,
}

impl Prng {
    fn new(init: &str) -> Self {
        Prng { x : init.trim().parse::<i64>().unwrap() }
    }

    fn step(&mut self) -> i64 {
        const MASK: i64 = 0xFFFFFF;
        self.x ^= (self.x * 64) & MASK;
        self.x ^= (self.x / 32) & MASK;
        self.x ^= (self.x * 2048) & MASK;
        return self.x;
    }

    fn step_many(&mut self, n: usize) -> i64 {
        for _ in 0..n {self.step();}
        return self.x;
    }

    fn prices(&mut self, n: usize) -> Vec<i64> {
        let mut seq = Vec::new();
        for _ in 0..n {seq.push(self.step() % 10);}
        return seq;
    }

    fn profit(&mut self, n: usize) -> ProfitMap {
        let prices = self.prices(n);
        let mut profit = HashMap::new();
        for m in 4..n {
            let seq: Sequence = (
                prices[m-3] - prices[m-4],
                prices[m-2] - prices[m-3],
                prices[m-1] - prices[m-2],
                prices[m-0] - prices[m-1]);
            profit.entry(seq).or_insert(prices[m]);
        }
        return profit;
    }
}

fn part1(input: &str) -> i64 {
    input.trim().lines().map(|x| Prng::new(x).step_many(2000)).sum()
}

fn part2(input:&str) -> i64 {
    let mut profit = ProfitMap::new();
    for line in input.trim().lines() {
        for (seq, val) in Prng::new(line).profit(2000).into_iter() {
            *profit.entry(seq).or_insert(0) += val;
        }
    }
    return *profit.values().max().unwrap();
}

const EXAMPLE1: &'static str = "1\n10\n100\n2024";
const EXAMPLE2: &'static str = "1\n2\n3\n2024";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 22).unwrap();

    assert_eq!(part1(EXAMPLE1), 37327623);
    assert_eq!(part2(EXAMPLE2), 23);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
