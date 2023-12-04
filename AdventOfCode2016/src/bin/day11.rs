/// Advent of Code 2016, Day 11
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const NUM_FLOORS: usize = 4;

// Mask-related functions:
fn mask_set(x:&mut u64, y:u64) {*x |= y;}
fn mask_clr(x:&mut u64, y:u64) {*x &= !y;}
fn mask_safe(gen: u64, mcu: u64) -> bool
    {(gen == 0) || (mcu & gen == mcu)}

// Assign sequential indices to each unique label.
struct Labeler {
    map: HashMap<String, u64>,
}

impl Labeler {
    fn new() -> Self {
        Labeler { map: HashMap::new() }
    }

    fn len(&self) -> u64 {
        self.map.len() as u64
    }

    fn lookup_or_create(&mut self, lbl: &str) -> u64 {
        if let Some(idx) = self.map.get(lbl) {
            return *idx;        // Match existing label.
        } else {
            let next_idx = self.len();
            self.map.insert(lbl.to_string(), self.len());
            return next_idx;    // Create a new label.
        }
    }
}

// Combined state for elevator, generators, and microchips.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    num_chips: u64,             // Number of chip-generator pairs
    elevator: usize,            // Elevator position [0..NUM_FLOORS)
    mask_gen: [u64;NUM_FLOORS], // Generators on each floor
    mask_mcu: [u64;NUM_FLOORS], // Microchips on each floor
}

impl State {
    // Create initial state from longform description.
    fn new(input: &str) -> (Labeler, State) {
        let mut labels = Labeler::new();
        let mut state = State {
            num_chips: 0,
            elevator: 0,
            mask_gen: [0;NUM_FLOORS],
            mask_mcu: [0;NUM_FLOORS],
        };
        // On each line, look for the word "generator" or "microchip".
        for (floor,line) in input.trim().lines().enumerate() {
            let tokens: Vec<&str> = line.split([' ', '-', ',', '.']).collect();
            for (n,&tok) in tokens.iter().enumerate() {
                if tok == "generator" {
                    let index = labels.lookup_or_create(tokens[n-1]);
                    state.add_gen(floor, index);
                } else if tok == "microchip" {
                    let index = labels.lookup_or_create(tokens[n-2]);
                    state.add_mcu(floor, index);
                }
            }
        }
        return (labels, state);
    }

    // Add a generator or microchip to the designated floor.
    fn add_gen(&mut self, floor: usize, index: u64) {
        mask_set(&mut self.mask_gen[floor], 1u64 << index);
        if index >= self.num_chips {self.num_chips += 1;}
    }
    fn add_mcu(&mut self, floor: usize, index: u64) {
        mask_set(&mut self.mask_mcu[floor], 1u64 << index);
        if index >= self.num_chips {self.num_chips += 1;}
    }

    // Have we reached the desired final state?
    fn done(&self) -> bool {
        let target = (1u64 << self.num_chips) - 1;
        return (self.elevator == NUM_FLOORS-1)
            && (self.mask_gen[self.elevator] == target)
            && (self.mask_mcu[self.elevator] == target);
    }

    // Move the elevator up or down, bringing designated device(s).
    fn next(&self, up:bool, take_gen:u64, take_mcu:u64) -> Option<State> {
        // Movement rules check before we start.
        if !mask_safe(take_gen, take_mcu) {return None;}
        if !up && self.elevator == 0 {return None;}
        if up && self.elevator >= NUM_FLOORS-1 {return None;}
        if take_gen == 0 && take_mcu == 0 {return None;}
        if take_gen & self.mask_gen[self.elevator] != take_gen {return None;}
        if take_mcu & self.mask_mcu[self.elevator] != take_mcu {return None;}
        // Move the designated objects.
        let mut next = self.clone();
        next.elevator = if up {self.elevator+1} else {self.elevator-1};
        mask_clr(&mut next.mask_gen[self.elevator], take_gen);
        mask_clr(&mut next.mask_mcu[self.elevator], take_mcu);
        mask_set(&mut next.mask_gen[next.elevator], take_gen);
        mask_set(&mut next.mask_mcu[next.elevator], take_mcu);
        // Safety checks on the old and new floors.
        if !mask_safe(next.mask_gen[self.elevator],
                      next.mask_mcu[self.elevator]) {return None;}
        if !mask_safe(next.mask_gen[next.elevator],
                      next.mask_mcu[next.elevator]) {return None;}
        return Some(next);
    }

    // Find all valid adjacent states.
    fn search(&self) -> Vec<State> {
        // Try each direction and each pair of indices...
        let mut list = Vec::new();
        for up in [true, false] {
            for aa in 0..self.num_chips {
                for bb in aa..self.num_chips {
                    let am = 1u64 << aa;
                    let bm = 1u64 << bb;
                    if let Some(x) = self.next(up, am|bm, 0)    {list.push(x);}
                    if let Some(x) = self.next(up, am, bm)      {list.push(x);}
                    if let Some(x) = self.next(up, bm, aa)      {list.push(x);}
                    if let Some(x) = self.next(up, 0, am|bm)    {list.push(x);}
                }
            }
        }
        return list;
    }
}

// Breadth first search for minimum number of moves.
fn bfs(init: &State, verbose: bool) -> usize {
    // Set the initial state...
    let mut queue: VecDeque<(State,usize)> = VecDeque::new();
    let mut visit: HashSet<State> = HashSet::new();
    queue.push_back((init.clone(), 0));
    visit.insert(init.clone());
    // For each item on the queue, visit all new adjacent states.
    while let Some((state, steps)) = queue.pop_front() {
        if verbose {println!("{:?}", state);}
        for next in state.search().into_iter() {
            if next.done() {return steps+1;}
            if !visit.contains(&next) {
                visit.insert(next.clone());
                queue.push_back((next, steps+1));
            }
        }
    }
    panic!("No solution");
}

fn part1(input: &str) -> usize {
    let (_, init) = State::new(input);
    return bfs(&init, false);
}

fn part2(input: &str) -> usize {
    let (mut lbl, mut init) = State::new(input);
    let idx1 = lbl.lookup_or_create("dilithium");
    let idx2 = lbl.lookup_or_create("elerium");
    init.add_gen(0, idx1);
    init.add_gen(0, idx2);
    init.add_mcu(0, idx1);
    init.add_mcu(0, idx2);
    return bfs(&init, false);
}

const TEST: &str = "\
The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 11).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), 11);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
