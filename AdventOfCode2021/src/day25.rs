/// Day 25: https://adventofcode.com/2021/day/25
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::collections::HashSet;

const VERBOSE:bool = false;

type RowCol = (usize,usize);

#[derive(Clone)]
struct Cucumbers {
    size: RowCol,           // Grid size (rows,cols)
    e: HashSet<RowCol>,     // Coordinates of east-moving sea cucumbers
    s: HashSet<RowCol>,     // Coordinates of south-moving sea cucumbers
}

impl Cucumbers {
    // Read map from file.
    fn new(filename: &str) -> Cucumbers {
        let lines = common::read_lines(filename);
        let rows = lines.len();
        let cols = lines[0].chars().count();
        let mut e = HashSet::new();
        let mut s = HashSet::new();
        for (r,line) in lines.iter().enumerate() {
            assert_eq!(cols, line.chars().count());
            for (c,ch) in line.chars().enumerate() {
                if ch == '>' {e.insert((r,c));}
                if ch == 'v' {s.insert((r,c));}
            }
        }
        Cucumbers { size:(rows,cols), e:e, s:s }
    }

    // Iterate one timestep.  Returns new state and number of moves.
    fn iter(&self) -> (usize,Cucumbers) {
        // State after movement.
        let mut moved  = 0usize;
        let mut next_e = HashSet::new();
        let mut next_s = HashSet::new();
        // East-moving heard goes first.
        for rc in self.e.iter() {
            // Move east with wraparound
            let next = (rc.0, (rc.1 + 1) % self.size.1);
            // Can we move safely?
            if self.e.contains(&next) || self.s.contains(&next) {
                next_e.insert(*rc);     // Stuck
            } else {
                next_e.insert(next);    // Move forward
                moved += 1;
            }
        }
        // South-moving herd goes second.
        for rc in self.s.iter() {
            // Move south with wraparound
            let next = ((rc.0+1) % self.size.0, rc.1);
            // Can we move safely?
            if next_e.contains(&next) || self.s.contains(&next) {
                next_s.insert(*rc);     // Stuck
            } else {
                next_s.insert(next);    // Move forward
                moved += 1;
            }
        }
        ( moved, Cucumbers { size:self.size, e:next_e, s:next_s } )
    }

    // Count iterations until deadlocked.
    fn time_to_deadlock(&self) -> usize {
        let mut iters = 0usize;
        let mut state = (0usize, self.clone());
        loop {
            iters += 1;
            state = state.1.iter();
            if VERBOSE {eprintln!("Iter {} -> {}", iters, state.0);}
            if state.0 == 0 {return iters;}
        }
    }
}

pub fn solve() {
    let test = Cucumbers::new("input/test25.txt");
    let data = Cucumbers::new("input/input25.txt");

    assert_eq!(test.time_to_deadlock(), 58);
    println!("Part1: {}", data.time_to_deadlock());
}
