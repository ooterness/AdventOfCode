/// Advent of Code 2024, Day 10
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;

// Row + Column coordinate
type Rc = (usize, usize);
type Delta = (isize, isize);
const DIRECTIONS: [Delta;4] = [(-1,0), (0,1), (1,0), (0,-1)];

// Data structures used to compute score and rating.
type Path = HashMap<Rc, usize>;

// A topographic map (height at each row, column)
struct Grid {
    h: Vec<Vec<usize>>,
    trail: Vec<Rc>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        // Parse the input data
        let mut data = Vec::new();
        let mut cmax = 0usize;
        for row_str in input.trim().lines() {
            let row_int: Vec<usize> = row_str.trim().chars()
                .map(|ch| ch.to_digit(10).unwrap_or(999) as usize)
                .collect();
            if cmax < row_int.len() {cmax = row_int.len();}
            data.push(row_int);
        }
        let rmax = data.len();
        // Find all the "trailheads" (height = 0)
        let mut trail = Vec::new();
        for r in 0..rmax {
            for c in 0..cmax {
                if data[r][c] == 0 {trail.push((r,c));}
            }
        }
        return Grid {h:data, trail:trail, rows:rmax, cols:cmax};
    }

    fn step_up(&self, prev: Path) -> Path {
        let mut next = Path::new();
        if prev.is_empty() {return next;}
        for (rc,ct) in prev.into_iter() {
            let href = self.h[rc.0][rc.1] + 1;
            for d in DIRECTIONS {
                let r2: usize = rc.0.overflowing_add_signed(d.0).0;
                let c2: usize = rc.1.overflowing_add_signed(d.1).0;
                if r2 >= self.rows || c2 >= self.cols {continue;}
                if self.h[r2][c2] != href {continue;}
                *next.entry((r2,c2)).or_insert(0) += ct;
            }
        }
        return next;
    }

    // Search function used for both heuristics.
    fn solve(&self, head: &Rc) -> Path {
        let mut path = Path::from([(*head, 1)]);
        for _ in 0..9 {path = self.step_up(path);}
        return path;
    }

    // The "score" for a trailhead is the number of reachable summits.
    fn score(&self, head: &Rc) -> usize {
        self.solve(head).len()
    }

    // The "rating" for a trailhead is the number of distinct paths.
    fn rating(&self, head: &Rc) -> usize {
        self.solve(head).values().sum()
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    return grid.trail.iter().map(|t| grid.score(t)).sum();
}

fn part2(input: &str) -> usize {
    let grid = Grid::new(input);
    return grid.trail.iter().map(|t| grid.rating(t)).sum();
}


const EXAMPLE1: &'static str = "\
    10..9..
    2...8..
    3...7..
    4567654
    ...8..3
    ...9..2
    .....01";

const EXAMPLE2: &'static str = "\
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732";

const EXAMPLE3: &'static str = "\
    .....0.
    ..4321.
    ..5..2.
    ..6543.
    ..7..4.
    ..8765.
    ..9....";

const EXAMPLE4: &'static str = "\
    012345
    123456
    234567
    345678
    4.6789
    56789.";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 10).unwrap();

    assert_eq!(part1(EXAMPLE1), 3);
    assert_eq!(part1(EXAMPLE2), 36);
    assert_eq!(part2(EXAMPLE2), 81);
    assert_eq!(part2(EXAMPLE3), 3);
    assert_eq!(part2(EXAMPLE4), 227);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
