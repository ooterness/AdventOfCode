/// Advent of Code 2016, Day 22
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct RowCol(usize, usize);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    size: usize,
    used: usize,
    avail: usize,
}

impl Node {
    fn new(line: &str) -> Option<(RowCol, Self)> {
        // Ignore header lines.
        if !line.starts_with("/dev") {return None;}
        // Example: "/dev/grid/node-x0-y0 88T 67T 21T 76%"
        let tokens: Vec<&str> = line.trim()
            .split([' ', 'x', 'y', '-', 'T'])
            .filter(|x| x.len() > 0)
            .collect();
        let col: usize = tokens[1].parse().unwrap();
        let row: usize = tokens[2].parse().unwrap();
        let size: usize = tokens[3].parse().unwrap();
        let used: usize = tokens[4].parse().unwrap();
        let avail: usize = tokens[5].parse().unwrap();
        assert_eq!(size, used + avail); // Sanity check
        return Some((
            RowCol(row, col),
            Node {size:size, used:used, avail:avail}
        ));
    }

    fn viable(&self, other: &Node) -> bool {
        self.used > 0 && other.avail >= self.used
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Grid {
    nodes: Vec<Node>,
    size: RowCol,
}

impl Grid {
    fn new(input: &str) -> Self {
        // Read the input, one line at a time.
        let mut grid = Grid { nodes: Vec::new(), size: RowCol(0,0) };
        for line in input.trim().lines() {
            if let Some((rc, node)) = Node::new(line.trim()) {
                // Confirm input is in expected column-by-column order.
                assert_eq!(grid.nodes.len(), grid.idx(&rc));
                if rc.0 >= grid.size.0 {grid.size.0 = rc.0 + 1;}
                if rc.1 >= grid.size.1 {grid.size.1 = rc.1 + 1;}
                grid.nodes.push(node);
            }
        }
        return grid;
    }

    // Get a list of positions adjacent to a given cell.
    fn adj(&self, rc: &RowCol) -> Vec<RowCol> {
        let mut tmp = Vec::new();
        if rc.0 > 0 {tmp.push(RowCol(rc.0-1, rc.1));}
        if rc.1 > 0 {tmp.push(RowCol(rc.0, rc.1-1));}
        if rc.0+1 < self.size.0 {tmp.push(RowCol(rc.0+1, rc.1));}
        if rc.1+1 < self.size.1 {tmp.push(RowCol(rc.0, rc.1+1));}
        return tmp;
    }

    // Get the location of the target data.
    fn find_goal(&self) -> RowCol {
        RowCol(0, self.size.1 - 1)
    }

    // Get the location of the empty node.
    fn find_empty(&self) -> RowCol {
        for r in 0..self.size.0 {
            for c in 0..self.size.1 {
                let rc = RowCol(r, c);
                if self.get(&rc).used == 0 {return rc;}
            }
        }
        panic!("Malformed input.");
    }

    fn idx(&self, rc: &RowCol) -> usize {
        rc.0 + self.size.0 * rc.1
    }

    fn get(&self, rc: &RowCol) -> &Node {
        let idx = self.idx(rc);
        self.nodes.get(idx).unwrap()
    }

    // Count the number of viable pars (Part-1)
    fn viable_pairs(&self) -> usize {
        let mut total = 0usize;
        for (a,aa) in self.nodes.iter().enumerate() {
            for (b,bb) in self.nodes.iter().enumerate() {
                if a != b && aa.viable(bb) {total += 1;}
            }
        }
        return total;
    }
}

fn part1(input: &str) -> usize {
    Grid::new(input).viable_pairs()
}

// Original problem is intractable, but the input can be greatly simplified:
//  * All nodes are either jumbo (i.e., too large to move anywhere) or
//    uniform in size (i.e., ~75% full with small variations in capacity).
//  * There is a single empty node with no data.
//  * As a result, there's no reason to track data as it moves around;
//    we're simply moving the empty node and dragging the goal with it.
//  * The only dynamic state is the location of the empty and goal nodes.
fn part2(input: &str) -> usize {
    // Read and simplify the initial state.
    let grid = Grid::new(input);
    let init = (grid.find_empty(), grid.find_goal());
    let threshold = 2 * grid.nodes[0].size;
    // Set up the search queue.
    let mut queue: VecDeque<(RowCol, RowCol, usize)> = VecDeque::new();
    let mut visit: HashSet<(RowCol, RowCol)> = HashSet::new();
    queue.push_back((init.0, init.1, 0));
    visit.insert(init);
    // Breadth first search until we can move goal data to (0,0).
    const END_GOAL: RowCol = RowCol(0, 0);
    while let Some((empty, goal, moves)) = queue.pop_front() {
        // Consider all moves adjacent to the empty node...
        for next in grid.adj(&empty).into_iter() {
            // Are we about to move the goal data?
            let new_goal = if next == goal {empty} else {goal};
            if new_goal == END_GOAL {
                return moves+1;     // Solved!
            } else if grid.get(&next).size > threshold {
                continue;           // Ignore jumbo nodes
            } else if visit.insert((next, new_goal)) {
                queue.push_back((next, new_goal, moves+1));
            }
        }
    }
    panic!("No solution.");
}

const TEST: &str = "\
    Filesystem            Size  Used  Avail  Use%
    /dev/grid/node-x0-y0   10T    8T     2T   80%
    /dev/grid/node-x0-y1   11T    6T     5T   54%
    /dev/grid/node-x0-y2   32T   28T     4T   87%
    /dev/grid/node-x1-y0    9T    7T     2T   77%
    /dev/grid/node-x1-y1    8T    0T     8T    0%
    /dev/grid/node-x1-y2   11T    7T     4T   63%
    /dev/grid/node-x2-y0   10T    6T     4T   60%
    /dev/grid/node-x2-y1    9T    8T     1T   88%
    /dev/grid/node-x2-y2    9T    6T     3T   66%";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 22).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), 7);
    assert_eq!(part2(TEST), 7);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
