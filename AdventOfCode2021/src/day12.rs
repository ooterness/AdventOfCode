/// Day 12: https://adventofcode.com/2021/day/12
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::collections::HashMap;

// Note: Using "u64" as a proxy for HashSet, since
//       there's never more than a few dozen rooms.
fn get_mask(idx: usize) -> u64 {
    1u64 << idx
}

struct Room {
    adj: Vec<usize>,                // Adjacent room indices
    smol: bool,                     // Small cave?
}

impl Room {
    fn new(lbl: &str) -> Room {
        let smol = lbl.chars().all(|c| c.is_lowercase());
        Room { adj: Vec::new(), smol: smol }
    }
}

struct Cave {
    labels: HashMap<String, usize>, // Map labels to indices
    rooms: Vec<Room>,               // Vector of Rooms
    start: usize,
    end: usize,
}

impl Cave {
    fn new(filename: &str) -> Cave {
        let mut cave = Cave {
            labels: HashMap::new(),
            rooms: Vec::new(),
            start: 0,
            end: 0,
        };
        // Each line indicates a connection between two named rooms.
        let lines = common::read_lines(filename);
        for line in lines.iter() {
            let words: Vec<&str> = line.split('-').collect();
            assert_eq!(words.len(), 2);
            let room0 = cave.get_room_idx(words[0]);
            let room1 = cave.get_room_idx(words[1]);
            cave.add_connection(room0, room1);
        }
        // Find starting and ending indices.
        cave.start  = *cave.labels.get("start").unwrap();
        cave.end    = *cave.labels.get("end").unwrap();
        return cave
    }

    fn get_room_idx(&mut self, lbl: &str) -> usize {
        if let Some(idx) = self.labels.get(lbl) {
            *idx    // Room already exists
        } else {
            let idx = self.rooms.len();
            self.labels.insert(String::from(lbl), idx);
            self.rooms.push(Room::new(lbl));
            idx     // Newly created object
        }
    }

    fn add_connection(&mut self, x: usize, y: usize) {
        // Sanity check: Two connected large rooms would form an infinite loop.
        assert!(self.rooms[x].smol || self.rooms[y].smol);
        // Add the path from X to Y and from Y to X.
        self.rooms[x].adj.push(y);
        self.rooms[y].adj.push(x);
    }

    // Recursive depth first search.
    fn search(&self, room: usize, prev: u64, spare: bool) -> u64 {
        let mut count = 0u64;
        for next in self.rooms[room].adj.iter() {
            let mask = get_mask(*next);
            if *next == self.end {
                // Reached end of maze.
                count += 1;
            } else if !self.rooms[*next].smol {
                // Large rooms recurse without additional checks.
                count += self.search(*next, prev, spare);
            } else if prev & mask == 0 {
                // Small rooms proceed only if we haven't visited yet.
                count += self.search(*next, prev | mask, spare);
            } else if spare && *next != self.start {
                // Special case: Revisit a single small room.
                count += self.search(*next, prev, false);
            }
        }
        return count
    }

    // Count all possible paths that avoid revisiting a small room.
    fn part1(&self) -> u64 {
        self.search(self.start, get_mask(self.start), false)
    }

    // Count all possible paths that revisit no more than one small room.
    fn part2(&self) -> u64 {
        self.search(self.start, get_mask(self.start), true)
    }
}

pub fn solve() {
    let test1 = Cave::new("input/test12a.txt");
    let test2 = Cave::new("input/test12b.txt");
    let test3 = Cave::new("input/test12c.txt");
    let input = Cave::new("input/input12.txt");

    assert_eq!(test1.part1(), 10);
    assert_eq!(test2.part1(), 19);
    assert_eq!(test3.part1(), 226);
    println!("Max rooms {}", input.rooms.len()); //???
    println!("Part1: {}", input.part1());

    assert_eq!(test1.part2(), 36);
    assert_eq!(test2.part2(), 103);
    assert_eq!(test3.part2(), 3509);
    println!("Part2: {}", input.part2());
}
