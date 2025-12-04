/// Advent of Code 2025, Day 4
/// Copyright 2025 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

type Rc = (usize, usize);

fn rc_adj(rc: &Rc) -> [Rc;8] {
    [(rc.0-1, rc.1-1), (rc.0-1, rc.1), (rc.0-1, rc.1+1),
     (rc.0,   rc.1-1),                 (rc.0,   rc.1+1),
     (rc.0+1, rc.1-1), (rc.0+1, rc.1), (rc.0+1, rc.1+1)]
}

#[derive(Clone)]
struct Grid {
    data: HashMap<Rc, usize>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut tmp = Self { data: HashMap::new() };
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                if ch == '@' {
                    let new_rc = (r+1, c+1);
                    let mut new_ct = 0usize;
                    for old_rc in rc_adj(&new_rc).iter() {
                        if let Some(old_ct) = tmp.data.get_mut(old_rc) {
                            *old_ct += 1; new_ct += 1;
                        }
                    }
                    tmp.data.insert(new_rc, new_ct);
                }
            }
        }
        return tmp;
    }

    fn accessible(&self) -> HashSet<Rc> {
        self.data.iter()
            .filter( |(_,ct)| **ct < 4 )
            .map( |(rc,_)| rc.clone() )
            .collect()
    }

    fn prune(&mut self) -> usize {
        let tmp = self.accessible();
        for rc in tmp.iter() {
            self.data.remove(rc);
        }
        for rc in tmp.iter() {
            for old_rc in rc_adj(rc).iter() {
                if let Some(old_ct) = self.data.get_mut(&old_rc) {
                    *old_ct -= 1;
                }
            }
        }
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
