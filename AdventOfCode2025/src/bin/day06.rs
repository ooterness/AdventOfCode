/// Advent of Code 2025, Day 6
/// Copyright 2025 by Alex Utter

use aocfetch;

struct Problem {
    dat: Vec<i64>,
    op: char,
}

impl Problem {
    fn new(op: char) -> Self {
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
    // Read input using Part 1 rules.
    fn parse1(input: &str) -> Self {
        // Read the input grid, one block at a time.
        let mut grid: Vec<Vec<i64>> = Vec::new();
        let mut problems: Vec<Problem> = Vec::new();
        for row in input.trim().lines() {
            if row.contains('+') || row.contains('*') {
                for item in row.trim().split_whitespace() {
                    let op = item.chars().next().unwrap();
                    problems.push(Problem::new(op));
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

    // Read input using Part 2 rules.
    fn parse2(input: &str) -> Self {
        // Read the input grid, one character at a time.
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut posn: Vec<usize> = Vec::new();
        let mut problems: Vec<Problem> = Vec::new();
        for row in input.lines() {
            if row.contains('+') || row.contains('*') {
                for (c,ch) in row.chars().enumerate() {
                    if ch == '+' || ch == '*' {
                        posn.push(c);
                        problems.push(Problem::new(ch));
                    }
                }
            } else {
                grid.push(row.chars().collect());
            }
        }
        // Add each column of numbers to the appropriate object.
        // (Start from the column with the operand, then scan right...)
        for p in 0..problems.len() {
            let mut c: usize = posn[p];
            loop {
                let col: String = grid.iter()
                    .filter_map( |row| row.get(c) ).collect();
                if let Ok(n) = col.trim().parse::<i64>() {
                    problems[p].dat.push(n); c += 1;
                } else {
                    break;
                }
            }
        }
        return ProblemSet { problems:problems };
    }
}

fn part1(input: &str) -> i64 {
    let problems = ProblemSet::parse1(input);
    return problems.problems.iter().map(|p| p.solve()).sum();
}

fn part2(input: &str) -> i64 {
    let problems = ProblemSet::parse2(input);
    return problems.problems.iter().map(|p| p.solve()).sum();
}

const EXAMPLE: &'static str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 6).unwrap();

    assert_eq!(part1(EXAMPLE), 4277556);
    assert_eq!(part2(EXAMPLE), 3263827);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
