/// Advent of Code 2023, Day 10
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const DEBUG: bool = true;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Rc(isize, isize);        // Row + column

const DIR_NW: Rc = Rc(-1,-1);
const DIR_N:  Rc = Rc(-1, 0);
const DIR_NE: Rc = Rc(-1, 1);
const DIR_E:  Rc = Rc( 0, 1);
const DIR_SE: Rc = Rc( 1, 1);
const DIR_S:  Rc = Rc( 1, 0);
const DIR_SW: Rc = Rc( 1,-1);
const DIR_W:  Rc = Rc( 0,-1);
const DIRECTIONS: [Rc;4] = [DIR_N, DIR_E, DIR_S, DIR_W];

struct Pipe {
    next: [bool;4],             // Connections to N/E/S/W
}

struct Maze {
    pipes: HashMap<Rc, Pipe>,   // Pipes at each R/C coordinate
    start: Rc,                  // Starting coordinate
    size:  Rc,                  // Maximum rows and columns
}

impl Rc {
    fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }

    fn expand(&self) -> Self {
        Self(3*self.0+1, 3*self.1+1)
    }

    fn contract(&self) -> Self {
        Self(self.0/3, self.1/3)
    }
}

impl Pipe {
    fn new(ch: char) -> Self {
        Pipe { next: match ch {
            '|' => [true, false, true, false],
            '-' => [false, true, false, true],
            'L' => [true, true, false, false],
            'J' => [true, false, false, true],
            '7' => [false, false, true, true],
            'F' => [false, true, true, false],
            _   => [false, false, false, false],
        } }
    }
}

impl Maze {
    fn new(input: &str) -> Self {
        // Build the maze and note starting position.
        let mut pipes = HashMap::new();
        let mut start = Rc(0,0);
        let mut rcmax = Rc(0,0);
        for (r,line) in input.trim().lines().enumerate() {
            rcmax.0 = core::cmp::max(rcmax.0, r as isize);
            for (c,ch) in line.trim().chars().enumerate() {
                rcmax.1 = core::cmp::max(rcmax.1, c as isize);
                let rc = Rc(r as isize, c as isize);
                pipes.insert(rc, Pipe::new(ch));
                if ch == 'S' { start = rc; }
            }
        }
        // Reciprocal connections for the starting cell.
        for n in 0..4 {
            let rc2 = start.add(&DIRECTIONS[n]);
            if let Some(adj) = pipes.get(&rc2) {
                pipes.get_mut(&start).unwrap().next[n] = adj.next[(n+2)%4];
            }
        }
        return Maze { pipes:pipes, start:start, size:rcmax.add(&DIR_SE) };
    }

    // Breadth-first search along the main loop.
    fn trace(&self) -> HashMap<Rc, usize> {
        let mut result = HashMap::new();
        let mut queue: VecDeque<(Rc,usize)> = VecDeque::new();
        result.insert(self.start, 0);
        queue.push_back((self.start, 0));
        while let Some((rc,steps)) = queue.pop_front() {
            let pipe = self.pipes.get(&rc).unwrap();
            for (n,d) in DIRECTIONS.iter().enumerate() {
                let adj = rc.add(d);
                if pipe.next[n] && self.pipes.contains_key(&adj) && !result.contains_key(&adj) {
                    result.insert(adj, steps+1);
                    queue.push_back((adj, steps+1));
                }
            }
        }
        return result;
    }

    // Find a list of tiles outside the main loop.
    fn outer(&self) -> HashSet<Rc> {
        // First, expand the maze by a factor of three.
        let mut maze: HashSet<Rc> = HashSet::new();
        for (rc,p) in self.pipes.iter() {
            // Diagonals are always passable.
            let rc2 = rc.expand();
            maze.insert(rc2.add(&DIR_NW));
            maze.insert(rc2.add(&DIR_NE));
            maze.insert(rc2.add(&DIR_SW));
            maze.insert(rc2.add(&DIR_SE));
            // Check connections to adjacent tiles.
            for (n,d) in DIRECTIONS.iter().enumerate() {
                if !p.next[n] {maze.insert(rc2.add(d));}
            }
        }
        // Add an outer ring to bypass weird pipes at edge.
        let start = Rc(0,0).expand().add(&DIR_NW);
        let rmax = 3 * self.size.0;
        let cmax = 3 * self.size.1;
        for r in -1..rmax {maze.insert(Rc(r,  -1));}    // Left
        for r in -1..rmax {maze.insert(Rc(r,cmax));}    // Right
        for c in -1..cmax {maze.insert(Rc(-1,  c));}    // Top
        for c in -1..cmax {maze.insert(Rc(rmax,c));}    // Bottom
        // Flood-fill starting from the upper-left corner.
        let mut result: HashSet<Rc> = HashSet::new();
        let mut queue: VecDeque<Rc> = VecDeque::new();
        result.insert(start);
        queue.push_back(start);
        while let Some(rc) = queue.pop_front() {
            for d in DIRECTIONS.iter() {
                let adj = rc.add(d);
                if maze.contains(&adj) && !result.contains(&adj) {
                    result.insert(adj);
                    queue.push_back(adj);
                }
            }
        }
        if DEBUG {
            for r in -1..=rmax {
                for c in -1..=cmax {
                    print!("{}", if result.contains(&Rc(r,c)) {'*'} else {' '});
                }
                println!(" Row {}", r+1);
            }
        }
        // Realign the result to the original grid.
        return result.iter()
            .map(|rc| rc.contract())
            .filter(|rc| self.pipes.contains_key(rc))
            .collect();
    }
}

fn part1(input: &str) -> usize {
    let maze = Maze::new(input);
    return *maze.trace().values().max().unwrap();
}

fn part2(input: &str) -> usize {
    let maze = Maze::new(input);
    return maze.pipes.len() - maze.outer().len();
}

const EXAMPLE1: &'static str = "\
    -L|F7
    7S-7|
    L|7||
    -L-J|
    L|-JF";

const EXAMPLE2: &'static str = "\
    ..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...";

const EXAMPLE3: &'static str = "\
    ...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........";

const EXAMPLE4: &'static str = "\
    ..........
    .S------7.
    .|F----7|.
    .||....||.
    .||....||.
    .|L-7F-J|.
    .|..||..|.
    .L--JL--J.
    ..........";

const EXAMPLE5: &'static str = "\
    .F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...";

const EXAMPLE6: &'static str = "\
    FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 10).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE1), 4);
    assert_eq!(part1(EXAMPLE2), 8);
    assert_eq!(part2(EXAMPLE3), 4);
    assert_eq!(part2(EXAMPLE4), 4);
    assert_eq!(part2(EXAMPLE5), 8);
    assert_eq!(part2(EXAMPLE6), 10);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
