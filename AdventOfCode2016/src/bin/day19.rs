/// Advent of Code 2016, Day 19
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;

struct Elf {
    prev: usize,    // Label for previous elf
    next: usize,    // Label for next elf
}

impl Elf {
    fn new(index: usize, size: usize) -> Self {
        Elf {
            prev:  (index+size-1)%size,
            next:  (index+1)%size,
        }
    }
}

struct Circle {
    elves: HashMap<usize, Elf>,
}

impl Circle {
    fn new(size: usize) -> Self {
        let mut elves = HashMap::new();
        for n in 0..size {
            elves.insert(n, Elf::new(n, size));
        }
        Circle { elves:elves }
    }

    // Remove the designated elf from the loop.
    // Returns the number of presents held by that elf.
    fn remove(&mut self, lbl: usize) -> usize {
        let victim = self.elves.remove(&lbl).unwrap();
        self.elves.get_mut(&victim.prev).unwrap().next = victim.next;
        self.elves.get_mut(&victim.next).unwrap().prev = victim.prev;
        return victim.next;
    }

    // Simulate game using Part-1 rules.
    fn part1(&mut self) -> usize {
        let mut next = 0usize;
        while self.elves.len() > 1 {
            let victim = self.elves[&next].next;
            next = self.remove(victim);
        }
        return next;
    }

    // Simulate game using Part-2 rules.
    fn part2(&mut self) -> usize {
        let mut active = 0usize;
        let mut victim = self.elves.len() / 2;
        while self.elves.len() > 1 {
            // Remove current target. Double-skip if new length is even.
            victim = self.remove(victim);
            if self.elves.len() % 2 == 0 {
                victim = self.elves.get(&victim).unwrap().next;
            }
            // Advance to the next player.
            active = self.elves.get(&active).unwrap().next;
        }
        return active;
    }
}

fn part1(input: &str) -> usize {
    let size: usize = input.trim().parse().unwrap();
    Circle::new(size).part1() + 1
}

fn part2(input: &str) -> usize {
    let size: usize = input.trim().parse().unwrap();
    Circle::new(size).part2() + 1
}

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 19).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1("5"), 3);
    assert_eq!(part2("5"), 2);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
