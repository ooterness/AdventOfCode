/// Day 12: https://adventofcode.com/2021/day/12
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::collections::HashMap;
use std::collections::HashSet;

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
}

impl Cave {
    fn new(filename: &str) -> Cave {
        let mut cave = Cave {
            labels: HashMap::new(),
            rooms: Vec::new(),
        };
        let lines = common::read_lines(filename);
        for line in lines.iter() {
            let words: Vec<&str> = line.split('-').collect();
            assert_eq!(words.len(), 2);
            let room0 = cave.get_room_idx(words[0]);
            let room1 = cave.get_room_idx(words[1]);
            cave.add_connection(room0, room1);
        }
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
        self.rooms[x].adj.push(y);
        self.rooms[y].adj.push(x);
    }

    // Count all possible paths that avoid revisiting a small room.
    fn part1(&self) -> u64 {
        // Find starting and ending indices.
        let start   = *self.labels.get("start").unwrap();
        let end     = *self.labels.get("end").unwrap();
        // Depth-first search of paths.
        type SearchState = (usize, HashSet<usize>);
        let initial: SearchState = (start, HashSet::from([start]));
        let mut queue = vec![initial];
        let mut count = 0u64;
        while let Some((room,prev)) = queue.pop() {
            for next in self.rooms[room].adj.iter() {
                if *next == end {
                    // Reached end of maze.
                    count += 1;
                } else if !self.rooms[*next].smol {
                    // Large rooms recurse without additional checks.
                    queue.push((*next, prev.clone()));
                } else if !prev.contains(next) {
                    // Small rooms proceed only if we haven't visited yet.
                    let mut prev2 = prev.clone();
                    prev2.insert(*next);
                    queue.push((*next, prev2));
                }
            }
        }
        count
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
    println!("Part1: {}", input.part1());
}
