/// Advent of Code 2024, Day 2
/// Copyright 2024 by Alex Utter

use aocfetch;

struct Report {
    cols: Vec<i64>,
}

impl Report {
    fn new(line: &str) -> Self {
        let cols = line.trim().split(' ')
            .map(|s| s.parse::<i64>().unwrap());
        Report { cols: cols.collect() }
    }

    // Convert an iterator to a difference vector.
    fn dvec<'a>(mut iter: impl Iterator<Item=&'a i64>) -> Vec<i64> {
        let mut result = Vec::new();
        let mut prev = iter.next().unwrap();
        while let Some(next) = iter.next() {
            result.push(next - prev);
            prev = next;
        }
        return result;
    }

    // Safe sequences are monotonic, with a step size of +1 to +3 or -1 to -3.
    // This function accepts the sequence iterator to be checked.
    fn safe_seq<'a>(iter: impl Iterator<Item=&'a i64>) -> bool {
        let dvec = Report::dvec(iter);
        let sign = dvec[0].signum();
        for diff in dvec.iter() {
            let delta = sign * diff;
            if delta < 1 || delta > 3 {return false;}
        }
        return true;
    }

    // Original sequence is safe?
    fn safe(&self) -> bool {
        Report::safe_seq(self.cols.iter())
    }

    // Trial-and-error: Does skipping any entry make it safe?
    fn safe_any(&self) -> bool {
        for skip in 0..self.cols.len() {
            let iter = self.cols.iter().enumerate()
                .filter_map(|(n,x)| if n == skip {None} else {Some(x)});
            if Report::safe_seq(iter) {return true;}
        }
        return false;
    }
}

struct Grid {
    rows: Vec<Report>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let rows = input.trim().lines()
            .map(|line| Report::new(line));
        Grid { rows: rows.collect() }
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    return grid.rows.iter()
        .filter(|r| r.safe()).count();
}

fn part2(input: &str) -> usize {
    let grid = Grid::new(input);
    return grid.rows.iter()
        .filter(|r| r.safe() || r.safe_any()).count();
}

const EXAMPLE: &'static str = "\
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 2).unwrap();

    assert_eq!(part1(EXAMPLE), 2);
    assert_eq!(part2(EXAMPLE), 4);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
