/// Advent of Code 2025, Day 6
/// Copyright 2025 by Alex Utter

use aocfetch;

struct Problem {
    dat: Vec<i64>,
    op: char,
}

impl Problem {
    fn new(input: &str) -> Self {
        let op = input.chars().next().unwrap();
        return Problem { dat:Vec::new(), op:op };
    }

    fn solve(&self) -> i64 {
        match self.op {
            '+' => self.dat.iter().sum(),
            '*' => self.dat.iter().product(),
            _   => panic!("Invalid operand: '{}'", self.op),
        }
    }
}

struct ProblemSet {
    problems: Vec<Problem>,
}

impl ProblemSet {
    fn new(input: &str) -> Self {
        // Read the input grid.
        let mut grid: Vec<Vec<i64>> = Vec::new();
        let mut problems: Vec<Problem> = Vec::new();
        for row in input.trim().lines() {
            if row.contains('+') || row.contains('*') {
                for item in row.trim().split_whitespace() {
                    problems.push(Problem::new(item));
                }
            } else {
                grid.push(row.trim().split_whitespace()
                    .filter_map( |n| n.parse::<i64>().ok() )
                    .collect());
            }
        }
        // Add each column to the appropriate object.
        for row in grid.into_iter() {
            assert!(row.len() == problems.len());
            for (c,n) in row.into_iter().enumerate() {
                problems[c].dat.push(n);
            }
        }
        return ProblemSet { problems:problems };
    }
}

fn part1(input: &ProblemSet) -> i64 {
    input.problems.iter().map(|p| p.solve()).sum()
}

fn part2(input: &ProblemSet) -> i64 {
    0
}

const EXAMPLE: &'static str = "\
    123 328  51 64
     45 64  387 23
      6 98  215 314
    *   +   *   +";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 6).unwrap();

    let example = ProblemSet::new(EXAMPLE);
    assert_eq!(part1(&example), 4277556);
    assert_eq!(part2(&example), 0);

    let time = std::time::Instant::now();
    let data = ProblemSet::new(&input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
