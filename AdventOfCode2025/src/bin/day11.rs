/// Advent of Code 2025, Day 11
/// Copyright 2025 by Alex Utter

use aocfetch;
use std::collections::HashMap;

type Cache = HashMap<usize,usize>;

const DEBUG:bool = false;

struct Node {
    label: String,
    fwd: Vec<usize>,    // Outputs to next node
    rev: Vec<usize>,    // Inputs from previous node
}

impl Node {
    fn new(label: &str) -> Self {
        Node {
            label: label.to_string(),
            fwd: Vec::new(),
            rev: Vec::new(),
        }
    }
}

struct Graph {
    index: HashMap<String, usize>,  // Map label to index
    nodes: Vec<Node>,               // List nodes in graph
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut tmp = Graph {
            index: HashMap::new(),
            nodes: Vec::new(),
        };
        for line in input.trim().lines() {
            let labels: Vec<&str> = line.trim().split(&[' ', ':']).collect();
            let root = tmp.find_or_create(&labels[0]);
            for label in &labels[2..] {
                let leaf = tmp.find_or_create(label);
                tmp.nodes[root].fwd.push(leaf);
                tmp.nodes[leaf].rev.push(root);
            }
            if DEBUG { println!("{}:{} -> {:?}", root, labels[0], tmp.nodes[root].fwd); }
        }
        return tmp;
    }

    fn find_or_create(&mut self, label:&str) -> usize {
        if let Some(idx) = self.index.get(label) {
            return *idx;
        } else {
            let new_idx = self.nodes.len();
            self.index.insert(label.to_string(), new_idx);
            self.nodes.push(Node::new(label));
            return new_idx;
        }
    }

    fn dfs(&self, node:usize, cache:&mut Cache) -> usize {
        // Depth first search with memoization.
        if let Some(ct) = cache.get(&node) { return *ct; }
        let total: usize = self.nodes[node].fwd.iter()
            .map( |next| self.dfs(*next, cache) )
            .sum();
        if DEBUG { println!("{} -> {}", self.nodes[node].label, total); }
        cache.insert(node, total);
        return total;
    }

    fn count_paths(&self, src_str:&str, dst_str:&str) -> usize {
        let src = self.index[src_str];
        let dst = self.index[dst_str];
        let mut cache = Cache::new();
        cache.insert(dst, 1);   // Base case
        return self.dfs(src, &mut cache);
    }
}

fn part1(input: &str) -> usize {
    Graph::new(input).count_paths("you", "out")
}

fn part2(input: &str) -> usize {
    let graph = Graph::new(input);
    return graph.count_paths("svr", "fft")
         * graph.count_paths("fft", "dac")
         * graph.count_paths("dac", "out")
         + graph.count_paths("svr", "dac")
         * graph.count_paths("dac", "fft")
         * graph.count_paths("fft", "out");
}

const EXAMPLE1: &'static str = "\
    aaa: you hhh
    you: bbb ccc
    bbb: ddd eee
    ccc: ddd eee fff
    ddd: ggg
    eee: out
    fff: out
    ggg: out
    hhh: ccc fff iii
    iii: out";
const EXAMPLE2: &'static str = "\
    svr: aaa bbb
    aaa: fft
    fft: ccc
    bbb: tty
    tty: ccc
    ccc: ddd eee
    ddd: hub
    hub: fff
    eee: dac
    dac: fff
    fff: ggg hhh
    ggg: out
    hhh: out";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 11).unwrap();

    assert_eq!(part1(EXAMPLE1), 5);
    assert_eq!(part2(EXAMPLE2), 2);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
