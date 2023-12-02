/// Advent of Code 2023, Day 1
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

// Parse a single numeric digit.
fn get_digit(ch: char) -> Option<i64>
{
    const BASE: u32 = '0' as u32;
    if ch.is_digit(10) {
        Some((ch as u32 - BASE) as i64)
    } else {None}
}

// Does a substring start or end in a numeric word?
// TODO: "zero" is not actually a legal input, but doesn't occur in my input.
const NUMBERS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine"];

fn word_left(input: &str) -> Option<i64>
{
    for (n, lbl) in NUMBERS.iter().enumerate() {
        if input.starts_with(lbl) {return Some(n as i64);}
    }
    return None;
}

fn word_right(input: &str) -> Option<i64>
{
    for (n, lbl) in NUMBERS.iter().enumerate() {
        if input.ends_with(lbl) {return Some(n as i64);}
    }
    return None;
}

// Get the first and last digit in a line of text.
// If the "allow_words" flag is set, also check for "one", "two", etc.
fn digits(input: &str, allow_words: bool) -> i64
{
    let mut d0 = -1i64;                 // Leftmost digit
    let mut d1 = -1i64;                 // Rightmost digit
    for (n, ch) in input.chars().enumerate() {
        if let Some(dd) = get_digit(ch) {
            if d0 < 0 {d0 = dd};        // First in sequence
            d1 = dd;                    // Last in sequence
        } else if allow_words {
            if d0 < 0 {
                if let Some(dd) = word_left(&input[n..]) {d0 = dd;}
            }
            if let Some(dd) = word_right(&input[..n+1]) {d1 = dd;}
        }
    }
    if d0 >= 0 && d1 >= 0 {
        return 10*d0 + d1;
    } else {
        return 0;
    }
}

fn part1(input: &str) -> i64
{
    input.lines().map(|line| digits(line, false)).sum()
}

fn part2(input: &str) -> i64
{
    input.lines().map(|line| digits(line, true)).sum()
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2023, 1).unwrap();

    // Unit tests on provided examples
    assert_eq!(digits("1abc2", false),              12);
    assert_eq!(digits("pqr3stu8vwx", false),        38);
    assert_eq!(digits("a1b2c3d4e5f", false),        15);
    assert_eq!(digits("treb7uchet", false),         77);
    assert_eq!(digits("two1nine", true),            29);
    assert_eq!(digits("eightwothree", true),        83);
    assert_eq!(digits("abcone2threexyz", true),     13);
    assert_eq!(digits("xtwone3four", true),         24);
    assert_eq!(digits("4nineeightseven2", true),    42);
    assert_eq!(digits("zoneight234", true),         14);
    assert_eq!(digits("7pqrstsixteen", true),       76);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
