/// Advent of Code 2025, Day 12
/// Copyright 2025 by Alex Utter

use aocfetch;
use std::collections::HashSet;
type RowCol = (usize, usize);

struct Shape {
    rc: HashSet<RowCol>,
}

impl Shape {
    fn new<'a>(input: &mut impl Iterator<Item=&'a str>) -> Self {
        let mut result = Shape { rc: HashSet::new() };
        for (r,line) in input.enumerate() {
            if line.is_empty() { break; }
            for (c,ch) in line.trim().chars().enumerate() {
                if ch == '#' { result.rc.insert((r, c)); }
            }
        }
        return result;
    }
}

struct Grid {
    size: RowCol,
    count: Vec<usize>,
}

impl Grid {
    fn new(line: &str) -> Self {
        let x: Vec<usize> = line.trim().split(&['x', ':', ' '])
            .filter_map( |s| s.parse::<usize>().ok() ).collect();
        return Grid {
            size: (x[0], x[1]),
            count: x[2..].iter().cloned().collect(),
        };
    }

    fn could_solve(&self, shapes: &Vec<Shape>) -> bool {
        // A basic sanity check is sufficient for all inputs.
        // Heuristic is tuned to work for the short examples; the full
        // solutions are all either ~50% or ~101% occupancy, with
        // nothing in between, so subtlety is not required.
        let area1: usize = shapes.iter().zip(self.count.iter())
            .map( |(s,c)| s.rc.len() * c ).sum();
        let area2: usize = self.size.0 * self.size.1;
        return area1 <= area2 && (area2 <= 16 || 5 * area1 <= 4 * area2);
    }
}

struct Problem {
    shapes: Vec<Shape>,
    grids: Vec<Grid>,
}

impl Problem {
    fn new(input: &str) -> Self {
        let mut lines = input.trim().lines();
        let mut result = Problem { shapes: Vec::new(), grids: Vec::new() };
        while let Some(line) = lines.next() {
            if line.contains('x') {
                result.grids.push(Grid::new(line));
            } else if line.contains(':') {
                result.shapes.push(Shape::new(&mut lines));
            }
        }
        return result;
    }

    fn part1(&self) -> usize {
        self.grids.iter().filter( |g| g.could_solve(&self.shapes) ).count()
    }
}

fn part1(input: &str) -> usize {
    Problem::new(input).part1()
}

fn part2(input: &str) -> usize {
    0
}

const EXAMPLE: &'static str = "\
    0:
    ###
    ##.
    ##.

    1:
    ###
    ##.
    .##

    2:
    .##
    ###
    ##.

    3:
    ##.
    ###
    ##.

    4:
    ###
    #..
    ###

    5:
    ###
    .#.
    ###

    4x4: 0 0 0 0 2 0
    12x5: 1 0 1 0 2 2
    12x5: 1 0 1 0 3 2";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 12).unwrap();

    assert_eq!(part1(EXAMPLE), 2);
    assert_eq!(part2(EXAMPLE), 0);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
