/// Day 24: https://adventofcode.com/2020/day/24
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
use std::collections::HashSet;
#[path = "common.rs"] mod common;

// Hexagonal grid coordinate.
// Using basis vectors +X = East, +Y = Southeast.
#[derive(Clone, Eq, Hash, PartialEq)]
struct HexCoord (i32, i32);

impl HexCoord {
    // Parse a line of characters to get implied coordinate.
    fn parse(line : &str) -> Option<HexCoord> {
        let lvec:Vec<char> = line.chars().collect();
        let mut x = 0i32;
        let mut y = 0i32;
        let mut pos = 0usize;
        while pos < lvec.len() {
            // Parse next 1 or 2 characters from line.
            match (lvec.get(pos), lvec.get(pos+1)) {
                (Some('e'), _) =>
                    {pos += 1; x += 1;}
                (Some('s'), Some('e')) =>
                    {pos += 2; y += 1;}
                (Some('s'), Some('w')) =>
                    {pos += 2; x -= 1; y += 1;}
                (Some('w'), _) =>
                    {pos += 1; x -= 1;}
                (Some('n'), Some('w')) =>
                    {pos += 2; y -= 1;}
                (Some('n'), Some('e')) =>
                    {pos += 2; x += 1; y -= 1;}
                _ =>
                    break,
            }
        }
        // Did we read the entire line?
        if pos == lvec.len() {
            return Some(HexCoord(x, y));
        } else {
            return None;
        }
    }

    // Return a vector with the six adjacent tiles.
    fn adj(&self) -> Vec<HexCoord> {
        vec!(HexCoord(self.0+1, self.1+0),  // E
             HexCoord(self.0+0, self.1+1),  // SE
             HexCoord(self.0-1, self.1+1),  // SW
             HexCoord(self.0-1, self.1+0),  // W
             HexCoord(self.0+0, self.1-1),  // NW
             HexCoord(self.0+1, self.1-1))  // NE
    }
}

// A set of hexagonal tiles.
#[derive(Clone)]
struct HexGrid {
    black: HashSet<HexCoord>,
}

impl HexGrid {
    fn new() -> HexGrid {
        HexGrid {black: HashSet::new()}
    }

    fn parse(lines : &Vec<String>) -> HexGrid {
        let mut grid = HexGrid::new();
        for line in lines.iter() {
            if let Some(next) = HexCoord::parse(line) {
                if grid.black.contains(&next) {
                    grid.black.remove(&next);
                } else {
                    grid.black.insert(next);
                }
            } else {
                eprintln!("Error parsing {}.", line);
            }
        }
        grid
    }

    fn count(&self) -> usize {
        self.black.len()
    }

    fn iter(&self) -> HexGrid {
        // HashMap of adjacency counts.
        let mut count:HashMap<HexCoord,usize> = HashMap::new();
        for tile in self.black.iter() {
            for adj in tile.adj() {
                let tmp = count.get(&adj).unwrap_or(&0).clone();
                count.insert(adj, tmp+1);
            }
        }
        // Create the new state based on adjacency counts...
        let mut next = HexGrid::new();
        for (tile,count) in count.iter() {
            if (*count == 2) || (*count == 1 && self.black.contains(tile)) {
                next.black.insert(tile.clone());
            }
        }
        next
    }

    fn iter_n(&self, iter:usize) -> HexGrid {
        let mut next = self.clone();
        for _ in 0..iter {next = next.iter();}
        next
    }
}

pub fn solve() {
    let test = common::read_strings("input/test24.txt");
    let input = common::read_strings("input/input24.txt");

    let test1 = HexGrid::parse(&test);
    assert_eq!(test1.count(), 10usize);
    assert_eq!(test1.iter_n(1).count(), 15usize);
    assert_eq!(test1.iter_n(2).count(), 12usize);
    assert_eq!(test1.iter_n(3).count(), 25usize);
    assert_eq!(test1.iter_n(4).count(), 14usize);
    assert_eq!(test1.iter_n(5).count(), 23usize);
    assert_eq!(test1.iter_n(6).count(), 28usize);
    assert_eq!(test1.iter_n(7).count(), 41usize);
    assert_eq!(test1.iter_n(8).count(), 37usize);
    assert_eq!(test1.iter_n(9).count(), 49usize);
    assert_eq!(test1.iter_n(10).count(), 37usize);
    assert_eq!(test1.iter_n(20).count(), 132usize);
    assert_eq!(test1.iter_n(30).count(), 259usize);
    assert_eq!(test1.iter_n(40).count(), 406usize);
    assert_eq!(test1.iter_n(50).count(), 566usize);
    assert_eq!(test1.iter_n(60).count(), 788usize);
    assert_eq!(test1.iter_n(70).count(), 1106usize);
    assert_eq!(test1.iter_n(80).count(), 1373usize);
    assert_eq!(test1.iter_n(90).count(), 1844usize);
    assert_eq!(test1.iter_n(100).count(), 2208usize);

    let part1 = HexGrid::parse(&input);
    let part2 = part1.iter_n(100);
    println!("Part1: {} black tiles", part1.count());
    println!("Part2: {} black tiles", part2.count());
}
