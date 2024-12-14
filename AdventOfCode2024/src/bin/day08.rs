/// Advent of Code 2024, Day 8
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

type Rc = (i64, i64);

struct Antennae {
    rc: Vec<Rc>,
}

struct Grid {
    ant: HashMap<char, Antennae>,
    rcmax: Rc,
}

impl Antennae {
    fn new() -> Self {
        Antennae { rc: Vec::new() }
    }

    fn add(&mut self, rc: Rc) {
        self.rc.push(rc);
    }

    fn node(&self, rcmax:&Rc, a:&Rc, b:&Rc, k:i64) -> Option<Rc> {
        if a == b {return None;}
        let rr = b.0 + k * (b.0 - a.0);
        let cc = b.1 + k * (b.1 - a.1);
        if rr < 0 || rcmax.0 < rr {return None;}
        if cc < 0 || rcmax.1 < cc {return None;}
        return Some((rr, cc));
    }

    fn nodes(&self, harmonics:bool, rcmax:&Rc) -> Vec<Rc> {
        let mut result = Vec::new();
        for a in self.rc.iter() {
            for b in self.rc.iter() {
                if harmonics {  // Consider all harmonics
                    let mut k = 0;
                    while let Some(rc) = self.node(rcmax, a, b, k) {
                        result.push(rc); k += 1;
                    }
                } else {        // Only the first antinode
                    if let Some(rc) = self.node(rcmax, a, b, 1) {
                        result.push(rc);
                    }
                }
            }
        }
        return result;
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut ant = HashMap::new();
        let mut rmax = 0i64;
        let mut cmax = 0i64;
        for (rr,row) in input.trim().lines().enumerate() {
            let r = rr as i64;
            if r > rmax {rmax = r;}
            for (cc,ch) in row.trim().chars().enumerate() {
                let c = cc as i64;
                if c > cmax {cmax = c;}
                if ch == '.' {continue;}
                ant.entry(ch).or_insert(Antennae::new()).add((r,c));
            }
        }
        return Grid { ant:ant, rcmax:(rmax,cmax) };
    }

    fn nodes(&self, harmonics: bool) -> HashSet<Rc> {
        let mut result = HashSet::new();
        for ant in self.ant.values() {
            for node in ant.nodes(harmonics, &self.rcmax).into_iter() {
                result.insert(node);
            }
        }
        return result;
    }
}

fn part1(input: &str) -> usize {
    Grid::new(input).nodes(false).len()
}

fn part2(input: &str) -> usize {
    Grid::new(input).nodes(true).len()
}

const EXAMPLE: &'static str = "\
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 8).unwrap();

    assert_eq!(part1(EXAMPLE), 14);
    assert_eq!(part2(EXAMPLE), 34);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
