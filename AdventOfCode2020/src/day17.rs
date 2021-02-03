/// Day 17: https://adventofcode.com/2020/day/17
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
#[path = "common.rs"] mod common;

type Coord = (i64,i64,i64,i64);
struct Cube {
    mode: bool, // False=3D, True=4D
    map: HashMap<Coord,()>,
}

impl Cube {
    /// Create initial state from 2D slice.
    fn compile(lines: &Vec<String>, mode: bool) -> Cube {
        let mut map = HashMap::new();
        for (y,s) in lines.iter().enumerate() {
            for (x,c) in s.chars().enumerate() {
                let xyz:Coord = (x as i64, y as i64, 0, 0);
                if c == '#' {map.insert(xyz, ());}
            }
        }
        return Cube {mode:mode, map:map}
    }

    /// Check whether the given coordinate is active.
    fn lookup(&self, xyz:&Coord) -> bool {
        if let Some(_) = self.map.get(xyz) {true} else {false}
    }

    /// Count adjacent active coordinates.
    fn count_adjacent(&self, xyz:&Coord) -> usize {
        let mut count:usize = 0;
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    for dw in if self.mode {-1..2} else {0..1} {
                        let xyz2:Coord = (xyz.0+dx, xyz.1+dy, xyz.2+dz, xyz.3+dw);
                        if (dx != 0 || dy != 0 || dz != 0 || dw != 0) && self.lookup(&xyz2) {
                            count += 1; // Count each neighbor but not self
                        }
                    }
                }
            }
        }
        return count
    }

    /// How many total active cells?
    fn count_active(&self) -> usize {
        self.map.len()
    }

    /// Iteration simulation by one timestep.
    fn iterate(&self) -> Cube {
        // Special case for null input.
        let mut next = HashMap::new();
        if self.map.is_empty() {return Cube {mode:self.mode, map:next};}

        // Find extents of input.
        let min_x:i64 = self.map.keys().map(|x| x.0).min().unwrap() - 1;
        let min_y:i64 = self.map.keys().map(|x| x.1).min().unwrap() - 1;
        let min_z:i64 = self.map.keys().map(|x| x.2).min().unwrap() - 1;
        let min_w:i64 = if !self.mode {0} else {
                        self.map.keys().map(|x| x.3).min().unwrap() - 1};
        let max_x:i64 = self.map.keys().map(|x| x.0).max().unwrap() + 1;
        let max_y:i64 = self.map.keys().map(|x| x.1).max().unwrap() + 1;
        let max_z:i64 = self.map.keys().map(|x| x.2).max().unwrap() + 1;
        let max_w:i64 = if !self.mode {0} else {
                        self.map.keys().map(|x| x.3).max().unwrap() + 1};

        // Iterate over all potential new cells.
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    for w in min_w..=max_w {
                        let xyz: Coord = (x,y,z,w);
                        let adj = self.count_adjacent(&xyz);
                        if (adj == 3) || (adj == 2 && self.lookup(&xyz)) {
                            next.insert(xyz, ());
                        }
                    }
                }
            }
        }
        return Cube {mode:self.mode, map:next}
    }

    /// Iterate N timesteps.
    fn iterate_n(&self, steps:usize) -> Cube {
        let mut next:Cube = Cube {mode:self.mode, map:self.map.clone()};
        for _ in 0..steps {
            next = next.iterate();
        }
        return next
    }
}

pub fn solve() {
    let example = vec![
        String::from(".#."),
        String::from("..#"),
        String::from("###"),
    ];

    // Simulate the first six timesteps of the 3D example.
    let test1 = Cube::compile(&example, false);
    assert_eq!(test1.iterate_n(0).count_active(), 5usize);
    assert_eq!(test1.iterate_n(1).count_active(), 11usize);
    assert_eq!(test1.iterate_n(2).count_active(), 21usize);
    assert_eq!(test1.iterate_n(3).count_active(), 38usize);
    assert_eq!(test1.iterate_n(6).count_active(), 112usize);

    // Simulate the first six timesteps of the 4D example.
    let test2 = Cube::compile(&example, true);
    assert_eq!(test2.iterate_n(0).count_active(), 5usize);
    assert_eq!(test2.iterate_n(1).count_active(), 29usize);
    assert_eq!(test2.iterate_n(2).count_active(), 60usize);
    assert_eq!(test2.iterate_n(6).count_active(), 848usize);

    // Part-1 3D solution:
    let input = common::read_strings("input/input17.txt");
    let cube1 = Cube::compile(&input, false);
    println!("Part1: {} active cubes.", cube1.iterate_n(6).count_active());

    // Part-2 4D solution:
    let cube2 = Cube::compile(&input, true);
    println!("Part2: {} active cubes.", cube2.iterate_n(6).count_active());
}
