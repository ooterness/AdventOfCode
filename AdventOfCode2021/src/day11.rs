/// Day 11: https://adventofcode.com/2021/day/11
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
#[path = "grid.rs"] mod grid;

#[derive(Clone)]
struct Cave {
    crabs: grid::Grid<u8>,
}

impl Cave {
    fn new(filename: &str) -> Cave {
        Cave { crabs: grid::read_grid(filename) }
    }

    fn explode(&mut self, crab: &grid::RowCol) -> u64 {
        // Mark this crab as exploded to prevent infinite loops.
        let mut count = 1u64;
        self.crabs.set(crab, 0);
        // Check each neighbor...
        for next in [crab.nw(), crab.nn(), crab.ne(), crab.ee(),
                     crab.se(), crab.ss(), crab.sw(), crab.ww()]
        {
            // Increment unexploded neighbors.
            let lvl = *self.crabs.get(&next).unwrap_or(&0);
            if lvl > 0 {self.crabs.set(&next, lvl+1);}
            // Check for chain explosions.
            if lvl >= 9 {count += self.explode(&next);}
        }
        return count
    }

    fn next(&mut self) -> u64 {
        // Increment all the energy counters by one...
        for crab in self.crabs.iter() {
            let lvl = *self.crabs.get(&crab).unwrap();
            self.crabs.set(&crab, lvl + 1);
        }
        // Explode any crabs at max energy...
        let mut count = 0u64;
        for crab in self.crabs.iter() {
            let lvl = *self.crabs.get(&crab).unwrap();
            if lvl > 9 {count += self.explode(&crab);}
        }
        return count
    }

    fn part1(&self, steps: usize) -> u64 {
        let mut caves = self.clone();
        let mut count = 0u64;
        for _n in 0..steps {count += caves.next();}
        return count
    }

    fn part2(&self) -> u64 {
        let mut caves = self.clone();
        let mut steps = 1u64;
        while caves.next() < 100 {steps += 1};
        return steps
    }
}

pub fn solve() {
    let test = Cave::new("input/test11.txt");
    let data = Cave::new("input/input11.txt");

    assert_eq!(test.part1(2), 35);
    assert_eq!(test.part1(10), 204);
    assert_eq!(test.part1(100), 1656);
    println!("Part1: {}", data.part1(100));

    assert_eq!(test.part2(), 195);
    println!("Part2: {}", data.part2());
}
