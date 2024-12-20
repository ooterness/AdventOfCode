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

    fn cost_thru(&self, skip:&Rc) -> Vec<usize> {
        let mut cost = Vec::new();
        let adj = self.grid.adj(skip);
        for c1 in adj.iter().filter_map(|rc| self.cost1.get(rc)) {
            for c2 in adj.iter().filter_map(|rc| self.cost2.get(rc)) {
                let new_cost = c1 + c2 + 2;
                if new_cost < self.cost_base() {cost.push(new_cost);}
            }
        }
        return cost;
    }
}

// How many wallhacks save at least 100 steps?
fn part1(input: &str) -> usize {
    let grid = Wallhack::new(input);
    let base = grid.cost_base();
    return grid.grid.walls.iter()
        .map(|rc| grid.cost_thru(rc))
        .map(|cvec| cvec.into_iter().filter(|c| c+100 <= base).count())
        .sum();
}

fn part2(input:&str) -> usize {
    0 //???
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
    assert_eq!(example.cost_thru(&(7,6)).into_iter().min().unwrap(), 20);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
