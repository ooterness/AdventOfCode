/// Advent of Code 2025, Day 7
/// Copyright 2025 by Alex Utter

use aocfetch;
use std::collections::HashSet;

type Row = HashSet<usize>;
type Beams = (Row, usize);

struct Splitters {
    split: Vec<Row>,
    start: Row,
}

impl Splitters {
    fn new(input: &str) -> Self {
        let mut split = Vec::new();
        let mut start = Row::new();
        for line in input.trim().lines() {
            let mut row = Row::new();
            for (col, ch) in line.trim().chars().enumerate() {
                if ch == 'S' { start.insert(col); }
                if ch == '^' { row.insert(col); }
            }
            split.push(row);
        }
        return Splitters { split:split, start:start };
    }

    fn row(&self, split:&Row, beams:&Beams) -> Beams {
        let mut out = Row::new();
        let mut ctr = beams.1;
        for beam in beams.0.iter() {
            if split.contains(beam) {
                ctr += 1;
                out.insert(beam - 1);
                out.insert(beam + 1);
            } else {
                out.insert(*beam);
            }
        }
        return (out, ctr);
    }

    fn split(&self) -> Beams {
        let mut beams = (self.start.clone(), 0usize);
        for split in self.split.iter() {
            beams = self.row(split, &beams);
        }
        return beams;
    }
}

fn part1(input: &str) -> usize {
    Splitters::new(input).split().1
}

fn part2(input: &str) -> usize {
    0 // TODO
}

const EXAMPLE: &'static str = "\
    .......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 7).unwrap();

    assert_eq!(part1(EXAMPLE), 21);
    assert_eq!(part2(EXAMPLE), 40);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
