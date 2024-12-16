/// Advent of Code 2024, Day 16
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Rc = (usize, usize);       // Row & column position
type Delta = (isize, isize);    // Row & column difference
type State = (Rc, usize);       // Row/column & direction
type Search = (usize, State);   // Cost & search state
const DIRECTIONS: [Delta;4] = [(0,1), (1,0), (0,-1), (-1,0)];

fn add(rc:&Rc, mv:&Delta) -> Rc {
    (rc.0.saturating_add_signed(mv.0),
     rc.1.saturating_add_signed(mv.1))
}

struct Maze {
    start: Rc,
    end:   Rc,
    walls: HashSet<Rc>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut maze = Maze {
            start: (0, 0),
            end:   (0, 0),
            walls: HashSet::new(),
        };
        for (r,row) in input.trim().lines().enumerate() {
            for (c,ch) in row.trim().chars().enumerate() {
                match ch {
                    '#' => {maze.walls.insert((r,c));},
                    'S' => {maze.start = (r,c);},
                    'E' => {maze.end = (r,c);},
                    _   => {},
                }
            }
        }
        return maze;
    }

    fn adj(&self, state:Search) -> Vec<Search> {
        let (cost, (rc, dir)) = state;
        let mut next = Vec::new();
        // Can we move forward?
        let fwd = add(&rc, &DIRECTIONS[dir]);
        if !self.walls.contains(&fwd) {next.push((cost+1, (fwd,dir)));}
        // We can always turn left or right.
        next.push((cost+1000, (rc, (dir+1)%4)));
        next.push((cost+1000, (rc, (dir+3)%4)));
        return next;
    }

    // Dijkstra's algorithm.
    fn solve(&self) -> (usize, HashSet<Rc>) {
        // Create search state. (Use "Reverse" to make a min-heap.)
        let mut costs: HashMap<State, usize> = HashMap::new();
        let mut prevs: HashMap<State, HashSet<State>> = HashMap::new();
        let mut queue: BinaryHeap<Reverse<Search>> = BinaryHeap::new();
        let mut best_cost = usize::MAX;
        // Set initial condition, facing east.
        let init: State = (self.start, 0);
        costs.insert(init, 0);
        queue.push(Reverse((0, init)));
        // Execute search using the priority queue...
        // Once we've found a solution, keep searching until all
        // remaining paths exceed the cost of the best path.
        while let Some(Reverse(prev)) = queue.pop() {
            if prev.0 >= best_cost {break;}     // Done searching?
            for (cost, state) in self.adj(prev) {
                let ref_cost = *costs.get(&state).unwrap_or(&usize::MAX);
                if cost < ref_cost {
                    // Is this a valid solution?  Set the halting threshold.
                    if state.0 == self.end && cost < best_cost {best_cost = cost;}
                    // Improvement -> Replace previous record.
                    costs.insert(state, cost);
                    prevs.insert(state, HashSet::from([prev.1]));
                    queue.push(Reverse((cost, state)));
                } else if cost == ref_cost {
                    // Tie -> Record all possible previous states.
                    prevs.entry(state).or_insert(HashSet::new()).insert(prev.1);
                }
            }
        }
        return (best_cost, self.trace(&prevs));
    }

    // Back-propagate to find all possible minimum-cost paths.
    fn trace(&self, prevs: &HashMap<State, HashSet<State>>) -> HashSet<Rc> {
        // Start search from each possible end-state.
        let mut queue: VecDeque<State> = (0..4)
            .map(|d| (self.end, d))
            .filter(|s| prevs.contains_key(&s))
            .collect();
        // Back-propagate.
        let mut path: HashSet<Rc> = HashSet::from([self.end]);
        while let Some(state) = queue.pop_front() {
            for prev in prevs[&state].iter() {
                path.insert(prev.0);
                if prevs.contains_key(prev) {queue.push_back(*prev);}
            }
        }
        return path;
    }
}

fn part1(input: &str) -> usize {
    Maze::new(input).solve().0
}

fn part2(input: &str) -> usize {
    Maze::new(input).solve().1.len()
}

const EXAMPLE1: &'static str = "\
    ###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############";

const EXAMPLE2: &'static str = "\
    #################
    #...#...#...#..E#
    #.#.#.#.#.#.#.#.#
    #.#.#.#...#...#.#
    #.#.#.#.###.#.#.#
    #...#.#.#.....#.#
    #.#.#.#.#.#####.#
    #.#...#.#.#.....#
    #.#.#####.#.###.#
    #.#.#.......#...#
    #.#.###.#####.###
    #.#.#...#.....#.#
    #.#.#.#####.###.#
    #.#.#.........#.#
    #.#.#.#########.#
    #S#.............#
    #################";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 16).unwrap();

    assert_eq!(part1(EXAMPLE1), 7036);
    assert_eq!(part1(EXAMPLE2), 11048);
    assert_eq!(part2(EXAMPLE1), 45);
    assert_eq!(part2(EXAMPLE2), 64);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
