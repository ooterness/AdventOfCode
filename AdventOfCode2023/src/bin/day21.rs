/// Advent of Code 2023, Day 21
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Rc(i32, i32);
type RcSet = HashSet<Rc>;

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

    fn step1(&self, curr: &RcSet, prev: &RcSet) -> RcSet {
        let mut next = RcSet::new();
        for rc0 in curr.iter() {
            for rc1 in DIRECTIONS.iter().map(|d| rc0.add(d)) {
                if self.is_path(&rc1) && !prev.contains(&rc1) {
                    next.insert(rc1);
                }
            }
        }
        return next;
    }

    fn steps(&self, count: usize) -> usize {
        // To reduce memory usage, simulate the active wave-front only, plus
        // accumulators for the total contained even and odd checkerboards.
        let mut ct_prev = 0usize;
        let mut ct_curr = 1usize;
        let mut rc_prev = RcSet::new();
        let mut rc_curr = RcSet::from([self.start]);
        for _ in 0..count {
            let rc_next = self.step1(&rc_curr, &rc_prev);
            let ct_next = ct_prev + rc_next.len();
            (rc_prev, rc_curr) = (rc_curr, rc_next);
            (ct_prev, ct_curr) = (ct_curr, ct_next);
        }
        return ct_curr;
    }

    fn predict(&self, count: usize) -> usize {
        // This method has only been tested for square inputs, and it
        // relies on assumptions that don't apply to the example input.
        // TODO: Assertion for remaining assumptions?
        assert_eq!(self.size.0, self.size.1);
        // Use direct simulation for small N or non-repeating grids.
        let period = self.size.0 as usize;
        if count < 3*period || !self.repeat {return self.steps(count);}
        // Gather some reference data points.
        let x0 = count % period;
        let x1 = x0 + period;
        let x2 = x1 + period;
        let xn = ((count - x0) / period) as isize;
        let y0 = self.steps(x0) as isize;
        let y1 = self.steps(x1) as isize;
        let y2 = self.steps(x2) as isize;
        // Extrapolate by fitting a quadratic polynomial.
        // https://old.reddit.com/r/adventofcode/comments/18o4y0m/
        let a = (y2 - 2*y1 + y0) / 2;
        let b = y1 - y0 - a;
        let c = y0;
        return (a*xn*xn + b*xn + c) as usize;
    }
}

fn part1(input: &str) -> usize {
    Garden::new(input, false).steps(64)
}

fn part2(input: &str) -> usize {
    Garden::new(input, true).predict(26501365)
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
    // Omit largest test cases because they take a long time.
    let example1 = Garden::new(EXAMPLE, false);
    let example2 = Garden::new(EXAMPLE, true);
    assert_eq!(example1.steps(6),    16);
    assert_eq!(example2.steps(6),    16);
    assert_eq!(example2.steps(10),   50);
    assert_eq!(example2.steps(50),   1594);
    assert_eq!(example2.steps(100),  6536);
    assert_eq!(example2.steps(500),  167004);
    //assert_eq!(example2.steps(1000), 668697);
    //assert_eq!(example2.steps(5000), 16733044);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
