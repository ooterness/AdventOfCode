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

    // Count corners by inspecting a 2x2 kernel.
    fn corners(&self, r:usize, c:usize) -> usize {
        let w = self.rc.contains(&Rc::new(r+0, c+0)) as usize;
        let x = self.rc.contains(&Rc::new(r+0, c+1)) as usize;
        let y = self.rc.contains(&Rc::new(r+1, c+0)) as usize;
        let z = self.rc.contains(&Rc::new(r+1, c+1)) as usize;
        return match (w, x, y, z) {
            (1, 0, 0, 1) => 2,  // Diagonal gap
            (0, 1, 1, 0) => 2,
            (1, 0, 0, 0) => 1,  // Outside corner
            (0, 1, 0, 0) => 1,
            (0, 0, 1, 0) => 1,
            (0, 0, 0, 1) => 1,
            (0, 1, 1, 1) => 1,  // Inside corner
            (1, 0, 1, 1) => 1,
            (1, 1, 0, 1) => 1,
            (1, 1, 1, 0) => 1,
            _            => 0,  // All other cases
        }
    }

    // Count sides in a shape, including internal sides.
    fn sides(&self) -> usize {
        let rmin = self.rc.iter().map(|rc| rc.r).min().unwrap() - 1;
        let rmax = self.rc.iter().map(|rc| rc.r).max().unwrap() + 1;
        let cmin = self.rc.iter().map(|rc| rc.c).min().unwrap() - 1;
        let cmax = self.rc.iter().map(|rc| rc.c).max().unwrap() + 1;
        let mut count = 0usize;
        for r in rmin..rmax {
            for c in cmin..cmax {
                count += self.corners(r, c);
            }
            if VERBOSE {
                let row: String = (cmin..cmax)
                    .map(|c| char::from_digit(self.corners(r,c) as u32, 10).unwrap()).collect();
                println!("{}", row);
            }
        }
        return count;
    }

    fn price1(&self) -> usize {
        if VERBOSE {println!("{} -> {} x {}", self.ch, self.area(), self.perimeter());}
        self.area() * self.perimeter()
    }

    fn price2(&self) -> usize {
        if VERBOSE {println!("{} -> {} x {}", self.ch, self.area(), self.sides());}
        self.area() * self.sides()
    }
}

impl Garden {
    fn new(input: &str) -> Self {
        // Pre-parse the input into a list of RC coordinates
        let mut plants: HashMap<char, HashSet<Rc>> = HashMap::new();
        for (r,row) in input.trim().lines().enumerate() {
            for (c,ch) in row.trim().chars().enumerate() {
                plants.entry(ch).or_insert(HashSet::new()).insert(Rc::new(r+1,c+1));
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

    fn price1(&self) -> usize {
        if VERBOSE {println!("Garden with {} regions:", self.reg.len());}
        self.reg.iter().map(|r| r.price1()).sum()
    }

    fn price2(&self) -> usize {
        if VERBOSE {println!("Garden with {} regions:", self.reg.len());}
        self.reg.iter().map(|r| r.price2()).sum()
    }
}

fn part1(input: &str) -> usize {
    Garden::new(input).price1()
}

fn part2(input: &str) -> usize {
    Garden::new(input).price2()
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

const EXAMPLE4: &'static str = "\
    EEEEE
    EXXXX
    EEEEE
    EXXXX
    EEEEE";

const EXAMPLE5: &'static str = "\
    AAAAAA
    AAABBA
    AAABBA
    ABBAAA
    ABBAAA
    AAAAAA";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 12).unwrap();

    assert_eq!(part1(EXAMPLE1), 140);
    assert_eq!(part1(EXAMPLE2), 772);
    assert_eq!(part1(EXAMPLE3), 1930);
    assert_eq!(part2(EXAMPLE1), 80);
    assert_eq!(part2(EXAMPLE2), 436);
    assert_eq!(part2(EXAMPLE3), 1206);
    assert_eq!(part2(EXAMPLE4), 236);
    assert_eq!(part2(EXAMPLE5), 368);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
