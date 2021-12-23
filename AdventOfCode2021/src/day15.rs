/// Day 15: https://adventofcode.com/2021/day/15
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
#[path = "grid.rs"] mod grid;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type CaveGrid = grid::Grid<usize>;
type RowCol = grid::RowCol;

// Node+score for use with BinaryHeap
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct RowColScore {
    rc: RowCol,
    dd: usize,
}

impl RowColScore {
    fn new(rc: &RowCol, dd:usize) -> RowColScore {
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
fn dijkstra(grid: &CaveGrid) -> usize {
    let mut best:   HashMap<RowCol,usize> = HashMap::new();
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
                let old = *best.get(&rc).unwrap_or(&usize::MAX);
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

// Tile-repetition + modulo for Part 2.
fn tile5x5(grid: &CaveGrid) -> CaveGrid {
    let mut new_data = vec![vec![0; 5*grid.size.c]; 5*grid.size.r];
    for rc in grid.iter() {
        let val = *grid.get(&rc).unwrap_or(&0);
        for tr in 0..5usize {
            for tc in 0..5usize {
                let new_r = (grid.size.r*tr) + (rc.r as usize);
                let new_c = (grid.size.c*tc) + (rc.c as usize);
                let mut new_val = val + tr + tc;
                if new_val > 9 {new_val -= 9;}  // 10+ wraps back to 1
                new_data[new_r][new_c] = new_val;
            }
        }
    }
    CaveGrid::new(new_data)
}

pub fn solve() {
    let test:CaveGrid = grid::read_grid("input/test15.txt");
    let data:CaveGrid = grid::read_grid("input/input15.txt");

    assert_eq!(dijkstra(&test), 40);
    println!("Part1: {}", dijkstra(&data));

    let test5 = tile5x5(&test);
    let data5 = tile5x5(&data);
    assert_eq!(dijkstra(&test5), 315);
    println!("Part1: {}", dijkstra(&data5));
}
