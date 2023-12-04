/// Advent of Code 2017, Day 5
/// Copyright 2023 by Alex Utter

use aocfetch;

type Program = Vec<i64>;

fn read_input(input: &str) -> Program
{
    return input.lines().map(|x| x.parse().unwrap()).collect();
}

fn part1(input: &Program) -> usize
{
    let mut prog    = input.clone();
    let mut count   = 0usize;
    let mut index   = 0i64;
    while 0 <= index && (index as usize) < prog.len() {
        let itemp = index as usize;
        count += 1;
        index += prog[itemp];
        prog[itemp] += 1;
    }
    return count    // Number of instructions required to escape.
}

fn part2(input: &Program) -> usize
{
    let mut prog    = input.clone();
    let mut count   = 0usize;
    let mut index   = 0i64;
    while 0 <= index && (index as usize) < prog.len() {
        let itemp = index as usize;
        count += 1;
        index += prog[itemp];
        if prog[itemp] < 3 {
            prog[itemp] += 1;
        } else {
            prog[itemp] -= 1;
        }
    }
    return count    // Number of instructions required to escape.
}

fn main() {
    // Fetch problem input from server.
    let test: Program  = vec![0, 3, 0, 1, -3];
    let input: Program = read_input(&aocfetch::get_data(2017, 5).unwrap());

    // Unit tests on provided example.
    assert_eq!(part1(&test), 5);
    assert_eq!(part2(&test), 10);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
