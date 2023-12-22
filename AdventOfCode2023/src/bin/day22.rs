/// Advent of Code 2023, Day 22
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const DEBUG: bool = false;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Xyz(i32, i32, i32);

impl Xyz {
    fn new(input: &str) -> Self {
        let tok: Vec<i32> = input.split(',')
            .map(|s| s.parse().unwrap()).collect();
        return Xyz(tok[0], tok[1], tok[2]);
    }

    fn add(&self, other: &Xyz) -> Self {
        Xyz(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    fn sub(&self, other: &Xyz) -> Self {
        Xyz(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Brick(Xyz, Xyz);

impl Brick {
    fn new(input: &str) -> Self {
        let tok: Vec<&str> = input.trim().split('~').collect();
        return Brick(Xyz::new(tok[0]), Xyz::new(tok[1]));
    }

    fn sub(&self, other: &Xyz) -> Self {
        Brick(self.0.sub(other), self.1.sub(other))
    }

    fn fill(&self) -> Vec<Xyz> {
        let mut result = Vec::new();
        for x in self.0.0..=self.1.0 {
            for y in self.0.1..=self.1.1 {
                for z in self.0.2..=self.1.2 {
                    result.push(Xyz(x,y,z));
                }
            }
        }
        return result;
    }

    fn ceil(&self) -> Vec<Xyz> {
        let mut result = Vec::new();
        for x in self.0.0..=self.1.0 {
            for y in self.0.1..=self.1.1 {
                result.push(Xyz(x,y,self.1.2));
            }
        }
        return result;
    }

    fn floor(&self) -> Vec<Xyz> {
        let mut result = Vec::new();
        for x in self.0.0..=self.1.0 {
            for y in self.0.1..=self.1.1 {
                result.push(Xyz(x,y,self.0.2));
            }
        }
        return result;
    }

    // Drop this brick until we contact an existing object.
    fn drop(&self, pile: &Pile) -> Self {
        let mut best: Brick = self.clone();
        for dz in 1..self.0.2 {
            let test = self.sub(&Xyz(0,0,dz));
            let overlap = test.floor().iter()
                .any(|xyz| pile.cubes.contains_key(xyz));
            if overlap {break;} else {best = test;}
        }
        return best;
    }

    // Which other bricks are directly supported by this brick?
    fn above(&self, pile: &Pile) -> HashSet<usize> {
        let mut result = HashSet::new();
        for xyz in self.ceil().iter() {
            if let Some(idx) = pile.cubes.get(&xyz.add(&Xyz(0,0,1))) {
                result.insert(*idx);
            }
        }
        return result;
    }

    // Which other bricks are directly supporting this brick?
    fn below(&self, pile: &Pile) -> HashSet<usize> {
        let mut result = HashSet::new();
        for xyz in self.floor().iter() {
            if let Some(idx) = pile.cubes.get(&xyz.sub(&Xyz(0,0,1))) {
                result.insert(*idx);
            }
        }
        return result;
    }
}

struct Pile {
    bricks: Vec<Brick>,
    cubes: HashMap<Xyz, usize>,
    above: Vec<HashSet<usize>>,
    below: Vec<HashSet<usize>>,
}

impl Pile {
    fn new(input: &str) -> Self {
        // Parse input and sort by increasing height.
        let mut bricks: Vec<Brick> = input.trim().lines().map(Brick::new).collect();
        bricks.sort_unstable_by_key(|brick| brick.0.2);
        // Drop bricks one-by-one into the pile.
        let mut pile = Pile {
            bricks: Vec::new(),
            cubes:  HashMap::new(),
            above:  Vec::new(),
            below:  Vec::new(),
        };
        for brick in bricks.into_iter() {
            pile.add(brick.drop(&pile));
        }
        if DEBUG {
            pile.print(false);
            pile.print(true);
        }
        pile.above = pile.bricks.iter().map(|b| b.above(&pile)).collect();
        pile.below = pile.bricks.iter().map(|b| b.below(&pile)).collect();
        return pile;
    }

    // Add a brick to the pile at its current position.
    fn add(&mut self, brick: Brick) {
        let index = self.bricks.len();
        for xyz in brick.fill().into_iter() {
            self.cubes.insert(xyz, index);
        }
        self.bricks.push(brick);
    }

    // How many bricks are safe to remove? (i.e., not a sole support)
    fn safe(&self) -> usize {
        let mut safe = vec![1usize;self.bricks.len()];
        for brick in 0..self.bricks.len() {
            let sup = &self.below[brick];
            if sup.len() == 1 {
                safe[*sup.iter().nth(0).unwrap()] = 0;
            }
        }
        return safe.iter().sum();
    }

    // How many bricks would fall if a given brick were removed?
    fn chain(&self, start: usize) -> usize {
        // Identify the support graph and its inverse.
        let above: Vec<HashSet<usize>> = self.bricks.iter()
            .map(|b| b.above(self)).collect();
        let below: Vec<HashSet<usize>> = self.bricks.iter()
            .map(|b| b.below(self)).collect();
        // Remove each no-longer-supported brick, recursively checking
        // for additional bricks that may no longer be supported.
        let mut fallen: HashSet<usize> = HashSet::from([start]);
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.extend(above[start].iter());
        while let Some(idx) = queue.pop_front() {
            if below[idx].iter().all(|b| fallen.contains(b)) {
                fallen.insert(idx);
                queue.extend(above[idx].iter());
            }
        }
        return fallen.len() - 1;
    }

    // Print the pile in the same style as the example.
    fn print(&self, yz: bool) {
        let xmax = self.cubes.keys().map(|xyz| xyz.0).max().unwrap_or(0);
        let ymax = self.cubes.keys().map(|xyz| xyz.1).max().unwrap_or(0);
        let zmax = self.cubes.keys().map(|xyz| xyz.2).max().unwrap_or(0);
        println!("{}   Z", if yz {"Y"} else {"X"});
        for z in (0..=zmax).rev() {
            if yz {
                for y in 0..=ymax {
                    let slice = (0..=xmax).filter_map(|x| self.cubes.get(&Xyz(x,y,z)));
                    print!("{}", Pile::slice2char(slice.collect()));
                }
                println!(" {}", z);
            } else {
                for x in 0..=xmax {
                    let slice = (0..=ymax).filter_map(|y| self.cubes.get(&Xyz(x,y,z)));
                    print!("{}", Pile::slice2char(slice.collect()));
                }
                println!(" {}", z);
            }
        }
    }

    // Helper for the print() method.
    fn slice2char(blocks: HashSet<&usize>) -> char {
        match blocks.len() {
            0 => '.',
            1 => {let tmp = *blocks.into_iter().nth(0).unwrap() as u32;
                  char::from_u32('A' as u32 + tmp).unwrap()},
            _ => '?',
        }
    }
}

fn part1(input: &str) -> usize {
    Pile::new(input).safe()
}

fn part2(input: &str) -> usize {
    let pile = Pile::new(input);
    (0..pile.bricks.len()).map(|n| pile.chain(n)).sum()
}

const EXAMPLE: &'static str = "\
    1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 22).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 5);
    assert_eq!(part2(EXAMPLE), 7);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
