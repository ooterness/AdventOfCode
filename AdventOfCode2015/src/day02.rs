/// Advent of Code 2015, Day 2
/// Copyright 2023 by Alex Utter

use std::cmp::max;
use std::cmp::min;
#[path = "fetch.rs"] mod fetch;

fn max3(x: usize, y: usize, z: usize) -> usize
{
    max(x, max(y, z))
}

fn min3(x: usize, y: usize, z: usize) -> usize
{
    min(x, min(y, z))
}

type Box = (usize, usize, usize);
fn read_line(line: &str) -> Box
{
    let x: Vec<usize> = line.split('x')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    assert_eq!(x.len(), 3);
    return (x[0], x[1], x[2]);
}

fn area(line: &str) -> usize
{
    let (x, y, z) = read_line(line);
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    return 2 * (xy + xz + yz) + min3(xy, xz, yz);
}

fn ribbon(line: &str) -> usize
{
    let (x, y, z) = read_line(line);
    let p = x + y + z - max3(x, y, z);
    return 2*p + x*y*z;
}

fn part1(input: &str) -> usize
{
    input.lines().map(area).sum()
}

fn part2(input: &str) -> usize
{
    input.lines().map(ribbon).sum()
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 2).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1("2x3x4"), 58);
    assert_eq!(part1("1x1x10"), 43);
    assert_eq!(part2("2x3x4"), 34);
    assert_eq!(part2("1x1x10"), 14);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
