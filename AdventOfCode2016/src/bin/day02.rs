/// Advent of Code 2016, Day 2
/// Copyright 2023 by Alex Utter

use aocfetch;

struct Posn(i64, i64);

fn move_one(prev: &Posn, step: char) -> Posn {
    match step {
        'U' => Posn(prev.0, prev.1-1),
        'D' => Posn(prev.0, prev.1+1),
        'L' => Posn(prev.0-1, prev.1),
         _  => Posn(prev.0+1, prev.1),
    }
}

// Given position on a square 3x3 grid, decode button value.
// (Return Some(x) for a valid button, or None if out-of-bounds.)
fn decode3(pos: &Posn) -> Option<i64>
{
    if 1 <= pos.0 && pos.0 <= 3 && 1 <= pos.1 && pos.1 <= 3 {
        Some(3*pos.1 + pos.0 - 3)
    } else {
        None
    }
}

// Given position on a diagonal 5x5 grid, decode button value.
// (Return Some(x) for a valid button, or None if out-of-bounds.)
fn decode5(pos: &Posn) -> Option<i64>
{
    match pos {
        Posn(3,1) => Some(0x1),
        Posn(2,2) => Some(0x2),
        Posn(3,2) => Some(0x3),
        Posn(4,2) => Some(0x4),
        Posn(1,3) => Some(0x5),
        Posn(2,3) => Some(0x6),
        Posn(3,3) => Some(0x7),
        Posn(4,3) => Some(0x8),
        Posn(5,3) => Some(0x9),
        Posn(2,4) => Some(0xA),
        Posn(3,4) => Some(0xB),
        Posn(4,4) => Some(0xC),
        Posn(3,5) => Some(0xD),
        _ => None,
    }
}

fn part1(input: &str) -> i64
{
    let mut seq = 0i64;
    let mut pos = Posn(2, 2);   // "5" in middle of 3x3
    for line in input.trim().lines() {
        // Attempt to move in the designated direction,
        // ignoring attempts to move out-of-bounds.
        for step in line.trim().chars() {
            let next = move_one(&pos, step);
            if let Some(_) = decode3(&next) {pos = next;}
        }
        // Append current button to the sequence.
        seq = 10*seq + decode3(&pos).unwrap();
    }
    return seq;
}

fn part2(input: &str) -> i64
{
    let mut seq = 0i64;
    let mut pos = Posn(1, 3);   // "5" at corner of 5x5
    for line in input.trim().lines() {
        // Attempt to move in the designated direction,
        // ignoring attempts to move out-of-bounds.
        for step in line.trim().chars() {
            let next = move_one(&pos, step);
            if let Some(_) = decode5(&next) {pos = next;}
        }
        // Append current button to the sequence.
        seq = 16*seq + decode5(&pos).unwrap();
    }
    return seq;
}

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 2).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1("ULL\nRRDDD\nLURDL\nUUUUD"), 1985);
    assert_eq!(part2("ULL\nRRDDD\nLURDL\nUUUUD"), 0x5DB3);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {:x}", part2(&input));
}
