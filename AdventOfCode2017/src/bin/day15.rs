/// Advent of Code 2017, Day 15
/// Copyright 2023 by Alex Utter

use aocfetch;

// Scale factors for the two generators.
const SCALE_A: u64 = 16807;
const SCALE_B: u64 = 48271;

// Parse the text input to get the seeds.
fn parse(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.trim().lines().collect();
    let line0: Vec<&str> = lines[0].split(' ').collect();
    let line1: Vec<&str> = lines[1].split(' ').collect();
    return (line0[4].parse().unwrap(), line1[4].parse().unwrap());
}

// Linear congruential generator (LCG)
struct Lcg {
    seed: u64,
    scale: u64,
    modulo: u64,
}

impl Lcg {
    fn new(seed: u64, scale: u64) -> Lcg {
        Lcg { seed:seed, scale:scale, modulo:2147483647 }
    }

    fn next(&mut self) -> u16 {
        self.seed = (self.seed * self.scale) % self.modulo;
        return (self.seed & 0xFFFF) as u16;
    }

    fn skip(&mut self, mask: u16) -> u16 {
        loop {
            let x = self.next();
            if x & mask == 0 { return x; }
        }
    }
}

// Simulate two LCGs and count agreements.
fn part1(seed_a: u64, seed_b: u64) -> usize {
    let mut count = 0usize;
    let mut lcg_a = Lcg::new(seed_a, SCALE_A);
    let mut lcg_b = Lcg::new(seed_b, SCALE_B);
    for _ in 0..40000000 {
        if lcg_a.next() == lcg_b.next() { count += 1; }
    }
    return count
}

// Simulate two modified LCGs and count agreements.
fn part2(seed_a: u64, seed_b: u64) -> usize {
    let mut count = 0usize;
    let mut lcg_a = Lcg::new(seed_a, SCALE_A);
    let mut lcg_b = Lcg::new(seed_b, SCALE_B);
    for _ in 0..5000000 {
        if lcg_a.skip(0x3) == lcg_b.skip(0x7) { count += 1; }
    }
    return count
}

fn main() {
    // Fetch problem input from server.
    let input = parse(&aocfetch::get_data(2017, 15).unwrap());

    // Unit tests on provided examples.
    assert_eq!(part1(65, 8921), 588);
    assert_eq!(part2(65, 8921), 309);

    // Solve for real input.
    println!("Part 1: {}", part1(input.0, input.1));
    println!("Part 2: {}", part2(input.0, input.1));
}
