/// Advent of Code 2024, Day 23
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

struct Network {
    index: HashMap<String, usize>,  // Label to index
    label: Vec<String>,             // Index to label
    links: Vec<HashSet<usize>>,     // Index to connection list
}

impl Network {
    fn new(input: &str) -> Self {
        let mut net = Network {
            index: HashMap::new(),
            label: Vec::new(),
            links: Vec::new(),
        };
        for line in input.trim().lines() {
            let tok: Vec<&str> = line.trim().split('-').collect();
            let a = net.get_idx(tok[0]);
            let b = net.get_idx(tok[1]);
            net.links[a].insert(b);
            net.links[b].insert(a);
        }
        return net;
    }

    fn get_idx(&mut self, lbl: &str) -> usize {
        if let Some(idx) = self.index.get(lbl) {
            return *idx;    // Existing index
        } else {
            let tmp = self.links.len();
            self.index.insert(String::from(lbl), tmp);
            self.label.push(String::from(lbl));
            self.links.push(HashSet::new());
            return tmp;     // New index
        }
    }

    fn is_triplet(&self, a:usize, b:usize, c:usize) -> bool {
        self.links[a].contains(&b) &&
        self.links[a].contains(&c) &&
        self.links[b].contains(&c)
    }

    fn is_label_t(&self, a:usize, b:usize, c:usize) -> bool {
        self.label[a].starts_with('t') ||
        self.label[b].starts_with('t') ||
        self.label[c].starts_with('t')
    }

    fn get_triplets(&self) -> Vec<(usize,usize,usize)> {
        // Find all densely-connected triplets.
        let mut result = Vec::new();
        for a in 0..self.links.len()-2 {
            for b in a+1..self.links.len()-1 {
                if !self.links[a].contains(&b) {continue;}
                for c in b+1..self.links.len() {
                    if self.is_triplet(a, b, c) {result.push((a,b,c));}
                }
            }
        }
        return result;
    }

    fn part1(&self) -> usize {
        // Find all densely-connected triplets where at least one
        // of the computer names starts with 't'.
        self.get_triplets().iter()
            .filter(|(a,b,c)| self.is_label_t(*a,*b,*c)).count()
    }

    // Given a seed, find the largest densely-connected cluster
    // containing that seed and larger indices.
    fn cluster_from(&self, index: usize) -> Option<HashSet<usize>> {
        let mut clumps: Vec<HashSet<usize>> = Vec::new();
        for &m in self.links[index].iter() {
            if m < index {continue;}
            // Are we densely connected with a prior cluster? Try expanding.
            let cmax = clumps.len();
            for c in 0..cmax {
                if clumps[c].iter().all(|n| self.links[m].contains(n)) {
                    let mut next = clumps[c].clone();
                    next.insert(m);
                    clumps.push(next);
                }
            }
            // Always add the new pair...
            clumps.push(HashSet::from([index,m]));
        }
        // Find the longest cluster, if any.
        clumps.into_iter().max_by(|x,y| x.len().cmp(&y.len()))
    }

    fn largest_cluster(&self) -> HashSet<usize> {
        let mut cluster = HashSet::new();
        for n in 0..self.links.len() {
            if let Some(tmp) = self.cluster_from(n) {
                if cluster.len() < tmp.len() {cluster = tmp;}
            }
        }
        return cluster;
    }

    fn part2(&self) -> String {
        let mut labels: Vec<&str> = self.largest_cluster().into_iter()
            .map(|n| self.label[n].as_str()).collect();
        labels.sort();
        return labels.join(",");
    }
}

fn part1(input: &str) -> usize {
    Network::new(input).part1()
}

fn part2(input:&str) -> String {
    Network::new(input).part2()
}

const EXAMPLE: &'static str = "\
    kh-tc\n qp-kh\n de-cg\n ka-co\n yn-aq\n qp-ub\n cg-tb\n vc-aq
    tb-ka\n wh-tc\n yn-cg\n kh-ub\n ta-co\n de-co\n tc-td\n tb-wq
    wh-td\n ta-ka\n td-qp\n aq-cg\n wq-ub\n ub-vc\n de-ta\n wq-aq
    wq-vc\n wh-yn\n ka-de\n kh-ta\n co-tc\n wh-qp\n tb-vc\n td-yn";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 23).unwrap();

    assert_eq!(part1(EXAMPLE), 7);
    assert_eq!(part2(EXAMPLE), "co,de,ka,ta");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
