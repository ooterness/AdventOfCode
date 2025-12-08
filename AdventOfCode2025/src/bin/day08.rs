/// Advent of Code 2025, Day 8
/// Copyright 2025 by Alex Utter

use aocfetch;
use std::collections::HashSet;

type Circuit = HashSet<usize>;
type Wire = (u64, usize, usize);

struct Box {
    x: i64,
    y: i64,
    z: i64,
}

impl Box {
    fn new(line: &str) -> Self {
        let n: Vec<i64> = line.trim().split(',')
            .filter_map( |s| s.parse().ok() ).collect();
        assert_eq!(n.len(), 3);
        return Box { x:n[0], y:n[1], z:n[2] };
    }

    fn dist_sq(&self, other:&Box) -> u64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        return (dx*dx + dy*dy + dz*dz) as u64;
    }
}

struct Problem {
    boxes: Vec<Box>,
    wires: Vec<Wire>,
    ckts: Vec<Circuit>,
}

impl Problem {
    fn new(input: &str) -> Self {
        // Parse X/Y/Z coordinates of each junction box.
        let boxes: Vec<Box> = input.trim().lines()
            .map( |line| Box::new(line) ).collect();
        // Initially, each box is its own island.
        let ckts = (0..boxes.len())
            .map( |n| HashSet::from([n]) ).collect();
        // Create sorted list of potential box-to-box connections.
        let mut wires = Vec::new();
        for m in 0..boxes.len()-1 {
            for n in m+1..boxes.len() {
                let len = boxes[m].dist_sq(&boxes[n]);
                wires.push((len, m, n));
            }
        }
        wires.sort();   // Sort ascending by length
        return Problem { boxes:boxes, wires:wires, ckts:ckts };
    }

    // Find the circuit containing a given junction box.
    fn find(&self, bidx:usize) -> Option<usize> {
        for (n, ckt) in self.ckts.iter().enumerate() {
            if ckt.contains(&bidx) { return Some(n); }
        }
        return None;
    }

    // Connect the Nth potential wire, joining two circuits if applicable.
    fn connect(&mut self, widx:usize) {
        // Identify circuits associated with each end of the wire.
        let (_, box_src, box_dst) = self.wires[widx];
        assert!(box_src < box_dst);
        let ckt_src = self.find(box_src).unwrap();
        let ckt_dst = self.find(box_dst).unwrap();
        if ckt_src != ckt_dst {
            // Merge separate circuits. Arbitrarily keep smaller index
            // to avoid off-by-one after deleting the larger index.
            let m = std::cmp::min(ckt_src, ckt_dst);
            let n = std::cmp::max(ckt_src, ckt_dst);
            let mut tmp = self.ckts.remove(n);
            self.ckts[m].extend(tmp.drain());
        }
    }

    fn part1(&mut self, count: usize) -> usize {
        // Make the first N connections.
        for n in 0..count { self.connect(n); }
        // Sort resulting circuits by size.
        let mut tmp: Vec<usize> = self.ckts.iter()
            .map( |ckt| ckt.len() ).collect();
        tmp.sort(); tmp.reverse();  // Sort largest -> smallest
        return tmp[0] * tmp[1] * tmp[2];
    }

    fn part2(&mut self) -> i64 {
        // Keep making connections until it's one big circuit.
        for n in 0..self.wires.len() {
            self.connect(n);
            if self.ckts.len() == 1 {
                let (_, src, dst) = self.wires[n];
                return self.boxes[src].x * self.boxes[dst].x;
            }
        }
        panic!("Can't connect all the boxes :(");
    }
}

fn part1(input: &str, count: usize) -> usize {
    Problem::new(input).part1(count)
}

fn part2(input: &str) -> i64 {
    Problem::new(input).part2()
}

const EXAMPLE: &'static str = "\
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 8).unwrap();

    assert_eq!(part1(EXAMPLE, 10), 40);
    assert_eq!(part2(EXAMPLE), 25272);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&input, 1000));
    println!("Part 2: {}", part2(&input));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
