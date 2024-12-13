/// Advent of Code 2024, Day 12
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

// Enable diagnostics?
const VERBOSE: bool = false;

// Row + Column coordinate
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Rc {r:usize, c:usize}

type Delta = (isize, isize);
const DIRECTIONS: [Delta;4] = [(-1,0), (0,1), (1,0), (0,-1)];

// A contiguous group of plants of a given type.
struct Region {
    ch: char,
    rc: HashSet<Rc>,
}

// A complete garden is a collection of Regions.
struct Garden {
    reg: Vec<Region>,
}

impl Rc {
    fn new(r:usize, c:usize) -> Self {
        Rc { r:r, c:c }
    }

    fn add(&self, d:&Delta) -> Self {
        Rc { r: self.r.overflowing_add_signed(d.0).0,
             c: self.c.overflowing_add_signed(d.1).0 }
    }

    fn adj(&self) -> [Rc;4] {
        DIRECTIONS.map(|d| self.add(&d))
    }
}

impl Region {
    fn new(ch:char, input:&mut HashSet<Rc>) -> Self {
        // Choose any input item as the seed for a new region.
        let first: Rc = *input.iter().next().unwrap();
        let seed: Rc = input.take(&first).unwrap();
        // Flood-fill to consume all contiguous input items.
        let mut queue: Vec<Rc> = Vec::from([seed]);
        let mut region: HashSet<Rc> = HashSet::from([seed]);
        while let Some(next) = queue.pop() {
            for adj in next.adj() {
                if let Some(rc) = input.take(&adj) {
                    queue.push(rc);
                    region.insert(rc);
                }
            }
        }
        return Region { ch:ch, rc:region };
    }

    fn area(&self) -> usize {
        self.rc.len()
    }

    fn perimeter(&self) -> usize {
        let mut total = 0usize;
        for rc in self.rc.iter() {
            total += rc.adj().iter().filter(|x| !self.rc.contains(x)).count();
        }
        return total;
    }

    fn price(&self) -> usize {
        if VERBOSE {println!("{} -> {} x {}", self.ch, self.area(), self.perimeter());}
        self.area() * self.perimeter()
    }
}

impl Garden {
    fn new(input: &str) -> Self {
        // Pre-parse the input into a list of RC coordinates
        let mut plants: HashMap<char, HashSet<Rc>> = HashMap::new();
        for (r,row) in input.trim().lines().enumerate() {
            for (c,ch) in row.trim().chars().enumerate() {
                plants.entry(ch).or_insert(HashSet::new()).insert(Rc::new(r,c));
            }
        }
        // For each plant type, extract contiguous regions.
        let mut garden = Garden { reg: Vec::new() };
        for (ch, mut rc) in plants.into_iter() {
            while !rc.is_empty() {
                garden.reg.push(Region::new(ch, &mut rc));
            }
        }
        return garden;
    }

    fn price(&self) -> usize {
        if VERBOSE {println!("Garden with {} regions:", self.reg.len());}
        self.reg.iter().map(|r| r.price()).sum()
    }
}

fn part1(input: &str) -> usize {
    Garden::new(input).price()
}

fn part2(input: &str) -> usize {
    0 //???
}

const EXAMPLE1: &'static str = "\
    AAAA
    BBCD
    BBCC
    EEEC";

const EXAMPLE2: &'static str = "\
    OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO";

const EXAMPLE3: &'static str = "\
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 12).unwrap();

    assert_eq!(part1(EXAMPLE1), 140);
    assert_eq!(part1(EXAMPLE2), 772);
    assert_eq!(part1(EXAMPLE3), 1930);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
