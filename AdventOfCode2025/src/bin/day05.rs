/// Advent of Code 2025, Day 5
/// Copyright 2025 by Alex Utter

use aocfetch;
use std::collections::LinkedList;

type Range = (usize, usize);

fn combine(a:&Range, b:&Range) -> Range {
    (std::cmp::min(a.0, b.0),
     std::cmp::max(a.1, b.1))
}

fn overlap(a:&Range, b:&Range) -> bool {
    (a.0 <= b.0 && b.0 <= a.1) ||
    (a.0 <= b.1 && b.1 <= a.1) ||
    (b.0 <= a.0 && a.0 <= b.1) ||
    (b.0 <= a.1 && a.1 <= b.1)
}

struct Problem {
    fresh: Vec<Range>,
    items: Vec<usize>,
}

impl Problem {
    fn new(input: &str) -> Self {
        let mut tmp = Problem {
            fresh: Vec::new(),
            items: Vec::new(),
        };
        for line in input.trim().lines() {
            let parts: Vec<usize> = line.trim().split('-')
                .filter_map( |s| s.parse::<usize>().ok() )
                .collect();
            if parts.len() > 1 {
                tmp.fresh.push((parts[0], parts[1]));
            } else if parts.len() > 0 {
                tmp.items.push(parts[0]);
            }
        }
        return tmp;
    }

    fn is_fresh(&self, item: usize) -> bool {
        self.fresh.iter().any( |&(ll,rr)| ll <= item && item <= rr )
    }

    fn count_fresh(&self) -> usize {
        self.items.iter().filter( |&n| self.is_fresh(*n) ).count()
    }

    fn count_total(&self) -> usize {
        self.fresh.iter().map( |&(ll,rr)| 1+rr-ll ).sum()
    }

    fn simplify(&self) -> Self {
        // Combine all overlapping ranges:
        //  * Put all unchecked ranges in a queue.
        //  * For each item in the queue:
        //      * Test against the simplified range set.
        //      * Pop any that overlap.
        //      * Fold overlapping range(s) into a single output.
        //      * Append the combined range to the simplified set.
        let mut accum = LinkedList::new();
        let mut queue = self.fresh.clone();
        while let Some(next) = queue.pop() {
            let comb:Range = accum
                .extract_if( |prev| overlap(&next, prev) )
                .fold(next, |acc,prev| combine(&acc, &prev));
            accum.push_back(comb);
        }
        return Problem {
            fresh: accum.into_iter().collect(),
            items: self.items.clone(),
        };
    }
}

fn part1(input: &Problem) -> usize {
    input.count_fresh()
}

fn part2(input: &Problem) -> usize {
    input.simplify().count_total()
}

const EXAMPLE: &'static str = "\
    3-5\n 10-14\n 16-20\n 12-18\n
    1\n 5\n 8\n 11\n 17\n 32";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 5).unwrap();

    let example = Problem::new(EXAMPLE);
    assert_eq!(part1(&example), 3);
    assert_eq!(part2(&example), 14);

    let time = std::time::Instant::now();
    let data = Problem::new(&input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
