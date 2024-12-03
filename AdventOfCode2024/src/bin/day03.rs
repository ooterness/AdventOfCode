/// Advent of Code 2024, Day 3
/// Copyright 2024 by Alex Utter

use aocfetch;

type Mul = (i64, i64);

fn parse(input: &str, part2: bool) -> Vec<Mul> {
    let mut result = Vec::new();
    let mut left  = 0i64;
    let mut right = 0i64;
    let mut state = 0usize;
    let mut keep  = true;
    for ch in input.trim().chars() {
        state = match(state, ch, ch.is_digit(10)) {
            (_, 'm', _)     => {1},     // Start of "mul(123,456)" or similar
            (1, 'u', _)     => {2},
            (2, 'l', _)     => {3},
            (3, '(', _)     => {4},
            (4, _, true)    => {left = 10*left + ch.to_digit(10).unwrap() as i64; 4},
            (4, ',', _)     => {5},
            (5, _, true)    => {right = 10*right + ch.to_digit(10).unwrap() as i64; 5},
            (5, ')', _)     => {6},     // End of "mul" instruction
            (_, 'd', _)     => {7},     // Start of "do()" or "don't()"
            (7, 'o', _)     => {8},
            (8, '(', _)     => {9},
            (9, ')', _)     => {10},    // End of "do()"
            (8, 'n', _)     => {11},
            (11, '\'', _)   => {12},
            (12, 't', _)    => {13},
            (13, '(', _)    => {14},
            (14, ')', _)    => {15},    // End of "don't()"
            _               => {0},     // Unexpected character -> Reset
        };
        // Complete multiply instruction, keep if valid.
        if state == 6 && keep && left < 1000 && right < 1000 {
            result.push((left, right));
        }
        // Complete "do" or "don't" instructions.
        if part2 && state == 10 {keep = true;}
        if part2 && state == 15 {keep = false;}
        // Reset parsing state?
        if state == 0 || state == 6 {
            left  = 0;
            right = 0;
            state = 0;
        }
    }
    return result;
}

fn part1(input: &str) -> i64 {
    parse(input, false).iter()
        .map(|(a,b)| a * b)
        .sum()
}

fn part2(input: &str) -> i64 {
    parse(input, true).iter()
        .map(|(a,b)| a * b)
        .sum()
}

const EXAMPLE1: &'static str = "\
    xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const EXAMPLE2: &'static str = "\
    xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 3).unwrap();

    assert_eq!(part1("mul(44,46)"), 2024);
    assert_eq!(part1("mul(123,4)"), 123*4);
    assert_eq!(part1("mul(4*"), 0);
    assert_eq!(part1("mul(6,9!"), 0);
    assert_eq!(part1("?(12,34)"), 0);
    assert_eq!(part1("mul ( 2 , 4 )"), 0);
    assert_eq!(part1(EXAMPLE1), 161);
    assert_eq!(part2(EXAMPLE2), 48);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
