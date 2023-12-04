/// Advent of Code 2015, Day 20
/// Copyright 2023 by Alex Utter

use aocfetch;

fn div_round_up(x: usize, y: usize) -> usize {
    (x + y - 1) / y
}

fn first_above(v: &Vec<usize>, min: usize) -> usize {
    for (n, &x) in v.iter().enumerate() {
        if x >= min {return n;}
    }
    return 0; // No solution
}

fn simulate(min_score: usize, max_visit: usize) -> Vec<usize> {
    // Worst case N is prime, so the only factors are 1 and N.
    // (And maybe just N, if the first elf stops early.)
    let max_house = min_score - (min_score > max_visit) as usize;

    // Simulate visits to each house.
    let mut score = vec![0; max_house+1];
    let mut f = 1usize;
    while f <= max_house {      // For each factor (i.e., elf number)
        let mut m = 1;          // Visit every multiple of F...
        while (m*f <= max_house) && (m <= max_visit) {
            score[m*f] += f;
            m += 1;
        }
        f += 1;
    }
    return score;
}

fn part1(input: &str) -> usize {
    let min_score: usize = div_round_up(input.parse().unwrap(), 10);
    let score = simulate(min_score, usize::MAX);
    return first_above(&score, min_score);
}

fn part2(input: &str) -> usize {
    let min_score: usize = div_round_up(input.parse().unwrap(), 11);
    let score = simulate(min_score, 50);
    return first_above(&score, min_score);
}

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2015, 20).unwrap();

    // Unit tests based on the provided examples:
    assert_eq!(part1("70"), 4);
    assert_eq!(part1("71"), 6);
    assert_eq!(part1("130"), 8);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
