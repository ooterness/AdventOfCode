/// Day 25: https://adventofcode.com/2020/day/25
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

#[derive(Clone)]
struct Key {
    subj:   u64,
    key:    u64,
    iter:   usize,
}

impl Key {
    fn new(subj:u64) -> Key {
        Key {subj:subj, key:1, iter:0}
    }

    fn iter(&mut self, n:usize) {
        for _ in 0..n {
            self.key = (self.key * self.subj) % 20201227u64;
            self.iter += 1;
        }
    }

    fn solve(val:u64) -> Key {
        let mut key = Key::new(7);
        while key.key != val {key.iter(1);}
        key
    }

    fn merge(pub1:&Key, pub2:&Key) -> Key {
        let mut key = Key::new(pub2.key);
        key.iter(pub1.iter);
        key
    }
}

pub fn solve() {
    // Solve the example.
    let test1a = Key::solve(5764801);           // Card public key
    let test1b = Key::solve(17807724);          // Door public key
    let test1c = Key::merge(&test1a, &test1b);  // Encryption key
    assert_eq!(test1a.iter, 8);
    assert_eq!(test1b.iter, 11);
    assert_eq!(test1c.key, 14897079u64);

    // Solve Part-1.
    let part1a = Key::solve(10212254);
    let part1b = Key::solve(12577395);
    let part1c = Key::merge(&part1a, &part1b);
    println!("Part1: {}", part1c.key);
}
