/// Advent of Code 2016, Day 16
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

struct BitString {
    bits: Vec<u8>,
}

impl BitString {
    fn new(x: &str) -> Self {
        let bits: Vec<u8> = x.trim().chars()
            .map(|ch| if ch == '1' {1} else {0})
            .collect();
        BitString { bits: bits }
    }

    fn len(&self) -> usize {
        self.bits.len()
    }

    fn even(&self) -> bool {
        self.bits.len() % 2 == 0
    }

    fn expand(&self) -> Self {
        let mut next: Vec<u8> = Vec::with_capacity(1 + 2*self.len());
        for b in self.bits.iter() {next.push(*b);}
        next.push(0);
        for b in self.bits.iter().rev() {next.push(1-b);}
        return BitString {bits: next};
    }

    fn contract(&self) -> Self {
        assert!(self.even());
        let new_len = self.len() / 2;
        let mut next: Vec<u8> = Vec::with_capacity(new_len);
        for n in 0..new_len {
            let a = self.bits[2*n+0];
            let b = self.bits[2*n+1];
            next.push(if a == b {1} else {0});
        }
        return BitString {bits: next};
    }

    fn to_string(&self) -> String {
        self.bits.iter()
            .map(|&b| if b > 0 {'1'} else {'0'})
            .collect()
    }
}

fn solve(input: &str, disk_size: usize) -> String {
    let mut tmp = BitString::new(input);
    while tmp.len() < disk_size {tmp = tmp.expand();}
    tmp.bits.truncate(disk_size);
    while tmp.even() {tmp = tmp.contract();}
    return tmp.to_string();
}

fn part1(input: &str) -> String {
    solve(input.trim(), 272)
}

fn part2(input: &str) -> String {
    solve(input.trim(), 35651584)
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 16).unwrap();

    // Unit tests on provided examples
    assert_eq!(
        BitString::new("111100001010").expand().to_string(),
        "1111000010100101011110000");
    assert_eq!(solve("110010110100", 12), "100");
    assert_eq!(solve("10000", 20), "01100");

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
