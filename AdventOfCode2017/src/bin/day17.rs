/// Advent of Code 2017, Day 17
/// Copyright 2023 by Alex Utter

use aocfetch;

// One node in a singly-linked list.
struct Node {
    value: u64,
    next: usize,
}

// A circular linked-list with a cursor.
struct List {
    nodes: Vec<Node>,
    cursor: usize,
}

impl List {
    // Create a new circular list with the value zero.
    fn new() -> List {
        let node0 = Node { value: 0, next: 0 };
        return List { nodes: vec![node0], cursor: 0 };
    }

    // Create and insert a node just after the cursor.
    fn insert(&mut self, value: u64) {
        let idx  = self.nodes.len();
        let next = self.nodes[self.cursor].next;
        let node = Node { value: value, next: next };
        self.nodes[self.cursor].next = idx;
        self.nodes.push(node);
        self.cursor = idx;
    }

    // Skip ahead N hops along the circular linked list.
    fn skip(&mut self, count: usize) {
        let ct_mod = count % self.nodes.len();
        for _ in 0..ct_mod { self.cursor = self.nodes[self.cursor].next; }
    }
}

fn part1(step: usize, size: u64) -> u64 {
    // Direct simulation of the insertion process.
    let mut list = List::new();
    for n in 0..size {
        list.skip(step);
        list.insert(n+1);
    }
    let last_node = &list.nodes[list.cursor];
    return list.nodes[last_node.next].value;
}

fn part2(step: usize, size: u64) -> u64 {
    // We only care about the value at index 1, so we can
    // simplify the simulation considerably.
    let mut index = 0usize;
    let mut value = 0u64;
    for n in 0..size {
        index = (index + step) % (n as usize + 1);
        if index == 0 {value = n + 1;}
        index = (index + 1) % (n as usize + 2);
    }
    return value;
}

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 17).unwrap();
    let step: usize = input.trim().parse().unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(3, 9), 5);
    assert_eq!(part1(3, 2017), 638);
    assert_eq!(part2(3, 1), 1);
    assert_eq!(part2(3, 2), 2);
    assert_eq!(part2(3, 3), 2);
    assert_eq!(part2(3, 4), 2);
    assert_eq!(part2(3, 5), 5);
    assert_eq!(part2(3, 9), 9);

    // Solve for real input.
    println!("Part 1: {}", part1(step, 2017));
    println!("Part 2: {}", part2(step, 50000000));
}
