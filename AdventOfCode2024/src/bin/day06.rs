/// Advent of Code 2024, Day 6
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashSet;

type Posn = (i64, i64);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Guard {
    rc: Posn,
    dir: usize,
}

impl Guard {
    fn new() -> Self {
        Guard {rc: (0,0), dir: 0}
    }

    fn inside(&self, size: &Posn) -> bool {
        return 0 <= self.rc.0 && self.rc.0 < size.0
            && 0 <= self.rc.1 && self.rc.1 < size.1;
    }

    fn next(&self) -> Posn {
        match self.dir {
            0 => (self.rc.0-1,  self.rc.1  ),   // North
            1 => (self.rc.0,    self.rc.1+1),   // East
            2 => (self.rc.0+1,  self.rc.1  ),   // South
            _ => (self.rc.0,    self.rc.1-1),   // West
        }
    }

    fn forward(&mut self) {
        self.rc = self.next();
    }

    fn rotate(&mut self) {
        self.dir = (self.dir + 1) % 4;
    }
}

#[derive(Clone)]
struct State {
    map: HashSet<Posn>, // Obstacles
    guard: Guard,       // Guard position
    size: Posn,         // Size of map
}

impl State {
    fn new(input: &str) -> Self {
        let mut state = State {
            map:    HashSet::new(),
            guard:  Guard::new(),
            size:   (0,0),
        };
        let mut max_row = 0usize;
        let mut max_col = 0usize;
        for (r,row) in input.trim().lines().enumerate() {
            if r >= max_row {max_row = r;}
            for (c,ch) in row.trim().chars().enumerate() {
                if c >= max_col {max_col = c;}
                let rc: Posn = (r as i64, c as i64);
                match ch {
                    '.' => {},
                    '#' => {state.map.insert(rc);}
                    '^' => {state.guard.rc = rc; state.guard.dir = 0;},
                    '>' => {state.guard.rc = rc; state.guard.dir = 1;},
                    'v' => {state.guard.rc = rc; state.guard.dir = 2;},
                    '<' => {state.guard.rc = rc; state.guard.dir = 3;},
                    _   => panic!("Invalid map token: {}", ch),
                };
            }
        }
        state.size = ((max_row+1) as i64, (max_col+1) as i64);
        return state;
    }

    fn is_loop(&self) -> bool {
        let mut guard = self.guard;
        let mut visit = HashSet::new();
        loop {
            if visit.contains(&guard) {return true;}
            if !guard.inside(&self.size) {return false;}
            visit.insert(guard.clone());
            if self.map.contains(&guard.next()) {
                guard.rotate();
            } else {
                guard.forward();
            }
        }
    }

    fn path(&self) -> HashSet<Posn> {
        let mut guard = self.guard;
        let mut visit = HashSet::new();
        loop {
            if !guard.inside(&self.size) {return visit;}
            visit.insert(guard.rc.clone());
            if self.map.contains(&guard.next()) {
                guard.rotate();
            } else {
                guard.forward();
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let init = State::new(input);
    return init.path().len();
}

fn part2(input: &str) -> usize {
    let init = State::new(input);
    let mut count = 0usize;
    for r in 0..init.size.0 {
        for c in 0..init.size.1 {
            // Is there already something at this location?
            if init.guard.rc.0 == r && init.guard.rc.1 == c {continue;}
            if init.map.contains(&(r,c)) {continue;}
            // Otherwise, place obstacle and test if the new path forms a loop.
            let mut temp = init.clone();
            temp.map.insert((r,c));
            if temp.is_loop() {count += 1;}
        }
    }
    return count;
}

const EXAMPLE: &'static str = "\
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 6).unwrap();

    assert_eq!(part1(EXAMPLE), 41);
    assert_eq!(part2(EXAMPLE), 6);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
