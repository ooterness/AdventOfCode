/// Advent of Code 2023, Day 14
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashSet;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Rc(isize, isize);

const DIR_N: Rc = Rc(-1, 0);
const DIR_S: Rc = Rc( 1, 0);
const DIR_W: Rc = Rc( 0,-1);
const DIR_E: Rc = Rc( 0, 1);

impl Rc {
    #[allow(dead_code)]
    fn add(&self, other: &Rc) -> Rc {
        Rc(self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: &Rc) -> Rc {
        Rc(self.0 - other.0, self.1 - other.1)
    }

    fn mul(&self, scale: isize) -> Rc {
        Rc(self.0 * scale, self.1 * scale)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cols:   isize,
    rows:   isize,
    round:  HashSet<Rc>,
    square: HashSet<Rc>,
}

impl State {
    fn new(input: &str) -> Self {
        let cols = input.trim().lines().nth(0)
            .unwrap().trim().chars().count() as isize;
        let rows = input.trim().lines().count() as isize;
        let mut round = HashSet::new();
        let mut square = HashSet::new();
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                let rc = Rc(r as isize, c as isize);
                if ch == 'O' {round.insert(rc);}
                if ch == '#' {square.insert(rc);}
            }
        }
        State {cols:cols, rows:rows, round:round, square:square}
    }

    // Is there a fixed block or wall at a given coordinate?
    fn is_wall(&self, rc: &Rc) -> bool {
        rc.0 < 0 || rc.0 >= self.rows ||
        rc.1 < 0 || rc.1 >= self.cols ||
        self.square.contains(rc)
    }

    // Return a list of all coordinates on the designated edge.
    fn edge(&self, dir: &Rc) -> Vec<Rc> {
        match *dir {
            DIR_N => (0..self.cols).map(|c| Rc(-1 as isize,        c as isize)).collect(),
            DIR_S => (0..self.cols).map(|c| Rc(self.rows as isize, c as isize)).collect(),
            DIR_W => (0..self.rows).map(|r| Rc(r as isize,        -1 as isize)).collect(),
            DIR_E => (0..self.rows).map(|r| Rc(r as isize, self.cols as isize)).collect(),
            _     => Vec::new(),
        }
    }

    // Return the post-tilt position of rocks stopped by a given block.
    fn segment(&self, dir: &Rc, start: &Rc) -> Vec<Rc> {
        let mut rc = start.sub(dir);
        let mut count = 0isize;
        while !self.is_wall(&rc) {
            if self.round.contains(&rc) {count += 1;}
            rc = rc.sub(dir);
        }
        (1..=count).map(|n| start.sub(&dir.mul(n))).collect()
    }

    // Move all round rocks as far as possible in the given direction.
    fn tilt(&self, dir: &Rc) -> State {
        // Ray-scanning algorithm: For the target edge and for each fixed
        // rock, scan() uphill to count the number of round rocks in that
        // segment, then place them in their final configuration.
        let mut moved = HashSet::new();
        for rc in self.edge(dir).iter() {
            moved.extend(self.segment(dir, &rc));
        }
        for rc in self.square.iter() {
            moved.extend(self.segment(dir, &rc));
        }
        assert_eq!(moved.len(), self.round.len());
        State {
            cols: self.cols,
            rows: self.rows,
            round: moved,
            square: self.square.clone(),
        }
    }

    // Simulate one iteration of the spin-cycle.
    fn spin(&self) -> Self {
        self.tilt(&DIR_N).tilt(&DIR_W).tilt(&DIR_S).tilt(&DIR_E)
    }

    // Calculate the system state at the end of a spin-cycle.
    fn spin_for(&self, count: usize) -> Self {
        // Keep a cache of previous states to look for repeats.
        // TODO: Linear search is inefficient, but we can't hash a HashSet.
        let mut cache = Vec::<State>::new();
        let mut state = self.clone();
        while cache.len() < count {
            if let Some(prev) = cache.iter().position(|s| s == &state) {
                // Repeat detected, fast-forward N periods.
                let rem = (count - cache.len()) % (cache.len() - prev);
                return cache[prev + rem].clone();
            } else {
                // Simulate one more timestep.
                (state, _) = (state.spin(), cache.push(state));
            }
        }
        return state;
    }

    // Calculate total load for this configuration of rocks.
    fn load(&self) -> isize {
        self.round.iter().map(|rc| self.cols - rc.0).sum()
    }

    // Print this state for debugging.
    #[allow(dead_code)]
    fn print(&self, lbl: &str) {
        println!("Debug: {}", lbl);
        for r in 0..self.rows {
            for c in 0..self.cols {
                let rc = Rc(r, c);
                let ch = if self.round.contains(&rc) {'O'}
                    else if self.square.contains(&rc) {'#'}
                    else {'.'};
                print!("{}", ch);
            }
            println!();
        }
    }
}

fn part1(input: &str) -> isize {
    State::new(input).tilt(&DIR_N).load()
}

fn part2(input: &str) -> isize {
    State::new(input).spin_for(1000000000).load()
}

const EXAMPLE: &'static str = "\
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 14).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 136);
    assert_eq!(part2(EXAMPLE), 64);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
