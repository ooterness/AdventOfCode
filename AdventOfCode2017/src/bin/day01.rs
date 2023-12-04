/// Advent of Code 2017, Day 1
/// Copyright 2023 by Alex Utter

extern crate aocfetch;

fn to_digit(x: &char) -> u64
{
    return u64::from(x.to_digit(10).unwrap())
}

fn part1(input: &str) -> u64
{
    let mut sum = 0u64;
    let mut prev = input.chars().last().unwrap();
    for next in input.chars() {
        if prev == next { sum += to_digit(&next); }
        prev = next
    }
    return sum
}

fn part2(input: &str) -> u64
{
    let mut sum = 0u64;
    let ivec: Vec<u64> = input.chars().map(|x| to_digit(&x)).collect();
    let half = ivec.len() / 2;
    for n in 0..half {
        if ivec[n] == ivec[n+half] {sum += ivec[n];}
    }
    return 2*sum
}

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2017, 1).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1("1122"), 3);
    assert_eq!(part1("1111"), 4);
    assert_eq!(part1("1234"), 0);
    assert_eq!(part1("91212129"), 9);
    assert_eq!(part2("1212"), 6);
    assert_eq!(part2("1221"), 0);
    assert_eq!(part2("123425"), 4);
    assert_eq!(part2("123123"), 12);
    assert_eq!(part2("12131415"), 4);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
