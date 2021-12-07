/// Day 7: https://adventofcode.com/2021/day/7
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

struct Crabs {
    pos: Vec<i64>,
}

impl Crabs {
    fn new(filename: &str) -> Crabs {
        let lines = common::read_lines(filename);
        Crabs {pos: common::split_str_as(&lines[0], ',')}
    }

    fn align(&self, fin: i64) -> i64 {
        self.pos.iter().map(|x| (x-fin).abs()).sum()
    }

    fn align_best(&self) -> i64 {
        let left  = *self.pos.iter().min().unwrap();
        let right = *self.pos.iter().max().unwrap();
        let mut best = self.align(left);
        for n in left+1..right+1 {
            let next = self.align(n);
            if next < best {best = next;}
        }
        best
    }
}

pub fn solve() {
    // Test reference alignments.
    let test = Crabs::new("input/test07.txt");
    assert_eq!(test.align(1), 41);
    assert_eq!(test.align(2), 37);
    assert_eq!(test.align(3), 39);
    assert_eq!(test.align(10), 71);
    assert_eq!(test.align_best(), 37);

    // Real input.
    let data = Crabs::new("input/input07.txt");
    println!("Part1: {}", data.align_best());
}
