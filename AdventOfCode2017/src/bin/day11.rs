/// Advent of Code 2017, Day 11
/// Copyright 2023 by Alex Utter

extern crate aocfetch;

// Cardinal axes are U = NE and V = N.
// https://www.redblobgames.com/grids/hexagons/#distances
type Hex = (i64, i64);

fn parse(input: &str) -> Vec<Hex> {
    let mut hex: Hex = (0, 0);
    let mut hexes: Vec<Hex> = Vec::new();
    for step in input.trim().split(',') {
        match step {
            "nw" => {hex.0 -= 1; hex.1 += 1;},
            "n"  => {hex.0 += 0; hex.1 += 1;},
            "ne" => {hex.0 += 1; hex.1 += 0;},
            "se" => {hex.0 += 1; hex.1 -= 1;},
            "s"  => {hex.0 += 0; hex.1 -= 1;},
            "sw" => {hex.0 -= 1; hex.1 += 0;},
            _ => {println!("Invalid input: {}", step);},
        }
        hexes.push(hex.clone());
    }
    return hexes;
}

fn distance(hex: &Hex) -> i64 {
    (hex.0.abs() + hex.1.abs() + (hex.0 + hex.1).abs()) / 2
}

fn part1(input: &str) -> i64 {
    let steps = parse(input);
    return distance(steps.last().unwrap());
}

fn part2(input: &str) -> i64 {
    let steps = parse(input);
    return steps.iter().map(|x| distance(x)).max().unwrap();
}

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 11).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1("ne,ne,ne"), 3);
    assert_eq!(part1("ne,ne,sw,sw"), 0);
    assert_eq!(part1("ne,ne,s,s"), 2);
    assert_eq!(part1("se,sw,se,sw,sw"), 3);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
