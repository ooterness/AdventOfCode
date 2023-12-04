/// Advent of Code 2023, Day 4
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

// Read a set of space-delimited numbers.
fn read_numbers(input: &str) -> HashSet<u64> {
    HashSet::from_iter(input.trim().split(' ').filter_map(|x| x.parse().ok()))
}

// Read each part of the card and count the number of matching items.
fn read_line(line: &str) -> usize {
    let sub: Vec<&str> = line.split(&[':', '|']).collect();
    assert_eq!(sub.len(), 3);
    let ll = read_numbers(sub[1]);
    let rr = read_numbers(sub[2]);
    return ll.intersection(&rr).count();
}

// Solve using Part-1 rules: sum(2**n)
fn part1(input: &str) -> u64 {
    let mut points = 0u64;
    for count in input.trim().lines().map(read_line) {
        if count > 0 {points += 1u64 << (count-1);}
    }
    return points;
}

// Solve using Part-2 rules: Win additional cards
fn part2(input: &str) -> usize {
    let mut cards = HashMap::<usize,usize>::new();
    let mut total = 0usize;
    for (n, win) in input.trim().lines().map(read_line).enumerate() {
        // How many copies of the current card?
        let count = *cards.get(&n).unwrap_or(&1);
        total += count;
        // Add new copies of the next N cards.
        for m in 1..=win {
            *cards.entry(n+m).or_insert(1) += count;
        }
    }
    return total;
}

const EXAMPLE: &'static str = "\
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 4).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 13);
    assert_eq!(part2(EXAMPLE), 30);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
