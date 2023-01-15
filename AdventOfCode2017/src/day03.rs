/// Advent of Code 2017, Day 3
/// Copyright 2023 by Alex Utter

use std::cmp::min;
use std::collections::HashMap;

// An X/Y coordinate. +X is right, +Y is up.
type XY = (i64, i64);
const DIR_U: XY = ( 0,  1);
const DIR_L: XY = (-1,  0);
const DIR_D: XY = ( 0, -1);
const DIR_R: XY = ( 1,  0);

// List of adjacent tiles for a given coordinate.
fn adj(xy: &XY) -> Vec<XY>
{
    return vec![(xy.0-1, xy.1+1), (xy.0, xy.1+1), (xy.0+1, xy.1+1),
                (xy.0-1, xy.1),                   (xy.0+1, xy.1),
                (xy.0-1, xy.1-1), (xy.0, xy.1-1), (xy.0+1, xy.1-1)]
}

// A simple integer square-root function.
fn sqrt(x: i64) -> i64
{
    let mut y = 1i64;
    while y*y <= x { y += 1; }
    return y - 1
}

// Step up to N times in the designated direction.
fn spiral_step(xy: &mut XY, dir: &XY, steps: i64) -> i64
{
    xy.0 += dir.0 * steps;
    xy.1 += dir.1 * steps;
    return steps
}

// Convert spiral index to row/column offset from origin.
fn spiral(idx: &i64) -> XY
{
    // Shortcut for the trivial case.
    if *idx < 2 { return (0,0); }

    // Which "ring" are we in? Nmax = Area = (2*ring-1)^2
    // i.e., Ring 1 = 1, Ring 2 = 2-9, Ring 3 = 10-25...
    let ring = (sqrt(*idx-1) + 1) / 2 + 1;  // Which ring?
    let side = 2 * ring - 2;                // Side length
    let base = 1 + (side-1) * (side-1);     // Area inside this ring?

    // Remaining steps walk around the current ring...
    let mut rem = idx - base;               // Remaining steps
    let mut xy: XY = (ring-1, 2-ring);      // Starting position
    rem -= spiral_step(&mut xy, &DIR_U, min(rem, side-1));
    rem -= spiral_step(&mut xy, &DIR_L, min(rem, side));
    rem -= spiral_step(&mut xy, &DIR_D, min(rem, side));
    rem -= spiral_step(&mut xy, &DIR_R, min(rem, side));
    assert_eq!(rem, 0);                     // Sanity check
    return xy
}

// Manhattan distance from the given index to the origin.
fn part1(input: &i64) -> i64
{
    let xy = spiral(input);
    xy.0.abs() + xy.1.abs()
}

fn part2(input: &i64) -> i64
{
    let mut idx = 1i64;
    let mut mem: HashMap::<XY, i64> = HashMap::new();
    mem.insert(spiral(&idx), 1);
    loop {
        // Calculate location and value of the next element.
        idx += 1;
        let xy = spiral(&idx);
        let sum = adj(&xy).iter()
            .map(|z| mem.get(z).unwrap_or(&0))
            .sum();
        // Stop when we exceed designated threshold.
        if sum > *input { return sum; }
        mem.insert(xy, sum);
    }
}

fn main() {
    // Input for this problem is a single number.
    let input: i64 = 289326;

    // Unit tests on provided examples and other edge cases.
    assert_eq!(part1(&1), 0);
    assert_eq!(part1(&12), 3);
    assert_eq!(part1(&23), 2);
    assert_eq!(part1(&26), 5);
    assert_eq!(part1(&31), 6);
    assert_eq!(part1(&1024), 31);
    assert_eq!(part2(&24), 25);
    assert_eq!(part2(&351), 362);
    assert_eq!(part2(&747), 806);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
