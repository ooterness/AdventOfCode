/// Advent of Code 2017, Day 6
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;

type MemState = Vec<usize>;

fn read_input(input: &str) -> MemState
{
    return input.trim().split('\t')         // Tab-delimited list
        .filter_map(|x| x.parse().ok())     // Keep each valid integer
        .collect();
}

fn balance(mem: &mut MemState)
{
    // Find the location of the largest memory bank.
    let mut max = 0usize;
    for n in 0..mem.len() {
        if mem[n] > mem[max] { max = n; }
    }
    // Clear and redistribute its contents.
    let incr_all = mem[max] / mem.len();
    let incr_rem = mem[max] % mem.len();
    mem[max] = 0;
    for n in 0..mem.len() {
        let extra = if n < incr_rem {1} else {0};
        let index = (max + n + 1) % mem.len();
        mem[index] += incr_all + extra;
    }
}

fn solve(input: &MemState) -> (usize, usize)
{
    let mut next = input.clone();
    let mut seen: HashMap<MemState,usize> = HashMap::new();
    loop {
        seen.insert(next.clone(), seen.len());
        balance(&mut next);
        if seen.contains_key(&next) { break; }
    }
    return (seen.len(), seen.len() - seen[&next]);
}

fn main() {
    // Fetch problem input from server.
    let test: MemState  = vec![0, 2, 7, 0];
    let input: MemState = read_input(&aocfetch::get_data(2017, 6).unwrap());

    // Unit tests on provided example.
    assert_eq!(solve(&test), (5, 4));

    // Solve for real input.
    println!("Part 1: {}", solve(&input).0);
    println!("Part 2: {}", solve(&input).1);
}
