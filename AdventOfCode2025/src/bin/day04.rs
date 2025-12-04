/// Advent of Code 2025, Day 4
/// Copyright 2025 by Alex Utter

use aocfetch;
use std::collections::HashSet;

type Rc = (usize, usize);

fn adj(rc: &Rc) -> [Rc;8] {
    [(rc.0-1, rc.1-1), (rc.0-1, rc.1), (rc.0-1, rc.1+1),
     (rc.0,   rc.1-1),                 (rc.0,   rc.1+1),
     (rc.0+1, rc.1-1), (rc.0+1, rc.1), (rc.0+1, rc.1+1)]
}

#[derive(Clone)]
struct Grid {
    data: HashSet<Rc>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut tmp = Self { data: HashSet::new() };
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                if ch == '@' { tmp.data.insert((r+1, c+1)); }
            }
        }
        return tmp;
    }

    fn accessible(&self) -> HashSet<Rc> {
        let mut result = HashSet::new();
        for rc in self.data.iter() {
            let count = adj(rc).iter()
                .filter_map(|p| self.data.get(p))
                .count();
            if count < 4 { result.insert(rc.clone()); }
        }
        return result;
    }

    fn prune(&mut self) -> usize {
        let tmp = self.accessible();
        for rc in tmp.iter() { self.data.remove(rc); }
        return tmp.len();
    }
}

fn part1(grid: &Grid) -> usize {
    grid.accessible().iter().count()
}

fn part2(grid: &Grid) -> usize {
    let mut rem = grid.clone();
    while rem.prune() > 0 {}
    return grid.data.len() - rem.data.len();
}

const EXAMPLE: &'static str = "\
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 4).unwrap();

    let example = Grid::new(EXAMPLE);
    assert_eq!(part1(&example), 13);
    assert_eq!(part2(&example), 43);

    let time = std::time::Instant::now();
    let data = Grid::new(&input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
