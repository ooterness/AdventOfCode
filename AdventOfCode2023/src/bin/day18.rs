/// Advent of Code 2023, Day 18
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Rc(isize, isize);
const DIR_N: Rc = Rc(-1,  0);
const DIR_S: Rc = Rc( 1,  0);
const DIR_W: Rc = Rc( 0, -1);
const DIR_E: Rc = Rc( 0,  1);
const DIRECTIONS: [Rc;4] = [DIR_N, DIR_W, DIR_S, DIR_E];

fn get_didx(label: &str) -> Option<usize> {
    match label {
        "U" => Some(0),
        "L" => Some(1),
        "D" => Some(2),
        "R" => Some(3),
        _   => None,
    }
}

fn flip_didx(idx: usize) -> usize {
    (idx + 2) % 4
}

impl Rc {
    fn add(&self, other: Rc) -> Self {
        Rc(self.0 + other.0, self.1 + other.1)
    }
}

type DirMask = [bool;4];
const MASK_NONE: DirMask = [false, false, false, false];

struct Trench {
    color: HashMap<Rc, u32>,
    holes: HashMap<Rc, DirMask>,
}

impl Trench {
    fn new(input: &str) -> Self {
        let mut color = HashMap::new();
        let mut holes = HashMap::new();
        let mut posn = Rc(0,0);
        for line in input.trim().lines() {
            let tok: Vec<&str> = line.trim().split(' ').collect();
            let dir = get_didx(&tok[0]).unwrap();
            let src = flip_didx(dir);
            let qty = usize::from_str_radix(&tok[1], 10).unwrap();
            let clr = u32::from_str_radix(&tok[2][2..8], 16).unwrap();
            for _ in 0..qty {
                color.insert(posn, clr);
                holes.entry(posn).or_insert(MASK_NONE)[dir] = true;
                posn = posn.add(DIRECTIONS[dir]);
                holes.entry(posn).or_insert(MASK_NONE)[src] = true;
            }
        }
        assert_eq!(posn, Rc(0,0));
        return Trench { color:color, holes:holes };
    }

    fn area(&self) -> usize {
        // Find bounding box.
        let rmin = self.holes.keys().map(|rc| rc.0).min().unwrap();
        let rmax = self.holes.keys().map(|rc| rc.0).max().unwrap();
        let cmin = self.holes.keys().map(|rc| rc.1).min().unwrap();
        let cmax = self.holes.keys().map(|rc| rc.1).max().unwrap();
        // Scanning each diagonal D=R+C, count spaces in the interior.
        let mut area = self.holes.len();
        for d in (rmin+cmin)..=(rmax+cmax) {
            let mut inside = false;
            for c in cmin..=cmax {
                match self.holes.get(&Rc(d-c,c)) {
                    None => if inside {area += 1;},
                    Some([false, true, false, true]) => {inside = !inside;},  // -
                    Some([true, false, false, true]) => {inside = !inside;},  // L
                    Some([false, true, true, false]) => {inside = !inside;},  // 7
                    Some([true, false, true, false]) => {inside = !inside;},  // |
                    _ => (),
                }
            }
        }
        return area;
    }
}

fn part1(input: &str) -> usize {
    Trench::new(input).area()
}

fn part2(input: &str) -> usize {
    0
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
    assert_eq!(part2(EXAMPLE), 0);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
