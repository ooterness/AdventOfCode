/// Advent of Code 2023, Day 23
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Rc(i32, i32);
type RcSet = HashSet<Rc>;

const DIR_N: Rc = Rc(-1,  0);
const DIR_S: Rc = Rc( 1,  0);
const DIR_W: Rc = Rc( 0, -1);
const DIR_E: Rc = Rc( 0,  1);
const DIRECTIONS: [Rc;4] = [DIR_N, DIR_S, DIR_W, DIR_E];

impl Rc {
    fn add(&self, other: &Rc) -> Self {
        Rc(self.0 + other.0, self.1 + other.1)
    }
}

struct Hike {
    posn: Rc,
    prev: RcSet,
}

impl Hike {
    fn new(maze: &Maze) -> Self {
        Hike { posn:maze.start, prev:RcSet::new() }
    }

    fn is_legal(&self, maze: &Maze, dir: &Rc) -> Option<Rc> {
        let posn = self.posn.add(dir);
        if !self.prev.contains(&posn) {
            let ok = match maze.paths.get(&posn) {
                Some(&'.') => true,
                Some(&'^') => maze.part2 || *dir == DIR_N,
                Some(&'v') => maze.part2 || *dir == DIR_S,
                Some(&'>') => maze.part2 || *dir == DIR_E,
                Some(&'<') => maze.part2 || *dir == DIR_W,
                _          => false,
            };
            if ok {return Some(posn);}
        }
        return None;
    }

    fn follow(&self, maze: &Maze) -> Option<Rc> {
        let next: Vec<Rc> = DIRECTIONS.iter()
            .filter_map(|dir| self.is_legal(maze, dir)).collect();
        if next.len() == 1 {
            return Some(next[0]);
        } else {
            return None;
        }
    }

    fn hike(&self, maze: &Maze, posn: Rc) -> Self {
        // Copy current state and move in the specified direction.
        let mut hike = Hike { posn:posn, prev:self.prev.clone() };
        hike.prev.insert(posn);
        // Keep moving as long as there is one valid direction.
        while let Some(next) = hike.follow(maze) {
            hike.posn = next; hike.prev.insert(next);
        }
        return hike;
    }

    fn next(&self, maze: &Maze) -> Vec<Hike> {
        return DIRECTIONS.iter()
            .filter_map(|dir| self.is_legal(maze, dir))
            .map(|rc| self.hike(maze, rc)).collect();
    }
}

struct Maze {
    paths:  HashMap<Rc, char>,
    part2:  bool,
    start:  Rc,
    exit:   Rc,
}

impl Maze {
    fn new(input: &str, part2: bool) -> Self {
        let mut paths = HashMap::new();
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                paths.insert(Rc(r as i32, c as i32), ch);
            }
        }
        let rmax = paths.keys().map(|rc| rc.0).max().unwrap();
        let start: Vec<Rc> = paths.iter()
            .filter(|(rc,ch)| rc.0 == 0 && **ch == '.')
            .map(|(rc,_)| *rc).collect();
        let exit: Vec<Rc> = paths.iter()
            .filter(|(rc,ch)| rc.0 == rmax && **ch == '.')
            .map(|(rc,_)| *rc).collect();
        return Maze {
            paths:  paths,
            part2:  part2,
            start:  start[0],
            exit:   exit[0],
        };
    }

    fn longest(&self) -> usize {
        let mut queue = vec![Hike::new(self)];
        let mut longest = 0usize;
        while let Some(hike) = queue.pop() {
            for next in hike.next(self).into_iter() {
                if next.posn == self.exit {
                    longest = core::cmp::max(longest, next.prev.len());
                } else {
                    queue.push(next);
                }
            }
        }
        return longest;
    }
}

fn part1(input: &str) -> usize {
    Maze::new(input, false).longest()
}

fn part2(input: &str) -> usize {
    // TODO: Faster algorithm? This takes a few hours on the full input.
    Maze::new(input, true).longest()
}

const EXAMPLE: &'static str = "\
    #.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 23).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 94);
    assert_eq!(part2(EXAMPLE), 154);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
