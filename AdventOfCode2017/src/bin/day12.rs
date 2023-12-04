/// Advent of Code 2017, Day 12
/// Copyright 2023 by Alex Utter

extern crate aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

struct Graph {
    nodes: HashMap<i64, Vec<i64>>,
}

impl Graph {
    fn new(input: &str) -> Graph {
        let mut nodes = HashMap::new();
        for line in input.trim().lines() {
            let words: Vec<&str> = line.split(" <-> ").collect();
            let left: i64 = words[0].parse().unwrap();
            let right: Vec<i64> = words[1].split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();
            nodes.insert(left, right);
        }
        return Graph { nodes }
    }

    fn reachable(&self, from: i64) -> HashSet<i64> {
        // Set initial state.
        let mut queue: Vec<i64> = vec![from];
        let mut visit: HashSet<i64> = HashSet::new();
        visit.insert(from);
        // Breadth first search...
        while queue.len() > 0 {
            let node = queue.pop().unwrap();
            for next in self.nodes[&node].iter() {
                if !visit.contains(next) {
                    queue.push(*next);
                    visit.insert(*next);
                }
            }
        }
        return visit;
    }
}

fn part1(input: &str) -> usize {
    let graph = Graph::new(input);
    return graph.reachable(0).len();
}

fn part2(input: &str) -> usize {
    let graph = Graph::new(input);
    // Make a set containing all the nodes.
    let mut pending: HashSet<i64> = graph.nodes.keys().cloned().collect();
    // Count the number of disconnected subsets.
    let mut count: usize = 0;
    while pending.len() > 0 {
        count += 1;
        // Pick an arbitrary root and find all connected nodes.
        let root = pending.iter().next().unwrap();
        let conn = graph.reachable(*root);
        // Remove all connected nodes from the pending list.
        for c in conn.iter() {pending.remove(c);}
    }
    return count
}

const TEST: &str = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 12).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(TEST), 6);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
