/// Advent of Code 2023, Day 10
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const DEBUG: bool = false;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Rc(isize, isize);        // Row + column

const DIR_NW: Rc = Rc(-1,-1);
const DIR_N:  Rc = Rc(-1, 0);
const DIR_E:  Rc = Rc( 0, 1);
const DIR_S:  Rc = Rc( 1, 0);
const DIR_W:  Rc = Rc( 0,-1);
const DIRECTIONS: [Rc;4] = [DIR_N, DIR_E, DIR_S, DIR_W];

struct Pipe {
    next: [bool;4],             // Connections to N/E/S/W
}

struct Maze {
    pipes: HashMap<Rc, Pipe>,   // Pipes at each R/C coordinate
    start: Rc,                  // Starting coordinate
}

impl Rc {
    fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
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
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
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
        return Maze { pipes:pipes, start:start };
    }

    // Print a HashSet for debugging.
    fn debug(set: &HashSet<Rc>) {
        let rmax = set.iter().map(|rc| rc.0).max().unwrap_or(1);
        let cmax = set.iter().map(|rc| rc.1).max().unwrap_or(1);
        for r in 0..=rmax {
            for c in 0..=cmax {
                print!("{}", if set.contains(&Rc(r,c)) {'*'} else {'-'});
            }
            println!(" Row {}", r);
        }
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
        if DEBUG {Self::debug(&result.keys().cloned().collect());}
        return result;
    }

    // Test if a given tile is inside the provided loop.
    fn inner_test(&self, path: &HashSet<Rc>, start: &Rc) -> bool {
        if path.contains(start) {return false;}
        let mut count = 0usize;             // Count wall-crossings.
        let mut pos = start.add(&DIR_NW);   // Scan until we reach edge...
        while let Some(pipe) = self.pipes.get(&pos) {
            if path.contains(&pos) {
                if pipe.next[0] && pipe.next[2] {count += 1;}
                if pipe.next[1] && pipe.next[3] {count += 1;}
                if pipe.next[0] && pipe.next[3] {count += 1;}
                if pipe.next[1] && pipe.next[2] {count += 1;}
            }
            pos = pos.add(&DIR_NW);
        }
        return (count % 2) > 0;
    }

    // List all the tiles completely inside the main loop.
    fn inner(&self) -> HashSet<Rc> {
        let path: HashSet<Rc> = self.trace().into_keys().collect();
        let mut result = HashSet::new();
        for rc in self.pipes.keys() {
            if self.inner_test(&path, rc) {result.insert(*rc);}
        }
        if DEBUG {Self::debug(&result);}
        return result;
    }
}

fn part1(input: &str) -> usize {
    let maze = Maze::new(input);
    return *maze.trace().values().max().unwrap();
}

fn part2(input: &str) -> usize {
    let maze = Maze::new(input);
    return maze.inner().len();
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
