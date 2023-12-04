/// Advent of Code 2015, Day 1
/// Copyright 2023 by Alex Utter

use aocfetch;

fn part1(input: &str) -> i64
{
    let mut floor = 0i64;
    for ch in input.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => (),
        }
    }
    return floor;
}

fn part2(input: &str) -> usize
{
    let mut floor = 0i64;
    for (n,ch) in input.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => (),
        }
        if floor < 0 {return n+1}
    }
    return 0;   // No solution?
}

fn main() {
    // Fetch input from server.
    let input = aocaocfetch::get_data(2015, 1).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1("(())"), 0);
    assert_eq!(part1("()()"), 0);
    assert_eq!(part1("((("), 3);
    assert_eq!(part1("(()(()("), 3);
    assert_eq!(part1("))((((("), 3);
    assert_eq!(part1("())"), -1);
    assert_eq!(part1("))("), -1);
    assert_eq!(part1(")))"), -3);
    assert_eq!(part1(")())())"), -3);
    assert_eq!(part2(")"), 1);
    assert_eq!(part2("()())"), 5);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
