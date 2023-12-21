/// Advent of Code 2023, Day 21
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Rc(i32, i32);
type RcSet = HashSet<Rc>;
type RcVec = Vec<Rc>;

const DIR_N: Rc = Rc(-1,  0);
const DIR_S: Rc = Rc( 1,  0);
const DIR_W: Rc = Rc( 0, -1);
const DIR_E: Rc = Rc( 0,  1);
const DIRECTIONS: [Rc;4] = [DIR_N, DIR_S, DIR_W, DIR_E];

impl Rc {
    fn add(&self, other: &Rc) -> Self {
        Rc(self.0 + other.0, self.1 + other.1)
    }

    fn rem(&self, other: &Rc) -> Self {
        Rc(self.0.rem_euclid(other.0), self.1.rem_euclid(other.1))
    }
}

struct Garden {
    repeat: bool,
    size:   Rc,
    start:  Rc,
    paths:  RcSet,
    rocks:  RcSet,
}

impl Garden {
    fn new(input: &str, repeat: bool) -> Self {
        let mut garden = Garden {
            repeat: repeat,
            size:   Rc(0,0),
            start:  Rc(0,0),
            paths:  RcSet::new(),
            rocks:  RcSet::new(),
        };
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                let rc = Rc(r as i32, c as i32);
                match ch {
                    '#' => {garden.rocks.insert(rc);},
                    '.' => {garden.paths.insert(rc);},
                    'S' => {garden.paths.insert(rc);
                            garden.start = rc;},
                    _   => {},
                }
            }
        }
        garden.size.0 = garden.paths.iter().map(|rc| rc.0).max().unwrap() + 1;
        garden.size.1 = garden.paths.iter().map(|rc| rc.1).max().unwrap() + 1;
        return garden;
    }

    fn is_path(&self, rc: &Rc) -> bool {
        let rcmod = if self.repeat {rc.rem(&self.size)} else {*rc};
        return self.paths.contains(&rcmod);
    }

    fn step1(&self, wave: &RcVec, cache: &mut RcSet) -> RcVec {
        let mut next = RcVec::new();
        for rc0 in wave.iter() {
            for rc1 in DIRECTIONS.iter().map(|d| rc0.add(d)) {
                if self.is_path(&rc1) && !cache.contains(&rc1) {
                    cache.insert(rc1); next.push(rc1);
                }
            }
        }
        return next;
    }

    fn steps(&self, count: usize) -> usize {
        let mut wave = vec![self.start];    // New items
        let mut even = RcSet::new();        // Cache for even steps
        let mut odd  = RcSet::new();        // Cache for odd steps
        for n in 1..=count {
            let cache = if n%2 == 0 {&mut even} else {&mut odd};
            wave = self.step1(&wave, cache);
            if n % 10000 == 0 {
                // TODO: Several minutes per 10k, far too slow for part 2.
                println!("Step {} = {}/{}", n, wave.len(), odd.len());
            }
        }
        return if count%2 == 0 {even.len()} else {odd.len()};
    }
}

fn part1(input: &str) -> usize {
    Garden::new(input, false).steps(64)
}

fn part2(input: &str) -> usize {
    Garden::new(input, true).steps(26501365)
}

const EXAMPLE: &'static str = "\
    ...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 21).unwrap();

    // Unit tests on provided examples
    let example1 = Garden::new(EXAMPLE, false);
    let example2 = Garden::new(EXAMPLE, true);
    assert_eq!(example1.steps(6),    16);
    assert_eq!(example2.steps(6),    16);
    assert_eq!(example2.steps(10),   50);
    assert_eq!(example2.steps(50),   1594);
    assert_eq!(example2.steps(100),  6536);
    assert_eq!(example2.steps(500),  167004);
    assert_eq!(example2.steps(1000), 668697);
    assert_eq!(example2.steps(5000), 16733044);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
