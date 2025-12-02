/// Advent of Code 2025, Day 1
/// Copyright 2025 by Alex Utter

use aocfetch;
use std::collections::HashSet;

type Range = (usize, usize);
type RVec = Vec<Range>;
type RSet = HashSet<usize>;

fn parse_one(input: &str) -> Result<Range, &str> {
    let pair: Vec<&str> = input.trim().split('-').collect();
    let lo: usize = pair[0].parse::<usize>().or(Err("BadNum"))?;
    let hi: usize = pair[1].parse::<usize>().or(Err("BadNum"))?;
    return Ok((lo, hi));
}

fn parse(input: &str) -> RVec {
    input.trim().split(',')
        .filter_map(|w| parse_one(w).ok())
        .collect()
}

// Length of integer "n" when converted to a base-10 string.
fn strlen(n: usize) -> (usize,usize) {
    let mut acc = 10usize;
    let mut len = 1usize;
    while n >= acc {
        acc *= 10;
        len += 1;
    }
    return (len, acc);
}

// Given a seed, repeat it N times: repeat(123, 2) -> 123123.
fn repeat(seed: usize, rpt: usize) -> usize {
    let scale = strlen(seed).1;     // 123 -> 1000 (10^N)
    let mut acc = seed;
    for _ in 1..rpt { acc = scale*acc + seed; }
    return acc;
}

// Given number of repetitions, scan for invalid IDs in a range.
fn scan(range: &Range, rpt: usize, ids: &mut RSet) {
    let (range_lo, range_hi) = range.clone();
    let len_base = strlen(range_lo).0;
    let len_diff = len_base - len_base / rpt;
    let mut seed = range_lo / 10usize.pow(len_diff as u32);
    loop {
        let tmp = repeat(seed, rpt);
        if tmp > range_hi {break;}
        seed += 1;
        if tmp < range_lo {continue;}
        // println!("{} x {} = {}", seed-1, rpt, tmp);
        ids.insert(tmp);
    }
}

fn part1(input: &RVec) -> usize {
    let mut total = 0usize;
    for range in input.iter() {
        let mut ids = RSet::new();
        scan(range, 2, &mut ids);
        total += ids.into_iter().sum::<usize>();
    }
    return total;
}

fn part2(input: &RVec) -> usize {
    let mut total = 0usize;
    for range in input.iter() {
        let max_rpt = strlen(range.1).0;
        let mut ids = RSet::new();
        for rpt in 2..=max_rpt {
            scan(range, rpt, &mut ids);
        }
        total += ids.into_iter().sum::<usize>();
    }
    return total;
}

const EXAMPLE: &'static str = "\
    11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    824824821-824824827,2121212118-2121212124";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 2).unwrap();

    assert_eq!(part1(&parse(EXAMPLE)), 1227775554);
    assert_eq!(part2(&parse(EXAMPLE)), 4174379265);

    let data = parse(&input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
