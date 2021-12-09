/// Day 9: https://adventofcode.com/2021/day/9
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::collections::HashSet;

type Height = u32;
struct HeightMap {
    h: Vec<Vec<Height>>,    // Row then column
}

impl HeightMap {
    fn new(filename: &str) -> HeightMap {
        let lines = common::read_lines(filename);
        let height = lines.iter()   // For each line...
            .map(|x| x.trim())      // Trim whitespace
            .map(|x| x.chars()      // Parse each digit
                .map(|d| d.to_digit(10).unwrap_or(10))
                .collect());
        HeightMap { h: height.collect() }
    }

    // Fetch map height with "10" for out-of-bounds
    fn get(&self, r: i32, c: i32) -> Height {
        if (0 <= r) && ((r as usize) < self.h.len()) &&
           (0 <= c) && ((c as usize) < self.h[r as usize].len())
           {self.h[r as usize][c as usize]} else {10}
    }

    // Is a given location a "low-point" per Part-1 description?
    fn is_low(&self, r: i32, c: i32) -> bool {
        (self.get(r,c) < self.get(r-1,c)) &&
        (self.get(r,c) < self.get(r+1,c)) &&
        (self.get(r,c) < self.get(r,c-1)) &&
        (self.get(r,c) < self.get(r,c+1))
    }

    fn risk(&self, r: i32, c: i32) -> u32 {
        if self.is_low(r,c) {self.get(r,c) + 1} else {0}
    }

    // Find the Part-1 "risk level".
    fn part1(&self) -> Height {
        let rows = self.h.len() as i32;
        let cols = self.h[0].len() as i32;
        (0..rows).map(|r|
            (0..cols).map(|c| self.risk(r,c))
                .sum::<Height>())
            .sum()
    }

    // Find the size of a given basin using flood-fill.
    fn basin(&self, r: i32, c: i32) -> Option<u64> {
        // Only start from the lowest point in the basin.
        if !self.is_low(r,c) {return None;}
        // Otherwise start breadth-first search.
        type Coord = (i32, i32);
        let mut visited = HashSet::<Coord>::new();
        let mut queue = Vec::<Coord>::new();
        visited.insert((r,c));
        queue.push((r,c));
        while let Some(rc) = queue.pop() {
            for next in [(rc.0-1,rc.1),(rc.0+1,rc.1),(rc.0,rc.1-1),(rc.0,rc.1+1)] {
                if self.get(next.0, next.1) < 9 && !visited.contains(&next) {
                    visited.insert(next);
                    queue.push(next);
                }
            }
        }
        Some(visited.len() as u64)
    }

    // Part-2 is the product of each basin size.
    fn part2(&self) -> u64 {
        // Find all the basins...
        let mut basins = Vec::<u64>::new();
        for r in 0..self.h.len() {
            for c in 0..self.h[r].len() {
                if let Some(x) = self.basin(r as i32, c as i32) {
                    basins.push(x);
                }
            }
        }
        // Identify the largest three.
        basins.sort(); basins.reverse();
        basins.iter().take(3).product()
    }
}

pub fn solve() {
    let test = HeightMap::new("input/test09.txt");
    let data = HeightMap::new("input/input09.txt");

    assert_eq!(test.part1(), 15);
    println!("Part1: {}", data.part1());

    assert_eq!(test.part2(), 1134);
    println!("Part2: {}", data.part2());
}
