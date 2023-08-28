/// Advent of Code 2015, Day 18
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    stuck: bool,
    lit: HashSet<(i64,i64)>,
}

impl Grid {
    fn new(input: &str, stuck: bool) -> Self {
        let mut tmp = Grid {
            rows:0, cols:0, stuck:stuck,
            lit:HashSet::new()
        };
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                if r >= tmp.rows {tmp.rows = r+1;}
                if c >= tmp.cols {tmp.cols = c+1;}
                if ch == '#' {tmp.lit.insert((r as i64, c as i64));}
            }
        }
        if stuck {tmp.corners();}
        return tmp;
    }

    fn count(&self) -> usize {
        self.lit.len()
    }

    fn corners(&mut self) {
        let r = self.rows as i64 - 1;
        let c = self.cols as i64 - 1;
        self.lit.insert((0, 0));
        self.lit.insert((r, 0));
        self.lit.insert((0, c));
        self.lit.insert((r, c));
    }

    fn neighbors(&self, r:i64, c:i64) -> usize {
        let nn = [(r-1,c-1), (r-1,c), (r-1,c+1),
                  (r,  c-1),          (r,  c+1),
                  (r+1,c-1), (r+1,c), (r+1,c+1)];
        nn.iter().map(|rc| self.lit.contains(&rc) as usize).sum()
    }

    fn step_one(&self) -> Self {
        let mut next = Grid {
            rows:self.rows, cols:self.cols, stuck:self.stuck,
            lit:HashSet::new()
        };
        for r in 0..self.rows as i64 {
            for c in 0..self.cols as i64 {
                let ct = self.neighbors(r, c);
                let st = self.lit.contains(&(r,c));
                if (st && ct == 2) || (ct == 3)
                    {next.lit.insert((r,c));}
            }
        }
        if self.stuck {next.corners();}
        return next;
    }

    fn step(&self, n: usize) -> Self {
        let mut tmp = self.clone();
        for _ in 0..n {tmp = tmp.step_one();}
        return tmp;
    }
}

fn part1(input: &str) -> usize {
    Grid::new(input, false).step(100).count()
}

fn part2(input: &str) -> usize {
    Grid::new(input, true).step(100).count()
}

const TEST: &str = "\
    .#.#.#
    ...##.
    #....#
    ..#...
    #.#..#
    ####..";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 18).unwrap();

    // Unit tests based on the provided examples:
    assert_eq!(Grid::new(TEST, false).step(0).count(), 15);
    assert_eq!(Grid::new(TEST, false).step(1).count(), 11);
    assert_eq!(Grid::new(TEST, false).step(2).count(), 8);
    assert_eq!(Grid::new(TEST, false).step(3).count(), 4);
    assert_eq!(Grid::new(TEST, false).step(4).count(), 4);
    assert_eq!(Grid::new(TEST, true).step(0).count(), 17);
    assert_eq!(Grid::new(TEST, true).step(1).count(), 18);
    assert_eq!(Grid::new(TEST, true).step(2).count(), 18);
    assert_eq!(Grid::new(TEST, true).step(3).count(), 18);
    assert_eq!(Grid::new(TEST, true).step(4).count(), 14);
    assert_eq!(Grid::new(TEST, true).step(5).count(), 17);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
