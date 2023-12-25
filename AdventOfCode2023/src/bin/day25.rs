/// Advent of Code 2023, Day 25
/// Copyright 2023 by Alex Utter

use aocfetch;
use core::cmp::max;
use core::cmp::min;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Edge(usize, usize);

impl Edge {
    fn new(a:usize, b:usize) -> Self {
        Edge(min(a,b), max(a,b))
    }
}

#[derive(Clone, Debug)]
struct Graph {
    // List of all undirected edges (from, to) -> weight.
    // Only store the symmetric pair where from < to.
    edges: HashMap<Edge, usize>,
    // Combined weight of each node by index.
    nodes: HashMap<usize, usize>,
}

impl Graph {
    fn new(input: &str) -> Self {
        // Tokenize the input line by line.
        let lines: Vec<Vec<&str>> = input.trim().lines()
            .map(|s| s.trim().split(&[':', ' ']).collect())
            .collect();
        // First pass identifies unique labels.
        let mut labels: HashMap<String, usize> = HashMap::new();
        for line in lines.iter() {
            for lbl in line.iter() {
                if lbl.len() > 0 && !labels.contains_key(*lbl) {
                    let new_idx = labels.len();
                    labels.insert(lbl.to_string(), new_idx);
                }
            }
        }
        // Second pass contstucts the graph.
        let mut edges = HashMap::new();
        for line in lines.iter() {
            let from = labels[line[0]];
            for lbl in line.iter().skip(2) {
                let to = labels[*lbl];
                edges.insert(Edge::new(to, from), 1);
            }
        }
        let mut nodes = HashMap::new();
        for n in 0..labels.len() {nodes.insert(n,1);}
        return Graph { edges:edges, nodes:nodes };
    }

    // Apply one permutation of Karger's algorithm, deleting randomly-selected
    // edges until there are only two nodes left. Return the final state.
    fn karger(&self, rng: &mut ThreadRng) -> Self {
        let mut graph = self.clone();
        while graph.nodes.len() > 2 {
            let index = rng.gen_range(0..graph.edges.len());
            graph.merge(index);
        }
        return graph;
    }

    // Clone this graph, deleting the direct connection between two nodes.
    fn merge(&mut self, edge: usize) {
        // Lookup from/to indices for the selected edge.
        let Edge(retain, remove) = self.edges.keys().nth(edge).unwrap().clone();
        // Remove the victim node and add its weight to the other.
        let wt_removed = self.nodes.remove(&remove).unwrap();
        let wt_retained = self.nodes.entry(retain).or_insert(0);
        *wt_retained += wt_removed;
        // Reroute all edges to or from the victim node.
        let edge_removed: Vec<Edge> = self.edges.keys()
            .filter(|Edge(f,t)| *f == remove || *t == remove)
            .cloned().collect();
        for edge in edge_removed.iter() {
            let w = self.edges.remove(&edge).unwrap().clone();
            if edge.0 == retain && edge.1 == remove {continue;}
            let next = if edge.0 == remove {
                Edge::new(edge.1, retain)
            } else {
                Edge::new(edge.0, retain)
            };
            *self.edges.entry(next).or_insert(0) += w;
        }
    }

    // Is this a global min-cut according to part-1 rules?
    fn part1(&self) -> Option<usize> {
        assert_eq!(self.edges.len(), 1);
        assert_eq!(self.nodes.len(), 2);
        let cut = self.edges.values().nth(0).cloned().unwrap();
        if cut == 3 {
            let wt: Vec<usize> = self.nodes.values().cloned().collect();
            return Some(wt[0] * wt[1]);
        } else {return None;}
    }
}

fn part1(input: &str) -> usize {
    let graph = Graph::new(input);
    let mut rng = rand::thread_rng();
    loop {
        // Keep trying Karger's algorithm until we get a min-cut.
        if let Some(answer) = graph.karger(&mut rng).part1() {return answer;}
    }
}

const EXAMPLE: &'static str = "\
    jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 25).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 54);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
}
