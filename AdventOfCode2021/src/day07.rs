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

    // Leftmost and rightmost points?
    fn left(&self) -> i64   {*self.pos.iter().min().unwrap_or(&0)}
    fn right(&self) -> i64  {*self.pos.iter().max().unwrap_or(&0)}

    // Flat-rate alignment cost
    fn align(&self, fin: i64) -> i64 {
        self.pos.iter().map(|x| (x-fin).abs()).sum()
    }

    // Search all possible targets for the lowest alignment cost.
    fn align_best(&self) -> i64 {
        (self.left()..self.right()+1).map(|x| self.align(x)).min().unwrap_or(0)
    }

    // Increasing-rate alignment cost
    fn accel(&self, fin: i64) -> i64 {
        self.pos.iter().map(|x| (x-fin).abs()).map(|x| (x+1)*x/2).sum()
    }

    // Search all possible targets for the lowest alignment cost.
    fn accel_best(&self) -> i64 {
        (self.left()..self.right()+1).map(|x| self.accel(x)).min().unwrap_or(0)
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

    // Tests with scaling costs.
    assert_eq!(test.accel(2), 206);
    assert_eq!(test.accel(5), 168);
    assert_eq!(test.accel_best(), 168);

    // Real input.
    println!("Part2: {}", data.accel_best());
}
