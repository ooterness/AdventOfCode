/// Advent of Code 2015, Day 9
/// Copyright 2023 by Alex Utter

use std::collections::HashMap;
use std::collections::VecDeque;
#[path = "fetch.rs"] mod fetch;

struct Cities {
    labels: HashMap<String, usize>,
    dist: HashMap<(usize,usize), u64>,
}

impl Cities {
    // Build the distance matrix for a list of cities.
    fn new(input: &str) -> Cities {
        let mut tmp = Cities {labels: HashMap::new(), dist: HashMap::new()};
        for line in input.lines() {
            let tok: Vec<&str> = line.trim().split(' ').collect();
            assert_eq!(tok.len(), 5);
            let from = tmp.label2idx(tok[0]);
            let to   = tmp.label2idx(tok[2]);
            let dist: u64 = tok[4].parse().unwrap();
            tmp.dist.insert((from,to), dist);
            tmp.dist.insert((to,from), dist);
        }
        return tmp;
    }

    // Convert city-name to city-index.
    fn label2idx(&mut self, label: &str) -> usize {
        if let Some(idx) = self.labels.get(label) {
            return *idx;        // Already exists in cache.
        } else {
            let new_idx = self.labels.len();
            self.labels.insert(label.to_string(), new_idx);
            return new_idx;     // Create a new label.
        }
    }

    fn salesman(&self, longest: bool) -> u64 {
        // Each state holds the following:
        //  * Index of Santa's current location.
        //  * Cumulative bit-mask of unvisited cities.
        //  * Cumulative distance traveled.
        type State = (usize, u32, u64);
        let mut queue: VecDeque<State> = VecDeque::new();
        // Try each possible starting location...
        let num_cities = self.labels.len();
        let init_mask = (1u32 << num_cities) - 1;
        for n in 0..num_cities {
            queue.push_back((n, init_mask - (1u32 << n), 0));
        }
        // Brute-force search until we visit all cities.
        let mut max_dist = 0u64;
        let mut min_dist = std::u64::MAX;
        while let Some((city, mask, accum)) = queue.pop_front() {
            for n in 0..num_cities {
                if city == n {continue;}
                let dist = accum + self.dist[&(city,n)];
                let next = 1u32 << n;
                if mask == next {
                    // Last city on the tour. Best path?
                    if dist > max_dist {max_dist = dist;}
                    if dist < min_dist {min_dist = dist;}
                } else if mask & next > 0 {
                    // Try visiting the next city on the tour.
                    queue.push_back((n, mask-next, dist));
                }
            }
        }
        return if longest {max_dist} else {min_dist};
    }
}

fn part1(input: &str) -> u64
{
    Cities::new(input).salesman(false)
}

fn part2(input: &str) -> u64
{
    Cities::new(input).salesman(true)
}

// Example from the problem statement:
const TEST: &str = "\
    London to Dublin = 464
    London to Belfast = 518
    Dublin to Belfast = 141";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 9).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(TEST), 605);
    assert_eq!(part2(TEST), 982);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
