/// Advent of Code 2015, Day 5
/// Copyright 2023 by Alex Utter

use std::collections::HashMap;
use aocfetch;

fn is_nice1(input: &str) -> bool
{
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let bad_pairs = vec![('a','b'), ('c','d'), ('p','q'), ('x','y')];
    let mut ct_vowels = 0usize;
    let mut flag_bad = false;
    let mut flag_dbl = false;
    let mut c1 = ' ';
    for c2 in input.chars() {
        // Check each of the three rules.
        if vowels.contains(&c2) {ct_vowels += 1;}
        if c1 == c2 {flag_dbl = true;}
        if bad_pairs.contains(&(c1,c2)) {flag_bad = true;}
        // Keep a history of the previous letter.
        c1 = c2;
    }
    return (ct_vowels >= 3) && flag_dbl && !flag_bad;
}

fn is_nice2(input: &str) -> bool
{
    let mut pairs: HashMap<(char,char), usize> = HashMap::new();
    let mut flag_dbl = false;
    let mut flag_rpt = false;
    let mut c1 = ' ';
    let mut c2 = ' ';
    for (n, c3) in input.chars().enumerate() {
        // Have we already seen this letter-pair?
        if let Some(p) = pairs.get(&(c2,c3)) {
            if n > p + 1 {flag_dbl = true;}     // Non-overlapping?
        } else {
            pairs.insert((c2,c3), n);           // First time, store index
        }
        // Set the flag for repeated letters with a gap.
        if c1 == c3 {flag_rpt = true;}
        // Keep a history of the previous two letters.
        c1 = c2; c2 = c3;
    }
    return flag_dbl && flag_rpt;
}

fn part1(input: &str) -> usize
{
    input.lines().filter(|x| is_nice1(x)).count()
}

fn part2(input: &str) -> usize
{
    input.lines().filter(|x| is_nice2(x)).count()
}

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2015, 5).unwrap();

    // Unit tests on provided examples
    assert_eq!(is_nice1("ugknbfddgicrmopn"), true);
    assert_eq!(is_nice1("aaa"), true);
    assert_eq!(is_nice1("jchzalrnumimnmhp"), false);
    assert_eq!(is_nice1("haegwjzuvuyypxyu"), false);
    assert_eq!(is_nice1("dvszwmarrgswjxmb"), false);
    assert_eq!(is_nice2("aaa"), false);
    assert_eq!(is_nice2("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(is_nice2("xxyxx"), true);
    assert_eq!(is_nice2("uurcxstgmygtbstg"), false);
    assert_eq!(is_nice2("ieodomkazucvgmuy"), false);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
