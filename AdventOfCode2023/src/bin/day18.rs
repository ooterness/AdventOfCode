/// Advent of Code 2023, Day 18
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Rc(isize, isize);
const DIR_N: Rc = Rc(-1,  0);
const DIR_S: Rc = Rc( 1,  0);
const DIR_W: Rc = Rc( 0, -1);
const DIR_E: Rc = Rc( 0,  1);

impl Rc {
    fn add(&self, other: Rc) -> Self {
        Rc(self.0 + other.0, self.1 + other.1)
    }

    fn mul(&self, other: usize) -> Self {
        Rc(self.0 * other as isize, self.1 * other as isize)
    }

    fn len(&self) -> usize {
        (self.0.abs() + self.1.abs()) as usize
    }
}

fn sorted_set<T: std::cmp::Ord>(x: HashSet<T>) -> Vec<T> {
    let mut tmp: Vec<T> = x.into_iter().collect();
    tmp.sort(); return tmp;
}

struct Trench {
    posn: Rc,               // Current position
    segs: HashMap<Rc, Rc>,  // Start, delta
}

impl Trench {
    fn new() -> Self {
        Trench { posn:Rc(0,0), segs:HashMap::new() }
    }

    fn part1(input: &str) -> Self {
        let mut tmp = Trench::new();
        for line in input.trim().lines() {
            let tok: Vec<&str> = line.trim().split(' ').collect();
            let seg = match tok[0] {
                "U" => DIR_N,
                "D" => DIR_S,
                "L" => DIR_W,
                "R" => DIR_E,
                _   => panic!("Invalid direction"),
            }.mul(usize::from_str_radix(&tok[1], 10).unwrap());
            tmp.append(seg);
        }
        assert_eq!(tmp.posn, Rc(0,0));
        return tmp;
    }

    fn part2(input: &str) -> Self {
        const DIRECTIONS: [Rc;4] = [DIR_E, DIR_S, DIR_W, DIR_N];
        let mut tmp = Trench::new();
        for line in input.trim().lines() {
            let tok: Vec<&str> = line.trim().split(' ').collect();
            let hex = usize::from_str_radix(&tok[2][2..8], 16).unwrap();
            let seg = DIRECTIONS[hex % 4].mul(hex / 16);
            tmp.append(seg);
        }
        assert_eq!(tmp.posn, Rc(0,0));
        return tmp;
    }

    fn append(&mut self, delta: Rc) {
        self.segs.insert(self.posn, delta);
        self.posn = self.posn.add(delta);
    }

    fn contains(&self, rc0: Rc) -> bool {
        // Add a half-meter offset by scaling everything 2x.
        let rr = 2*rc0.0 + 1;
        let cc = 2*rc0.1 + 1;
        // Count line-crossings from (R0,C0) to (R0,+inf).
        let cross = self.segs.iter()
            .map(|(rc,dd)| (min(2*rc.0, 2*rc.0+2*dd.0),
                            max(2*rc.0, 2*rc.0+2*dd.0),
                            2*rc.1))
            .filter(|(r0,r1,c0)| *r0 < rr && rr < *r1 && cc < *c0);
        return (cross.count() % 2) > 0;
    }

    fn perimeter(&self) -> usize {
        self.segs.iter().map(|s| s.1.len()).sum()
    }

    fn area(&self) -> usize {
        // Find all unique row and column coordinates.
        let rr = sorted_set(self.segs.keys().map(|rc| rc.0).collect());
        let cc = sorted_set(self.segs.keys().map(|rc| rc.1).collect());
        // For each rectangular section, test if it is inside the loop.
        let mut area = 1 + self.perimeter() / 2;
        for r in 1..rr.len() {
            for c in 1..cc.len() {
                if self.contains(Rc(rr[r-1], cc[c-1])) {
                    let dr = rr[r] - rr[r-1];
                    let dc = cc[c] - cc[c-1];
                    area += (dr * dc) as usize;
                }
            }
        }
        return area;
    }
}

fn part1(input: &str) -> usize {
    Trench::part1(input).area()
}

fn part2(input: &str) -> usize {
    Trench::part2(input).area()
}

const EXAMPLE: &'static str = "\
    R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 18).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 62);
    assert_eq!(part2(EXAMPLE), 952408144115);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
