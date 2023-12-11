/// Advent of Code 2023, Day 11
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

struct Rc(isize, isize);

struct Galaxies {
    galaxies: Vec<Rc>,
}

impl Galaxies {
    fn new(input: &str) -> Self {
        let mut galaxies = Vec::new();
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                if ch == '#' {galaxies.push(Rc(r as isize, c as isize));}
            }
        }
        return Galaxies { galaxies: galaxies };
    }

    fn expand(&self, factor: isize) -> Self {
        // Create mapping function for expanding rows and columns.
        let rows = remap(factor, self.galaxies.iter().map(|rc| rc.0));
        let cols = remap(factor, self.galaxies.iter().map(|rc| rc.1));
        // Rebuild the list of galaxies
        let result = self.galaxies.iter()
            .map(|rc| Rc(rows[&rc.0], cols[&rc.1]));
        return Galaxies { galaxies: result.collect() };
    }

    fn total_distance(&self) -> isize {
        let mut total = 0isize;
        for m in 0..self.galaxies.len()-1 {
            for n in m+1..self.galaxies.len() {
                total += (self.galaxies[m].0 - self.galaxies[n].0).abs();
                total += (self.galaxies[m].1 - self.galaxies[n].1).abs();
            }
        }
        return total;
    }
}

// Given an iterator over occupied rows or columns, create a HashMap
// that remaps input coordinates to expanded output coordinates.
fn remap(factor: isize, input: impl Iterator<Item=isize>) -> HashMap<isize,isize> {
    // Find and sort all unique input values.
    let unique: HashSet<isize> = input.collect();
    let mut sorted: Vec<isize> = unique.into_iter().collect();
    sorted.sort();
    // Build the coordinate mapping function.
    let mut prev   = 0isize;
    let mut offset = 0isize;
    let mut result = HashMap::new();
    for x in sorted.into_iter() {
        offset += (factor - 1) * (x - prev);
        result.insert(x, x + offset);
        prev = x + 1;
    }
    return result;
}

fn solve(input: &str, factor: isize) -> isize {
    Galaxies::new(input).expand(factor).total_distance()
}

fn part1(input: &str) -> isize {
    solve(input, 2)
}

fn part2(input: &str) -> isize {
    solve(input, 1000000)
}

const EXAMPLE: &'static str = "\
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 11).unwrap();

    // Unit tests on provided examples
    assert_eq!(solve(EXAMPLE, 2), 374);
    assert_eq!(solve(EXAMPLE, 10), 1030);
    assert_eq!(solve(EXAMPLE, 100), 8410);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
