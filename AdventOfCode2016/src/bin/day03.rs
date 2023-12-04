/// Advent of Code 2016, Day 3
/// Copyright 2023 by Alex Utter

use aocfetch;

struct Triplet(i64, i64, i64);

fn read_line(line: &str) -> Triplet
{
    let x: Vec<i64> = line
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    return Triplet(x[0], x[1], x[2]);
}

fn valid(x:i64, y:i64, z:i64) -> bool {
    x < y+z && y < x+z && z < x+y
}

fn part1(input: &str) -> i64
{
    let mut count = 0i64;
    for line in input.trim().lines() {
        let Triplet(x,y,z) = read_line(line);
        if valid(x, y, z) {count += 1;}
    }
    return count;
}

fn part2(input: &str) -> i64
{
    let mut count = 0i64;
    let lines: Vec<&str> = input.trim().lines().collect();
    for n in 0..(lines.len()/3) {
        let a = read_line(lines[3*n+0]);
        let b = read_line(lines[3*n+1]);
        let c = read_line(lines[3*n+2]);
        if valid(a.0, b.0, c.0) {count += 1};
        if valid(a.1, b.1, c.1) {count += 1};
        if valid(a.2, b.2, c.2) {count += 1};
    }
    return count;
}

const TEST: &str = "\
101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 3).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1("5 10 25\n3 4 5\n"), 1);
    assert_eq!(part1(TEST), 3);
    assert_eq!(part2(TEST), 6);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
