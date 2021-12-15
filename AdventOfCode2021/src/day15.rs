/// Day 14: https://adventofcode.com/2021/day/14
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
#[path = "grid.rs"] mod grid;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type CaveGrid = grid::Grid<u64>;
type RowCol = grid::RowCol;

// Node+score for use with BinaryHeap
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct RowColScore {
    rc: RowCol,
    dd: u64,
}

impl RowColScore {
    fn new(rc: &RowCol, dd:u64) -> RowColScore {
        RowColScore { rc:rc.clone(), dd:dd }
    }
}

impl Ord for RowColScore {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dd.cmp(&self.dd)  // Reversed so we get a min-heap
    }
}

impl PartialOrd for RowColScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Find lowest-cost path from upper to lower corner.
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
fn djikstra(grid: &CaveGrid) -> u64 {
    let mut best:   HashMap<RowCol,u64> = HashMap::new();
    let mut done:   HashSet<RowCol> = HashSet::new();
    let mut queue:  BinaryHeap<RowColScore> = BinaryHeap::new();

    // Insert the starting point.
    let start = RowCol {r:0, c:0};
    let end = RowCol {r:grid.size.r as i32 - 1, c:grid.size.c as i32 - 1};
    best.insert(start, 0);
    queue.push(RowColScore::new(&start, 0));

    // Keep popping from priority queue until we find the exit.
    while let Some(next) = queue.pop() {
        // Stop immediately if we've reached the end node.
        if next.rc == end {break;}
        // Skip nodes that we've already visited.
        if !done.insert(next.rc) {continue;}
        // Otherwise, process each of the immediate neighbors.
        for rc in [next.rc.nn(), next.rc.ee(), next.rc.ss(), next.rc.ww()] {
            if let Some(cost) = grid.get(&rc) {
                let old = *best.get(&rc).unwrap_or(&u64::MAX);
                let new = next.dd + cost;
                if new < old {
                    best.insert(rc, new);
                    queue.push(RowColScore::new(&rc, new));
                }
            }
        }
    }
    return best[&end]
}

pub fn solve() {
    let test:CaveGrid = grid::read_grid("input/test15.txt");
    let data:CaveGrid = grid::read_grid("input/input15.txt");

    assert_eq!(djikstra(&test), 40);
    println!("Part1: {}", djikstra(&data));
}
