/// Advent of Code 2015, Day 8
/// Copyright 2023 by Alex Utter

use aocfetch;

// Decoder state for the "escape" function.
enum Decode {
    Begin,
    Normal,
    Escape,
    Hex1,
    Hex2(char),
}

// Convert a pair of hex characters into its ASCII representation.
fn hex_esc(a: char, b: char) -> char
{
    let ascii = 16 * a.to_digit(16).unwrap() + b.to_digit(16).unwrap();
    if ascii < 128 {
        return char::from_u32(ascii).unwrap();
    } else {
        return 'X'; // Extended characters are not valid UTF-8
    }
}

// Decode a string containing escape characters.
// e.g., "aaa\"aaa" -> aaa\aaa
fn escape(input: &str) -> String
{
    let mut result = String::new();
    let mut state = Decode::Begin;
    for ch in input.chars() {
        state = match (state, ch) {
            (Decode::Begin, '"')     => {Decode::Normal}, // Start of string
            (Decode::Begin, _)       => {Decode::Begin},  // Waiting
            (Decode::Normal, '"')    => {Decode::Begin},  // End of string
            (Decode::Normal, '\\')   => {Decode::Escape}, // Escape seq ("\\")
            (Decode::Normal, _)      => {result.push(ch); Decode::Normal},
            (Decode::Escape, 'x')    => {Decode::Hex1},   // Hex-escape ("\x27")
            (Decode::Escape, _)      => {result.push(ch); Decode::Normal},
            (Decode::Hex1, _)        => {Decode::Hex2(ch)},
            (Decode::Hex2(x), _)     => {result.push(hex_esc(x,ch)); Decode::Normal},
        };
    }
    return result;
}

// Encode a string using escape characters.
// e.g., "aaa\"aaa" -> "\"aaa\\\"aaa\""
fn encode(input: &str) -> String
{
    let mut result = String::new();
    result.push('"');
    for ch in input.chars() {
        match ch {
            '"'     => result += "\\\"",
            '\\'    => result += "\\\\",
            _       => result.push(ch),
        };
    }
    result.push('"');
    return result;
}

fn part1(input: &str) -> usize
{
    let mut total = 0usize;
    for line in input.lines() {
        let x = line.trim();
        let y = escape(x);
        total += x.len() - y.len();
    }
    return total;
}

fn part2(input: &str) -> usize
{
    let mut total = 0usize;
    for line in input.lines() {
        let x = line.trim();
        let y = encode(x);
        total += y.len() - x.len();
    }
    return total;
}

// Unit tests.
const TEST1: &str = "\"\"";             // ""
const TEST2: &str = "\"abc\"";          // "abc"
const TEST3: &str = "\"aaa\\\"aaa\"";   // "aaa\"aaa"
const TEST4: &str = "\"\\x27\"";        // "\x27"
const TEST5: &str = "\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"\n";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2015, 8).unwrap();

    // Unit tests on provided examples.
    assert_eq!(escape(TEST1), "");
    assert_eq!(escape(TEST2), "abc");
    assert_eq!(escape(TEST3), "aaa\"aaa");
    assert_eq!(escape(TEST4), "'");
    assert_eq!(part1(TEST5), 12);
    assert_eq!(encode(TEST1).len(), 6);
    assert_eq!(encode(TEST2).len(), 9);
    assert_eq!(encode(TEST3).len(), 16);
    assert_eq!(encode(TEST4).len(), 11);
    assert_eq!(part2(TEST5), 19);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
