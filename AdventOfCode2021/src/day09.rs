/// Day 9: https://adventofcode.com/2021/day/9
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
#[path = "grid.rs"] mod grid;
use std::collections::HashSet;
use grid::RowCol;

type Height = u8;
struct HeightMap {
    h: grid::Grid<Height>,
}

impl HeightMap {
    fn new(filename: &str) -> HeightMap {
        HeightMap { h: grid::read_grid(filename) }
    }

    // Fetch map height with "10" for out-of-bounds
    fn get(&self, rc: &RowCol) -> Height {
        *self.h.get(rc).unwrap_or(&10)
    }

    // Is a given location a "low-point" per Part-1 description?
    fn is_low(&self, rc: &RowCol) -> bool {
        (self.get(rc) < self.get(&rc.nn())) &&
        (self.get(rc) < self.get(&rc.ee())) &&
        (self.get(rc) < self.get(&rc.ss())) &&
        (self.get(rc) < self.get(&rc.ww()))
    }

    fn risk(&self, rc: &RowCol) -> Height {
        if self.is_low(rc) {self.get(rc) + 1} else {0}
    }

    // Find the Part-1 "risk level".
    fn part1(&self) -> u64 {
        self.h.iter()
            .map(|rc| self.risk(&rc) as u64)
            .sum()
    }

    // Find the size of a given basin using flood-fill.
    fn basin(&self, start: RowCol) -> Option<u64> {
        // Only start from the lowest point in the basin.
        if !self.is_low(&start) {return None;}
        // Otherwise start breadth-first search.
        let mut visited = HashSet::<RowCol>::new();
        let mut queue = Vec::<RowCol>::new();
        visited.insert(start);
        queue.push(start);
        while let Some(rc) = queue.pop() {
            for next in [rc.nn(), rc.ee(), rc.ss(), rc.ww()] {
                if self.get(&next) < 9 && !visited.contains(&next) {
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
        for rc in self.h.iter() {
            if let Some(x) = self.basin(rc) {
                basins.push(x);
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
