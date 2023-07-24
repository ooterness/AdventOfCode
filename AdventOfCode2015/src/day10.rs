/// Advent of Code 2015, Day 10
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

fn look_say(seed: &str) -> String {
    let mut result = String::new();
    let mut count = 0usize;
    let mut digit = seed.chars().nth(0).unwrap();
    for ch in seed.chars() {
        if ch == digit {
            count += 1;
        } else {
            result += &format!("{}", count);
            result.push(digit);
            digit = ch;
            count = 1;
        }
    }
    result += &format!("{}", count);
    result.push(digit);
    return result;
}

fn look_iter(seed: &str, iter: usize) -> String
{
    let mut result = seed.to_string();
    for _ in 0..iter {
        result = look_say(&result);
    }
    return result;
}

fn part1(input: &str) -> usize
{
    look_iter(input, 40).len()
}

fn part2(input: &str) -> usize
{
    look_iter(input, 50).len()
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 10).unwrap();

    // Unit tests on provided examples.
    assert_eq!(look_iter("1", 5), "312211");

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
