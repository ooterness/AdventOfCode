/// Advent of Code 2024, Day 20
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Rc = (usize, usize);       // Row, column
type Delta = (isize, isize);    // Change in row, column
const DIRECTIONS: [Delta;4] = [(-1,0), (0,1), (1,0), (0,-1)];

struct Grid {
    rows:   usize,
    cols:   usize,
    start:  Rc,
    goal:   Rc,
    walls:  HashSet<Rc>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = Grid {
            rows:   0usize,
            cols:   0usize,
            start:  (0,0),
            goal:   (0,0),
            walls:  HashSet::new(),
        };
        for (r,row) in input.trim().lines().enumerate() {
            if grid.rows <= r {grid.rows = r + 1;}
            for (c,ch) in row.trim().chars().enumerate() {
                if grid.cols <= c {grid.cols = c + 1;}
                if ch == '#' {grid.walls.insert((r,c));}
                if ch == 'S' {grid.start = (r,c);}
                if ch == 'E' {grid.goal  = (r,c);}
            }
        }
        return grid;
    }

    fn add(&self, rc:&Rc, d:&Delta) -> Option<Rc> {
        let next = (rc.0.overflowing_add_signed(d.0).0,
                    rc.1.overflowing_add_signed(d.1).0);
        if next.0 >= self.rows || next.1 >= self.cols {return None;}
        if self.walls.contains(&next) {return None;}
        return Some(next);
    }

    fn adj(&self, rc:&Rc) -> Vec<Rc> {
        DIRECTIONS.iter().filter_map(|d| self.add(rc, d)).collect()
    }

    // Minimum path-length from a given node to all others.
    fn cost_from(&self, init:Rc) -> HashMap<Rc,usize> {
        // Simple breadth-first-search flood fill.
        let mut costs: HashMap<Rc, usize> = HashMap::new();
        let mut visit: HashSet<Rc> = HashSet::new();
        let mut queue: VecDeque<(usize,Rc)> = VecDeque::new();
        costs.insert(init, 0);    
        visit.insert(init);
        queue.push_back((0, init));
        while let Some((cost, prev)) = queue.pop_front() {
            for rc in self.adj(&prev) {
                if !visit.contains(&rc) {
                    costs.insert(rc, cost+1);
                    visit.insert(rc);
                    queue.push_back((cost+1, rc));
                }
            }
        }
        return costs;
    }
}

struct Wallhack {
    grid:  Grid,
    cost1: HashMap<Rc, usize>,
    cost2: HashMap<Rc, usize>,
}

impl Wallhack {
    fn new(input: &str) -> Self {
        let grid = Grid::new(input);
        let cost1 = grid.cost_from(grid.start);
        let cost2 = grid.cost_from(grid.goal);
        Wallhack { grid:grid, cost1:cost1, cost2:cost2 }
    }

    fn cost_base(&self) -> usize {
        *self.cost1.get(&self.grid.goal).unwrap()
    }

    fn count_hacks(&self, skip:usize, save:usize) -> usize {
        let mut count = 0usize;
        for (prev, c1) in self.cost1.iter() {
            for (next, c2) in self.cost2.iter() {
                // Is Manhattan distance within the wallhack range?
                let dist = prev.0.abs_diff(next.0) + prev.1.abs_diff(next.1);
                if dist > skip {continue;}
                // Any cost savings to this move?
                let new_cost = c1 + c2 + dist;
                if new_cost + save <= self.cost_base() {count += 1;}
            }
        }
        return count;
    }
}

// How many 2-step wallhacks save at least 100 steps?
fn part1(input: &str) -> usize {
    Wallhack::new(input).count_hacks(2, 100)
}

// How many 20-step wallhacks save at least 100 steps?
fn part2(input:&str) -> usize {
    Wallhack::new(input).count_hacks(20, 100)
}

const EXAMPLE: &'static str = "\
    ###############
    #...#...#.....#
    #.#.#.#.#.###.#
    #S#...#.#.#...#
    #######.#.#.###
    #######.#.#...#
    #######.#.###.#
    ###..E#...#...#
    ###.#######.###
    #...###...#...#
    #.#####.#.###.#
    #.#...#.#.#...#
    #.#.#.#.#.#.###
    #...#...#...###
    ###############";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 20).unwrap();

    let example = Wallhack::new(EXAMPLE);
    assert_eq!(example.cost_base(), 84);
    assert_eq!(example.count_hacks(2, 20), 5);
    assert_eq!(example.count_hacks(2, 64), 1);
    assert_eq!(example.count_hacks(20, 72), 29);
    assert_eq!(example.count_hacks(20, 74), 7);
    assert_eq!(example.count_hacks(20, 76), 3);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
