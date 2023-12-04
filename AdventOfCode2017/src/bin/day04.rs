/// Advent of Code 2017, Day 4
/// Copyright 2023 by Alex Utter

extern crate aocfetch;
use std::collections::HashSet;

// Is a given passphrase valid under Part 1 rules?
fn valid_p1(phrase: &str) -> bool
{
    let mut seen : HashSet<&str> = HashSet::new();
    for word in phrase.split(' ') {
        if seen.contains(word) { return false; }
        seen.insert(word);
    }
    return true;    // No duplicates
}

// Is a given passphrase valid under Part 2 rules?
fn valid_p2(phrase: &str) -> bool
{
    let mut seen : HashSet<String> = HashSet::new();
    for word in phrase.split(' ') {
        // Sort individual letters into a new string.
        let mut wvec: Vec<char> = word.chars().collect();
        wvec.sort();
        let wsort: String = wvec.into_iter().collect();
        // All anagrams have the same sorted string.
        if seen.contains(&wsort) { return false; }
        seen.insert(wsort);
    }
    return true;    // No duplicates
}

fn part1(input: &str) -> usize
{
    return input.lines().filter(|x| valid_p1(x)).count()
}

fn part2(input: &str) -> usize
{
    return input.lines().filter(|x| valid_p2(x)).count()
}

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 4).unwrap();

    // Unit tests on provided examples.
    assert!(valid_p1("aa bb cc dd ee"));
    assert!(!valid_p1("aa bb cc dd aa"));
    assert!(valid_p1("aa bb cc dd aaa"));
    assert!(valid_p1("aa bb cc dd ee"));
    assert!(!valid_p1("aa bb cc dd aa"));
    assert!(valid_p1("aa bb cc dd aaa"));
    assert!(valid_p2("abcde fghij"));
    assert!(!valid_p2("abcde xyz ecdab"));
    assert!(valid_p2("a ab abc abd abf abj"));
    assert!(valid_p2("iiii oiii ooii oooi oooo"));
    assert!(!valid_p2("oiii ioii iioi iiio"));

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
