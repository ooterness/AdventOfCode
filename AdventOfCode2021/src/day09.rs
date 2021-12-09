/// Day 9: https://adventofcode.com/2021/day/9
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

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
}

pub fn solve() {
    let test = HeightMap::new("input/test09.txt");
    let data = HeightMap::new("input/input09.txt");

    assert_eq!(test.part1(), 15);
    println!("Part1: {}", data.part1());
}
