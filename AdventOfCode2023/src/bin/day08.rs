/// Advent of Code 2023, Day 8
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Repeat(usize, usize);

struct Maze {
    seq: Vec<bool>,
    aaa: HashSet<usize>,
    zzz: HashSet<usize>,
    lbl: HashMap<String, usize>,
    map: HashMap<usize, (usize, usize)>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut maze = Maze {
            seq: Vec::new(),
            aaa: HashSet::new(),
            zzz: HashSet::new(),
            lbl: HashMap::new(),
            map: HashMap::new(),
        };
        for line in input.trim().lines() {
            if maze.seq.is_empty() {
                maze.seq = line.trim().chars().map(|c| c == 'R').collect();
            } else if !line.is_empty() {
                maze.add(line);
            }
        }
        return maze;
    }

    fn add(&mut self, line: &str) {
        let tok: Vec<&str> = line.trim().split(&[' ', '(', ',', ')']).collect();
        let idx0 = self.get(tok[0]);
        let idx1 = self.get(tok[3]);
        let idx2 = self.get(tok[5]);
        self.map.insert(idx0, (idx1, idx2));
    }

    fn get(&mut self, lbl: &str) -> usize {
        if let Some(idx) = self.lbl.get(lbl) {
            return *idx;        // Already exists
        } else {
            let new_idx = self.lbl.len();
            if lbl.ends_with('A') {self.aaa.insert(new_idx);}
            if lbl.ends_with('Z') {self.zzz.insert(new_idx);}
            self.lbl.insert(lbl.to_string(), new_idx);
            return new_idx;     // Create new label
        }
    }

    fn step(&self, pos: usize, count: usize) -> usize {
        let next = self.seq.get(count % self.seq.len()).unwrap();
        if *next {self.map[&pos].1} else {self.map[&pos].0}
    }

    // Given a starting node, count steps to the 1st and 2nd exit nodes.
    fn rpt(&self, start: usize) -> Repeat {
        let mut pos1 = start;
        let mut count1 = 0usize;
        while !self.zzz.contains(&pos1) {
            pos1 = self.step(pos1, count1); count1 += 1;
        }
        let mut pos2 = self.step(pos1, count1);
        let mut count2 = count1 + 1;
        while !self.zzz.contains(&pos2) {
            pos2 = self.step(pos2, count2); count2 += 1;
        }
        assert!(pos1 == pos2);  // TODO: How to handle multi-exit loops?
        return Repeat(count1, count2 - count1);
    }
}

// Increment a += k*b such that a >= c.
fn incr(a: &mut usize, b: usize, c: usize) {
    *a += b * ((c - *a + b - 1) / b);
}

// Given a pair of Repeat objects, find the least common multiple.
fn lcm(r1: &Repeat, r2: &Repeat) -> Repeat {
    let mut acc1 = r1.0;
    let mut acc2 = r2.0;
    while acc1 != acc2 {
        if acc1 < acc2 {incr(&mut acc1, r1.1, acc2)}
                  else {incr(&mut acc2, r2.1, acc1)};
    }
    let mut step1 = r1.1;
    let mut step2 = r2.1;
    while step1 != step2 {
        if step1 < step2 {incr(&mut step1, r1.1, step2)}
                    else {incr(&mut step2, r2.1, step1)};
    }
    return Repeat(acc1, step1);
}

fn part1(input: &str) -> usize {
    let maze = Maze::new(input);
    let mut steps = 0usize;
    let mut pos = *maze.lbl.get("AAA").unwrap();
    let target  = *maze.lbl.get("ZZZ").unwrap();
    while pos != target {
        pos = maze.step(pos, steps); steps += 1;
    }
    return steps;
}

fn part2(input: &str) -> usize {
    let maze = Maze::new(input);
    let rpt: Vec<_> = maze.aaa.iter().map(|p| maze.rpt(*p)).collect();
    let mut acc = rpt[0];
    for r in rpt[1..].iter() {acc = lcm(&acc, r);}
    return acc.0;
}

const EXAMPLE1: &'static str = "\
    RL\n
    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";

const EXAMPLE2: &'static str = "\
    LLR\n
    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";

const EXAMPLE3: &'static str = "\
    LR\n
    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 8).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE1), 2);
    assert_eq!(part1(EXAMPLE2), 6);
    assert_eq!(part2(EXAMPLE1), 2);
    assert_eq!(part2(EXAMPLE2), 6);
    assert_eq!(part2(EXAMPLE3), 6);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
