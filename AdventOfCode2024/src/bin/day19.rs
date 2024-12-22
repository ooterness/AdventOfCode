/// Advent of Code 2024, Day 19
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;

type Design = Vec<char>;

struct Tree {
    towel: bool,
    child: HashMap<char, Tree>,
}

impl Tree {
    fn new() -> Self {
        Tree { towel:false, child:HashMap::new() }
    }

    // Insert a new towel type into the lookup tree.
    fn add(&mut self, towel: &str) {
        let mut root = self;
        for ch in towel.trim().chars() {
            root = root.child.entry(ch).or_insert(Tree::new());
        }
        root.towel = true;
    }

    // From an initial state, find the size of each matching towel.
    fn next(&self, design: &Design, skip:usize) -> Vec<usize> {
        let mut root = self;
        let mut temp = Vec::new();
        for (n,ch) in design.iter().enumerate().skip(skip) {
            if let Some(next) = root.child.get(ch) {
                if next.towel {temp.push(n+1);}
                root = next;    // Keep scanning for longer matches
            } else {
                break;          // No matching substrings
            }
        }
        return temp;
    }

    // Count the number of ways to create a given design.
    fn count(&self, design: &Design) -> usize {
        // Breadth first search until we find a full match.
        let mut count = vec![0usize; design.len()+1];
        count[0] = 1usize;
        for m in 0..design.len() {
            if count[m] > 0 {
                for n in self.next(design, m) {
                    count[n] += count[m];
                }
            }
        }
        return count[design.len()];
    }
}

struct Towels {
    towels: Tree,
    designs: Vec<Design>,
}

impl Towels {
    fn new(input: &str) -> Self {
        let mut tmp = Towels { towels: Tree::new(), designs: Vec::new() };
        for line in input.trim().lines() {
            if line.contains(',') {
                for tok in line.split(',') { tmp.towels.add(tok); }
            } else if !line.is_empty() {
                tmp.designs.push(line.trim().chars().collect());
            }
        }
        return tmp;
    }

    fn part1(&self) -> usize {
        self.designs.iter().filter(|d| self.towels.count(d) > 0).count()
    }

    fn part2(&self) -> usize {
        self.designs.iter().map(|d| self.towels.count(d)).sum()
    }
}

fn part1(input: &str) -> usize {
    Towels::new(input).part1()
}

fn part2(input:&str) -> usize {
    Towels::new(input).part2()
}

const EXAMPLE: &'static str = "\
    r, wr, b, g, bwu, rb, gb, br

    brwrr
    bggr
    gbbr
    rrbgbr
    ubwu
    bwurrg
    brgr
    bbrgwb";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 19).unwrap();

    assert_eq!(part1(EXAMPLE), 6);
    assert_eq!(part2(EXAMPLE), 16);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
