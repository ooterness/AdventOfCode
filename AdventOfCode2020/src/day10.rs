/// Day 10: https://adventofcode.com/2020/day/10
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

/// When using every adapter, count +1 and +3 steps.
fn part1(list: &Vec<usize>) -> u64 {
    // Sort the input list.
    let mut seq = list.clone();
    seq.sort();
    // Count steps of +1 and +3.
    let mut prev   = 0usize;
    let mut count1 = 0u64;
    let mut count3 = 1u64;  // Final output always +3
    for x in seq {
        match x - prev {
            1 => Some(count1 += 1),
            3 => Some(count3 += 1),
            _ => None,
        };
        prev = x;
    }
    count1 * count3
}

/// Count total number of possible adapter paths.
fn part2(list: &Vec<usize>) -> u64 {
    // Sort the input list and find maximum.
    let mut seq = list.clone();
    seq.sort();
    let max:usize = *seq.iter().max().unwrap();
    // Memoize number of paths from 0 to exactly X.
    let mut memo:Vec<u64> = vec![0u64; max+1usize];
    memo[0] = 1u64;
    // For each adapter, accept the preceding three options.
    for x in seq {
        let d1:u64 = if x > 0 {memo[x-1]} else {0};
        let d2:u64 = if x > 1 {memo[x-2]} else {0};
        let d3:u64 = if x > 2 {memo[x-3]} else {0};
        memo[x] = d1 + d2 + d3;
    }
    memo[max]
}

pub fn solve() {
    let example1:Vec<usize> = vec![
        16,10,15,5,1,11,7,19,6,12,4];
    let example2:Vec<usize> = vec![
        28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,
        38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3];

    // Test against example inputs.
    assert_eq!(part1(&example1), 35u64);
    assert_eq!(part1(&example2), 220u64);
    assert_eq!(part2(&example1), 8u64);
    assert_eq!(part2(&example2), 19208u64);

    // Read and solve main input
    let input:Vec<usize> = common::read_integers("input/input10.txt")
        .iter().map(|x| *x as usize).collect();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
