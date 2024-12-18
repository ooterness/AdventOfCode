/// Advent of Code 2024, Day 18
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Posn = (usize, usize);     // Column, row
type Delta = (isize, isize);
const DIRECTIONS: [Delta;4] = [(-1,0), (0,1), (1,0), (0,-1)];

struct Grid {
    rmax: usize,
    drops: Vec<Posn>,
}

impl Grid {
    fn new(input: &str, rmax: usize) -> Self {
        let mut grid = Grid { rmax: rmax, drops: Vec::new() };
        for line in input.trim().lines() {
            let tok: Vec<usize> = line.trim().split(',')
                .map(|s| s.parse().unwrap()).collect();
            grid.drops.push((tok[0], tok[1]));
        }
        return grid;
    }

    fn add(&self, p:&Posn, d:&Delta) -> Option<Posn> {
        let next = (p.0.overflowing_add_signed(d.0).0,
                    p.1.overflowing_add_signed(d.1).0);
        if next.0 > self.rmax || next.1 > self.rmax {return None;}
        return Some(next);
    }

    fn adj(&self, posn:&Posn) -> Vec<Posn> {
        DIRECTIONS.iter().filter_map(|d| self.add(posn, d)).collect()
    }

    fn walls(&self, time:usize) -> HashSet<Posn> {
        self.drops[0..time].iter().cloned().collect()
    }

    // Using A* search with Manhattan distance heuristic.
    fn part1(&self, time:usize) -> Option<usize> {
        let walls = self.walls(time);
        let start: Posn = (0, 0);
        let goal: Posn = (self.rmax, self.rmax);
        let mut costs: HashMap<Posn, usize> = HashMap::new();
        let mut queue: BinaryHeap<Reverse<(usize, Posn)>> = BinaryHeap::new();
        costs.insert(start, 0);         // Initial condition
        queue.push(Reverse((2*self.rmax, start)));
        while let Some(Reverse((_, prev_posn))) = queue.pop() {
            let prev_cost = *costs.get(&prev_posn).unwrap();
            if prev_posn == goal {return Some(prev_cost);}
            for next_posn in self.adj(&prev_posn) {
                if walls.contains(&next_posn) {continue;}
                let cost = costs.entry(next_posn).or_insert(usize::MAX);
                if prev_cost + 1 < *cost {
                    *cost = prev_cost + 1;
                    let guess = prev_cost + 1 + (self.rmax - next_posn.0)
                                              + (self.rmax - next_posn.1);
                    queue.push(Reverse((guess, next_posn)));
                }
            }
        }
        return None;
    }

    // Use binary search to find the exact cutoff index.
    fn part2(&self) -> usize {
        let mut min = 0usize;
        let mut max = self.drops.len() - 1;
        while min < max {
            let mid = (min + max) / 2;
            if let Some(_) = self.part1(mid) {
                min = mid + 1;
            } else {
                max = mid;
            }
        }
        return min;
    }
}

fn part1(input: &str, rmax: usize, time: usize) -> usize {
    let grid = Grid::new(input, rmax);
    return grid.part1(time).unwrap();
}

fn part2(input:&str, rmax: usize) -> String {
    let grid = Grid::new(input, rmax);
    let posn = grid.drops[grid.part2() - 1];
    return format!("{},{}", posn.0, posn.1);
}

const EXAMPLE: &'static str = "\
    5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2
    5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 18).unwrap();

    assert_eq!(part1(EXAMPLE, 6, 12), 22);
    assert_eq!(part2(EXAMPLE, 6), "6,1");

    println!("Part 1: {}", part1(&input, 70, 1024));
    println!("Part 2: {}", part2(&input, 70));
}
