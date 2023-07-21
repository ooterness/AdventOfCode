/// Advent of Code 2015, Day 3
/// Copyright 2023 by Alex Utter

use std::collections::HashSet;
#[path = "fetch.rs"] mod fetch;

type Pos = (i64, i64);

fn command(ch: char, xy: &mut Pos)
{
    match ch {
        '^' => xy.1 += 1,   // North = +Y
        'v' => xy.1 -= 1,   // South = -Y
        '>' => xy.0 += 1,   // East = +X
        '<' => xy.0 -= 1,   // West = -X
        _   => (),
    }
}

fn part1(input: &str) -> usize
{
    let mut visit: HashSet<Pos> = HashSet::new();
    let mut xy: Pos = (0, 0);
    visit.insert(xy);
    for ch in input.chars() {
        command(ch, &mut xy);
        visit.insert(xy);
    }
    return visit.len();
}

fn part2(input: &str) -> usize
{
    let mut visit: HashSet<Pos> = HashSet::new();
    let mut santa = true;       // Whose turn?
    let mut xy1: Pos = (0, 0);  // Santa
    let mut xy2: Pos = (0, 0);  // Robo-santa
    visit.insert(xy1);
    for ch in input.chars() {
        if santa {
            command(ch, &mut xy1);
            visit.insert(xy1);
        } else {
            command(ch, &mut xy2);
            visit.insert(xy2);
        }
        santa = !santa;
    }
    return visit.len();
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 3).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(">"), 2);
    assert_eq!(part1("^>v<"), 4);
    assert_eq!(part1("^v^v^v^v^v"), 2);
    assert_eq!(part2("^v"), 3);
    assert_eq!(part2("^>v<"), 3);
    assert_eq!(part2("^v^v^v^v^v"), 11);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
