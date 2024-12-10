/// Advent of Code 2024, Day 7
/// Copyright 2024 by Alex Utter

use aocfetch;

// What's the next-largest power of 10? e.g., 123 -> 1000
fn scale(x:u64) -> u64 {
    let mut rem = x;
    let mut out = 10u64;
    while rem >= 10 {
        rem /= 10; out *= 10;
    }
    return out;
}

struct Equation {
    lhs: u64,
    rhs: Vec<(u64,u64)>,
    opmax: usize,
}

impl Equation {
    // Parse one equation, e.g., "7290: 6 8 6 15"
    fn new(line: &str, opmax:usize) -> Self {
        let tok: Vec<u64> = line.trim().split([' ', ':'])
            .filter_map(|s| s.parse().ok()).collect();
        let rhs = tok[1..].iter()
            .map(|&x| (x, scale(x))).collect();
        return Equation {lhs:tok[0], rhs:rhs, opmax:opmax};
    }

    // Apply the specified operation to the accumulator.
    //  0 for "add", 1 for "multiply", or 2 for "concat".
    fn calc(&self, n:usize, op:usize, accum: u64) -> u64 {
        let (val, scl) = self.rhs[n];
        return match op {
            0 => accum + val,
            1 => accum * val,
            _ => accum * scl + val,
        }
    }

    // Try all possiblities for the next operation.
    fn search(&self, n:usize, accum: u64) -> bool {
        // Search completed? Compare against expected value.
        if n == self.rhs.len() {return self.lhs == accum;}
        // Terminate search early? Accumulator only increases.
        if accum > self.lhs {return false;}
        // Otherwise, continue depth-first search...
        return (self.opmax > 0 && self.search(n+1, self.calc(n, 0, accum)))
            || (self.opmax > 1 && self.search(n+1, self.calc(n, 1, accum)))
            || (self.opmax > 2 && self.search(n+1, self.calc(n, 2, accum)));
    }

    // Is there a sequence of operations where LHS = seq(RHS)?
    fn valid(&self) -> bool {
        return self.search(1, self.rhs[0].0);
    }
}

fn solve(input: &str, opmax: usize) -> u64 {
    let mut total = 0u64;
    for line in input.trim().lines() {
        let eq = Equation::new(line, opmax);
        if eq.valid() {total += eq.lhs;}
    }
    return total;
}

fn part1(input: &str) -> u64 {
    solve(input, 2)
}

fn part2(input: &str) -> u64 {
    solve(input, 3)
}

const EXAMPLE: &'static str = "\
    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 7).unwrap();

    assert_eq!(part1(EXAMPLE), 3749);
    assert_eq!(part2(EXAMPLE), 11387);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
