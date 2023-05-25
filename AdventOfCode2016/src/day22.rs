/// Advent of Code 2016, Day 22
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct RowCol(usize, usize);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    used: u16,
    avail: u16,
    goal: bool,
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
        let size: u16 = tokens[3].parse().unwrap();
        let used: u16 = tokens[4].parse().unwrap();
        let avail: u16 = tokens[5].parse().unwrap();
        assert_eq!(size, used + avail); // Sanity check
        return Some((
            RowCol(row, col),
            Node {used:used, avail:avail, goal:false}
        ));
    }

    fn drain(&mut self) -> (u16, bool) {
        let xfer = (self.used, self.goal);
        self.avail += self.used;
        self.used   = 0;
        self.goal   = false;
        return xfer;
    }

    fn fill(&mut self, xfer: (u16, bool)) {
        self.avail -= xfer.0;
        self.used  += xfer.0;
        self.goal   = self.goal || xfer.1;
    }

    fn size(&self) -> u16 {
        self.used + self.avail
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
        // Flag the goal node in the upper-right corner.
        let goal = RowCol(0, grid.size.1-1);
        grid.get_mut(&goal).goal = true;
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

    // Get a list of all possible moves.
    fn moves(&self) -> Vec<Grid> {
        let mut adj = Vec::new();
        for r in 0..self.size.0 {
            for c in 0..self.size.1 {
                let rc0 = RowCol(r, c);
                let aa = self.get(&rc0);
                for rc1 in self.adj(&rc0).iter() {
                    let bb = self.get(rc1);
                    // Destination MUST always be an empty node.
                    if bb.used == 0 && aa.viable(bb) {
                        let mut tmp = self.clone();
                        let xfer = tmp.get_mut(&rc0).drain();
                        tmp.get_mut(rc1).fill(xfer);
                        adj.push(tmp);
                    }
                }
            }
        }
        return adj;
    }

    fn idx(&self, rc: &RowCol) -> usize {
        rc.0 + self.size.0 * rc.1
    }

    fn get(&self, rc: &RowCol) -> &Node {
        let idx = self.idx(rc);
        self.nodes.get(idx).unwrap()
    }

    fn get_mut(&mut self, rc: &RowCol) -> &mut Node {
        let idx = self.idx(rc);
        self.nodes.get_mut(idx).unwrap()
    }

    // Force this grid to a simplified state:
    // Except for jumbo nodes, all nodes are of similar size.
    // --> Don't bother tracking individual sizes, just 0/1.
    fn simplify(&self) -> Self {
        let threshold = 2 * self.nodes[0].size();
        let mut temp = self.clone();
        for node in temp.nodes.iter_mut() {
            if node.size() < threshold {
                node.avail = 1;
                node.used  = if node.used > 0 {1} else {0};
            } else {
                node.avail = 0;
                node.used  = 999;
            }
        }
        return temp;
    }

    // Is this grid in a solved state (Part-2)
    fn solved(&self) -> bool {
        self.get(&RowCol(0, 0)).goal
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

fn part2(input: &str) -> usize {
    // Read and simplify the initial state.
    let init = Grid::new(input).simplify();
    // Set up the search queue.
    let mut queue: VecDeque<(Grid,usize)> = VecDeque::new();
    let mut visit: HashSet<Grid> = HashSet::new();
    queue.push_back( (init.clone(), 0) );
    visit.insert(init.clone());
    // Breadth first search
    let mut mprev = 0usize; //???
    while let Some((grid,moves)) = queue.pop_front() {
        if moves > mprev {mprev = moves; println!("{}-{}", moves, queue.len());} //???
        for next in grid.moves().into_iter() {
            if next.solved() {
                return moves+1;
            } else if visit.insert(next.clone()) {
                queue.push_back( (next, moves+1) );
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
    let input = fetch::get_data(2016, 22).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), 7);
    assert_eq!(part2(TEST), 7);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
