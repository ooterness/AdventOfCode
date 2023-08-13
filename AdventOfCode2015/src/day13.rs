/// Advent of Code 2015, Day 13
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashMap;

struct Permutation {
    idx: Vec<usize>,
}

impl Permutation {
    fn new(n: usize) -> Self {
        Permutation { idx: (0..n).collect() }
    }

    fn len(&self) -> usize {
        self.idx.len()
    }

    // Pandita algorithm: https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
    fn next(&mut self) -> bool {
        // Find the largest index k such that a[k] < a[k+1].
        let len = self.len();
        let mut k: usize = len;
        for n in 0..len-2 { if self.idx[n] < self.idx[n+1] {k = n;} }
        // If no such index exists, the permutation is the last permutation.
        if k == len {return false;}
        // Find the largest index l greater than k such that a[k] < a[l].
        let mut l: usize = len;
        for n in k+1..len-1 { if self.idx[k] < self.idx[n] {l = n;} }
        // Swap the value of a[k] with that of a[l].
        (self.idx[k], self.idx[l]) = (self.idx[l], self.idx[k]);
        // Reverse the sequence from a[k+1] up to and including the final element a[n].
        let r = (len - k - 1) / 2;
        for n in 1..=r {(self.idx[k+n], self.idx[len-n]) = (self.idx[len-n], self.idx[k+n]);}
        return true;
    }
}

struct Diners {
    labels: HashMap<String, usize>,
    diners: Vec<HashMap<usize,i64>>,
}

impl Diners {
    fn new(input: &str) -> Self {
        let mut result = Diners {
            labels: HashMap::new(),
            diners: Vec::new() };
        for line in input.trim().lines() {
            let words: Vec<&str> = line.trim().split([' ', '.']).collect();
            assert_eq!(words.len(), 12);
            let idx1 = result.get_diner(words[0]);
            let idx2 = result.get_diner(words[10]);
            let gain: i64 = if words[2] == "gain" {1} else {-1};
            let val: i64 = words[3].parse().unwrap();
            result.diners[idx1].insert(idx2, val * gain);
        }
        return result;
    }

    fn get_diner(&mut self, label: &str) -> usize {
        if let Some(idx) = self.labels.get(label) {
            return *idx;
        } else {
            let new_idx = self.labels.len();
            self.labels.insert(String::from(label), new_idx);
            self.diners.push(HashMap::new());
            return new_idx;
        }
    }

    fn len(&self) -> usize {
        self.labels.len()
    }

    fn score(&self, order: &Permutation) -> i64 {
        let len = order.len();
        let mut total = 0i64;
        for n in 0..len {
            let nn = order.idx[n];
            let ll = order.idx[(n + len - 1) % len];
            let rr = order.idx[(n + 1) % len];
            if nn < self.len() {
                total += self.diners[nn].get(&ll).unwrap_or(&0);
                total += self.diners[nn].get(&rr).unwrap_or(&0);
            }
        }
        return total;
    }

    fn best_score(&self, num_guests: usize) -> i64 {
        let mut score = i64::MIN;
        let mut order = Permutation::new(num_guests);
        loop {
            score = i64::max(score, self.score(&order));
            if !order.next() {break;}
        }
        return score;
    }
}

fn part1(input: &str) -> i64
{
    let diners = Diners::new(input);
    return diners.best_score(diners.len());
}

fn part2(input: &str) -> i64
{
    let diners = Diners::new(input);
    return diners.best_score(diners.len() + 1);
}

const TEST: &str = "\
    Alice would gain 54 happiness units by sitting next to Bob.
    Alice would lose 79 happiness units by sitting next to Carol.
    Alice would lose 2 happiness units by sitting next to David.
    Bob would gain 83 happiness units by sitting next to Alice.
    Bob would lose 7 happiness units by sitting next to Carol.
    Bob would lose 63 happiness units by sitting next to David.
    Carol would lose 62 happiness units by sitting next to Alice.
    Carol would gain 60 happiness units by sitting next to Bob.
    Carol would gain 55 happiness units by sitting next to David.
    David would gain 46 happiness units by sitting next to Alice.
    David would lose 7 happiness units by sitting next to Bob.
    David would gain 41 happiness units by sitting next to Carol.";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 13).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(TEST), 330);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
