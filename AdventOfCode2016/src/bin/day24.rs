/// Advent of Code 2016, Day 24
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct RowCol(usize, usize);

struct Maze {
    path: HashSet<RowCol>,
    start: RowCol,
    wires: HashMap<RowCol, usize>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut maze = Maze {
            path: HashSet::new(),
            start: RowCol(0,0),
            wires: HashMap::new() 
        };
        for (row,line) in input.trim().lines().enumerate() {
            for (col,ch) in line.trim().chars().enumerate() {
                let rc = RowCol(row, col);
                // Ignore walls; everything else is terrain.
                if ch == '#' {continue;}
                maze.path.insert(rc);
                // Note starting location and other points of interest.
                if ch == '0' {
                    maze.start = rc;
                } else if let Some(n) = ch.to_digit(10) {
                    maze.wires.insert(rc, n as usize - 1);
                }
            }
        }
        return maze;
    }

    // List of valid moves from given position.
    fn adj(&self, rc: &RowCol) -> Vec<RowCol> {
        let mut tmp = Vec::new();
        for next in [RowCol(rc.0-1, rc.1),
                     RowCol(rc.0+1, rc.1),
                     RowCol(rc.0, rc.1-1),
                     RowCol(rc.0, rc.1+1)] {
            if self.path.contains(&next) {tmp.push(next);}
        }
        return tmp;
    }

    // Find shortest path through all points of interest.
    fn solve(&self, cleanup: bool) -> usize {
        // Use bit-masks to track which points have been visited.
        let mask_rtn = 1u64 << self.wires.len();    // Return to start?
        let mask_poi = mask_rtn - 1;                // All other POI
        let mask_all = mask_poi | if cleanup {mask_rtn} else {0};
        // Breadth-first search from initial position.
        let mut queue: VecDeque<(RowCol, u64, usize)> = VecDeque::new();
        let mut visit: HashSet<(RowCol, u64)> = HashSet::new();
        queue.push_back((self.start, 0, 0));
        visit.insert((self.start, 0));
        while let Some((posn, mask, steps)) = queue.pop_front() {
            for next in self.adj(&posn).into_iter() {
                // Is the new position a point of interest?
                let mut mask_new = mask;
                if let Some(n) = self.wires.get(&next) {mask_new |= 1u64 << n;}
                if mask == mask_poi && next == self.start {mask_new |= mask_rtn;}
                // Return solution, or add new states to the queue.
                if mask_new == mask_all {
                    return steps+1;
                } else if visit.insert((next, mask_new)) {
                    queue.push_back((next, mask_new, steps+1));
                }
            }
        }
        panic!("No solution.");
    }
}

fn part1(input: &str) -> usize {
    let maze = Maze::new(input);
    return maze.solve(false);
}

fn part2(input: &str) -> usize {
    let maze = Maze::new(input);
    return maze.solve(true);
}

const TEST: &str = "\
    ###########
    #0.1.....2#
    #.#######.#
    #4.......3#
    ###########";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 24).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), 14);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
