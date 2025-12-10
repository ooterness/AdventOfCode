/// Advent of Code 2025, Day 9
/// Copyright 2025 by Alex Utter

use aocfetch;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;

type RowCol = (i64, i64);

struct Polygon {
    rc: Vec<RowCol>,
    rows: HashSet<i64>,
    cols: HashSet<i64>,
}

impl Polygon {
    fn new(input: &str) -> Self {
        let mut poly = Polygon {
            rc: Vec::new(),
            rows: HashSet::new(),
            cols: HashSet::new(),
        };
        for line in input.trim().lines() {
            let rc: Vec<i64> = line.trim().split(',')
                .filter_map( |s| s.parse::<i64>().ok() )
                .collect();
            poly.rc.push((rc[0], rc[1]));
            poly.rows.insert(rc[0]);
            poly.cols.insert(rc[1]);
        }
        return poly;
    }

    // Given two corners, find the area of the contained rectangle.
    fn area(&self, m:usize, n:usize) -> i64 {
        let dr = (self.rc[m].0 - self.rc[n].0).abs() + 1;
        let dc = (self.rc[m].1 - self.rc[n].1).abs() + 1;
        return dr * dc;
    }

    // Given two corners, normalize the bounding rectangle.
    fn rect(&self, m:usize, n:usize) -> (i64, i64, i64, i64) {
        ( min(self.rc[m].0, self.rc[n].0),
          max(self.rc[m].0, self.rc[n].0),
          min(self.rc[m].1, self.rc[n].1),
          max(self.rc[m].1, self.rc[n].1) )
    }

    // Fetch normalized perimeter segment #N as a rectangle.
    fn seg(&self, n:usize) -> (i64, i64, i64, i64) {
        let m = if n > 0 {n-1} else {self.rc.len() - 1};
        return self.rect(m, n);
    }

    // Check if all points along perimeter are inside the polygon.
    // (Scan only unique row and column coordinates of interest.)
    fn check_perimeter(&self, m:usize, n:usize) -> bool {
        let (r0, r1, c0, c1) = self.rect(m, n);
        for &r in self.rows.iter() {
            if r0 <= r && r <= r1 {
                if !self.contains((r, c0)) { return false; }
                if !self.contains((r, c1)) { return false; }
            }
        }
        for &c in self.cols.iter() {
            if c0 <= c && c <= c1 {
                if !self.contains((r0, c)) { return false; }
                if !self.contains((r1, c)) { return false; }
            }
        }
        return true;
    }

    // Check if a given point is inside the polygon.
    fn contains(&self, rc:RowCol) -> bool {
        // Any point on the perimeter is "inside".
        for m in 0..self.rc.len() {
            let (r0, r1, c0, c1) = self.seg(m);
            if r0 <= rc.0 && rc.0 <= r1 && c0 <= rc.1 && rc.1 <= c1 { return true; }
        }
        // For all other points, scan to far left and count crossings.
        let mut count = 0usize;
        for m in 0..self.rc.len() {
            let (r0, r1, c0, c1) = self.seg(m);
            if c0 == c1 && r0 <= rc.0 && rc.0 <= r1 && rc.1 > c0 { count += 1; }
        }
        return (count % 2) == 1;
    }
}

fn part1(input: &str) -> i64 {
    let poly = Polygon::new(input);
    let mut best = 0i64;
    for m in 0..poly.rc.len()-1 {
        for n in m+1..poly.rc.len() {
            let next = poly.area(m, n);
            if next > best { best = next; }
        }
    }
    return best;
}

// 3436560 is too low???
fn part2(input: &str) -> i64 {
    let poly = Polygon::new(input);
    let mut best = 0i64;
    for m in 0..poly.rc.len()-1 {
        for n in m+1..poly.rc.len() {
            let next = poly.area(m, n);
            if next > best && poly.check_perimeter(m, n) { best = next; }
        }
    }
    return best;
}

const EXAMPLE: &'static str = "\
    7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 9).unwrap();

    assert_eq!(part1(EXAMPLE), 50);
    assert_eq!(part2(EXAMPLE), 24);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
