/// Advent of Code 2017, Day 19
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashMap;

type Rc = (i64, i64);
type Maze = HashMap<Rc, Option<char>>;

// Direction index 0/1/2/3 = South/East/North/West.
const DIR: [Rc;4] = [(1,0), (0,-1), (-1,0), (0,1)];

// Parse input string to decode the maze.
fn parse(input: &str) -> Maze {
    let mut maze = Maze::new();
    let lines = input.lines();
    for (r,row) in lines.enumerate() {
        for (c,col) in row.chars().enumerate() {
            let rc = (r as i64, c as i64);
            match col {
                ' ' => {},
                '|' => {maze.insert(rc, None);},
                '-' => {maze.insert(rc, None);},
                '+' => {maze.insert(rc, None);},
                 _  => {maze.insert(rc, Some(col));},
            }
        }
    }
    return maze;
}

// Find the maze entrance.
fn start(maze: &Maze) -> Option<Rc> {
    for (r,c) in maze.keys() {
        if *r == 0 {return Some((*r,*c));}
    }
    return None;
}

// Follow the path, noting each visited letter and total path length.
fn solve(input: &str) -> (String, usize) {
    let maze = parse(input);
    let mut rc = start(&maze).unwrap(); // Current position
    let mut dd = 0usize;                // Current direction
    let mut path = String::new();       // Letters visited
    let mut plen = 1usize;              // Total steps taken
    loop {
        // Add letters as we pass through them.
        if let Some(c) = maze[&rc] {path.push(c);}
        // Define directions for left, straight, right.
        let ddl = (dd + 3) % 4;
        let dds = (dd + 0) % 4;
        let ddr = (dd + 1) % 4;
        let rcl: Rc = (rc.0+DIR[ddl].0, rc.1+DIR[ddl].1);
        let rcs: Rc = (rc.0+DIR[dds].0, rc.1+DIR[dds].1);
        let rcr: Rc = (rc.0+DIR[ddr].0, rc.1+DIR[ddr].1);
        // Keep moving straight if possible, otherwise turn.
        if maze.contains_key(&rcs)      {rc = rcs; dd = dds;}
        else if maze.contains_key(&rcl) {rc = rcl; dd = ddl;}
        else if maze.contains_key(&rcr) {rc = rcr; dd = ddr;}
        else {break;}   // Reached end of maze?
        plen += 1;      // Count total steps...
    }
    return (path, plen);
}

const TEST: &str = concat![
    "     |          \n",
    "     |  +--+    \n",
    "     A  |  C    \n",
    " F---|----E|--+ \n",
    "     |  |  |  D \n",
    "     +B-+  +--+ \n"];

fn main() {
    // Fetch problem input from server.
    let input = fetch::get_data(2017, 19).unwrap();

    // Unit tests on provided examples.
    assert_eq!(solve(TEST).0, "ABCDEF");
    assert_eq!(solve(TEST).1, 38);

    // Solve for real input.
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
