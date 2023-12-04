/// Advent of Code 2017, Day 22
/// Copyright 2023 by Alex Utter

extern crate aocfetch;
use std::collections::HashMap;

type Rc = (i64, i64);
const DIRECTIONS: [Rc;4] = [(-1,0), (0,1), (1,0), (0,-1)];

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

struct Grid {
    map: HashMap<Rc,State>, // State of each node
    dir: usize,             // Cursor facing
    loc: Rc,                // Cursor location
    count: usize,           // Count infection events
    simple: bool,           // Simplified Part-1 rules?
}

impl Grid {
    fn new(input: &str, simple: bool) -> Grid {
        // Read the map.
        let mut map = HashMap::new();
        for (r, row) in input.lines().enumerate() {
            for (c, col) in row.chars().enumerate() {
                let rc: Rc = (r as i64, c as i64);
                if col == '#' { map.insert(rc, State::Infected); }
            }
        }
        // Measure size and start in the middle.
        let size = input.lines().count() as i64;
        let mid  = (size - 1) / 2;
        return Grid {map:map, dir:0, loc:(mid,mid), count:0, simple:simple}
    }

    fn step(&mut self) {
        // What's the state of the current node?
        let state = *self.map.get(&self.loc).unwrap_or(&State::Clean);
        if self.simple && state == State::Clean {
            self.dir = (self.dir + 3) % 4;      // Infect and turn left
            self.map.insert(self.loc, State::Infected);
            self.count += 1;
        } else if self.simple {                 // Clean and turn right
            self.dir = (self.dir + 1) % 4;
            self.map.insert(self.loc, State::Clean);
        } else if state == State::Clean {       // Weaken and turn left
            self.dir = (self.dir + 3) % 4;
            self.map.insert(self.loc, State::Weakened);
        } else if state == State::Weakened {    // Infect and move ahead
            self.map.insert(self.loc, State::Infected);
            self.count += 1;
        } else if state == State::Infected {    // Flag and turn right
            self.dir = (self.dir + 1) % 4;
            self.map.insert(self.loc, State::Flagged);
        } else {                                // Clean and turn around
            self.dir = (self.dir + 2) % 4;
            self.map.insert(self.loc, State::Clean);
        }
        // Take one step forward.
        let dir = DIRECTIONS[self.dir];
        self.loc.0 += dir.0; self.loc.1 += dir.1;
    }
}

fn part1(input: &str, steps: usize) -> usize {
    let mut grid = Grid::new(input.trim(), true);
    for _ in 0..steps { grid.step(); }
    return grid.count;
}

fn part2(input: &str, steps: usize) -> usize {
    let mut grid = Grid::new(input.trim(), false);
    for _ in 0..steps { grid.step(); }
    return grid.count;
}

const TEST: &str = "\
..#
#..
...";

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 22).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(TEST, 70), 41);
    assert_eq!(part1(TEST, 10000), 5587);
    assert_eq!(part2(TEST, 100), 26);
    assert_eq!(part2(TEST, 10000000), 2511944);

    // Solve for real input.
    println!("Part 1: {}", part1(&input, 10000));
    println!("Part 2: {}", part2(&input, 10000000));
}
