/// Advent of Code 2023, Day 9
/// Copyright 2023 by Alex Utter

use aocfetch;

struct Sequence {
    val: Vec<i64>,
}

impl Sequence {
    fn new(line: &str) -> Self {
        let tmp = line.trim().split(' ').map(|x| x.parse().unwrap());
        Sequence { val: tmp.collect() }
    }

    fn prev(&self) -> i64 {
        if self.val.iter().all(|x| *x == 0) {return 0;}
        return self.val.first().unwrap() - self.diff().prev();
    }

    fn next(&self) -> i64 {
        if self.val.iter().all(|x| *x == 0) {return 0;}
        return self.val.last().unwrap() + self.diff().next();
    }

    fn diff(&self) -> Sequence {
        let tmp = (1..self.val.len()).map(|n| self.val[n] - self.val[n-1]);
        Sequence { val: tmp.collect() }
    }
}

fn part1(input: &str) -> i64 {
    input.trim().lines().map(|s| Sequence::new(s).next()).sum()
}

fn part2(input: &str) -> i64 {
    input.trim().lines().map(|s| Sequence::new(s).prev()).sum()
}

const EXAMPLE: &'static str = "\
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 9).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 114);
    assert_eq!(part2(EXAMPLE), 2);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
