/// Advent of Code 2023, Day 17
/// Copyright 2023 by Alex Utter

use aocfetch;
use core::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Rc(isize, isize);
const DIR_Z: Rc = Rc( 0,  0);
const DIR_N: Rc = Rc(-1,  0);
const DIR_S: Rc = Rc( 1,  0);
const DIR_W: Rc = Rc( 0, -1);
const DIR_E: Rc = Rc( 0,  1);

impl Rc {
    fn add(&self, other: Rc) -> Self {
        Rc(self.0 + other.0, self.1 + other.1)
    }

    fn mul(&self, other: isize) -> Self {
        Rc(self.0 * other, self.1 * other)
    }

    // Return all possible turns.
    fn turn(&self) -> Vec<Self> {
        match *self {
            DIR_N | DIR_S => vec![DIR_E, DIR_W],
            DIR_W | DIR_E => vec![DIR_N, DIR_S],
            _ => vec![DIR_N, DIR_S, DIR_E, DIR_W],
        }
    }
}

struct City {
    blocks: HashMap<Rc, usize>,
    dmin: usize,
    dmax: usize,
    rcmax: Rc,
}

impl City {
    fn new(input: &str, dmin:usize, dmax:usize) -> Self {
        let mut blocks = HashMap::new();
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                let rc = Rc(r as isize, c as isize);
                blocks.insert(rc, ch.to_digit(10).unwrap() as usize);
            }
        }
        let rmax = blocks.keys().map(|rc| rc.0).max().unwrap();
        let cmax = blocks.keys().map(|rc| rc.1).max().unwrap();
        return City { blocks:blocks, rcmax:Rc(rmax,cmax), dmin:dmin, dmax:dmax };
    }

    fn search(&self, src:Rc, dst:Rc) -> usize {
        // Using Dijkstra's with "node" as combined position + direction.
        let mut queue = BinaryHeap::<Reverse<(usize, Rc, Rc)>>::new();
        let mut cost = HashMap::<(Rc,Rc), usize>::new();
        queue.push(Reverse((0, src, DIR_Z)));
        cost.insert((src, DIR_Z), 0);
        while let Some(Reverse((heat, posn, dir))) = queue.pop() {
            for new_dir in dir.turn().into_iter() {
                let mut new_heat = heat;
                for n in 1..=self.dmax {
                    let new_posn = posn.add(new_dir.mul(n as isize));
                    if let Some(h) = self.blocks.get(&new_posn) {
                        new_heat += h;
                        if n < self.dmin {continue;}
                        let prev = cost.entry((new_posn, new_dir)).or_insert(usize::MAX);
                        if new_heat < *prev {
                            queue.push(Reverse((new_heat, new_posn, new_dir)));
                            *prev = new_heat;
                        }
                    } else {break;} // Abort on out-of-bounds
                }
            }
        }
        // Consolidate approach directions.
        return [DIR_N, DIR_S, DIR_E, DIR_W].into_iter()
            .filter_map(|d| cost.get(&(dst,d)).copied())
            .min().unwrap();
    }
}

fn part1(input: &str) -> usize {
    let city = City::new(input, 1, 3);
    return city.search(Rc(0,0), city.rcmax);
}

fn part2(input: &str) -> usize {
    let city = City::new(input, 4, 10);
    return city.search(Rc(0,0), city.rcmax);
}

const EXAMPLE: &'static str = "\
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 17).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 102);
    assert_eq!(part2(EXAMPLE), 94);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
