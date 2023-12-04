/// Advent of Code 2017, Day 24
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashSet;

type Segment = (u64, u64);      // Type for each end of segment
type Segments = Vec<Segment>;

fn parse(input: &str) -> Segments {
    let mut seg = Segments::new();
    for line in input.lines() {
        let tmp: Vec<u64> = line.split('/')
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(tmp.len(), 2);
        seg.push((tmp[0], tmp[1]));
    }
    return seg;
}

struct Bridge {
    length: usize,      // Number of segments
    strength: u64,      // Total score of all segments
    used_mask: u64,     // Which segments used?
    end_type: u64,      // Type for exposed end
}
type Bridges = Vec<Bridge>;

impl Bridge {
    fn new() -> Bridge {
        Bridge { length: 0, strength: 0, used_mask: 0, end_type: 0 }
    }

    fn can_add(&self, idx: &usize, seg: &Segment) -> bool {
        let mask = 1u64 << idx;
        let plug = (self.end_type == seg.0) || (self.end_type == seg.1);
        let used = (self.used_mask & mask) > 0;
        return plug && !used;
    }

    fn add(&self, idx: &usize, seg: &Segment) -> Bridge {
        assert!(self.can_add(idx, seg));
        Bridge {
            length: self.length + 1,
            strength: self.strength + seg.0 + seg.1,
            used_mask: self.used_mask | (1u64<<idx),
            end_type: if self.end_type == seg.0 {seg.1} else {seg.0},
        }
    }

    fn add_all(&self, segments: &Segments) -> Bridges {
        segments.iter().enumerate()
            .filter(|(n,x)| self.can_add(n, x))
            .map(|(n,x)| self.add(&n, x))
            .collect()
    }
}

// Breadth-first search over all bridges.  As we do so, note the
// strongest bridge and the strength of the longest bridge.
fn stronk(input: &Segments) -> (u64, u64) {
    // Initial state is the empty bridge.
    let mut queue: Vec<Bridge> = vec![Bridge::new()];
    let mut visited: HashSet<u64> = HashSet::new();
    visited.insert(0);
    // Iterate over all possible bridges.
    let mut max_long = (0usize, 0u64);
    let mut max_str = 0u64;
    while let Some(bridge) = queue.pop() {
        for next in bridge.add_all(input).into_iter() {
            if !visited.contains(&next.used_mask) {
                // Update running maxima.
                max_long = std::cmp::max(max_long, (next.length, next.strength));
                max_str = std::cmp::max(max_str, next.strength);
                // Add new bridge to the queue.
                visited.insert(next.used_mask);
                queue.push(next);
            }
        }
    }
    return (max_str, max_long.1);
}

const TEST: &str = "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 24).unwrap();

    // Unit tests based on the provided examples.
    assert_eq!(stronk(&parse(TEST)), (31, 19));

    // Solve for real input.
    let (part1, part2) = stronk(&parse(&input));
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}
