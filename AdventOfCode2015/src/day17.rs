/// Advent of Code 2015, Day 17
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct PathCount {
    count: HashMap<usize, usize>,
}

impl PathCount {
    fn zero() -> Self {
        Self { count: HashMap::new() }
    }

    fn one() -> Self {
        Self { count: HashMap::from([(0,1)]) }
    }

    fn min(&self) -> usize {
        *self.count.iter().min().unwrap().1
    }

    fn sum(&self) -> usize {
        self.count.values().sum()
    }
}

struct Containers {
    // Lookup by total capacity, then number of containers -> Count paths.
    paths: HashMap<usize, PathCount>,
}

impl Containers {
    // Count reachable states for a given set of containers.
    fn new(input: &str) -> Self {
        // Initial state: 0 capacity, 0 containers, 1 path.
        let mut tmp = Containers {
            paths: HashMap::from([(0, PathCount::one())])
        };
        // Update reachable-state counters for each container.
        for line in input.lines() {
            tmp.add(line.trim().parse().unwrap());
        }
        return tmp;
    }

    // Add a new container of the specified size.
    fn add(&mut self, new_size: usize) {
        for (old_size, path) in self.paths.clone().iter() {
            for (old_count, old_routes) in path.count.iter() {
                *self.paths.entry(*old_size + new_size)
                    .or_insert(PathCount::zero())
                    .count.entry(*old_count+1)
                    .or_insert(0) += *old_routes;
            }
        }
    }

    // Count all possible ways to reach a given total capacity.
    fn part1(&self, size: usize) -> usize {
        self.paths.get(&size).unwrap_or(&PathCount::zero()).sum()
    }

    // Find minimum number of containers required to reach a given capacity.
    // Count the number of solutions using that number of containers.
    fn part2(&self, size: usize) -> usize {
        self.paths.get(&size).unwrap_or(&PathCount::zero()).min()
    }
}

fn part1(input: &str) -> usize {
    Containers::new(input).part1(150)
}

fn part2(input: &str) -> usize {
    Containers::new(input).part2(150)
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 17).unwrap();

    // Unit tests based on the provided examples:
    let test = Containers::new("20\n15\n10\n5\n5");
    assert_eq!(test.part1(25), 4);
    assert_eq!(test.part2(25), 3);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
