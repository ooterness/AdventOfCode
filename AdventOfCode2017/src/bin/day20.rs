/// Advent of Code 2017, Day 20
/// Copyright 2023 by Alex Utter

extern crate aocfetch;
use std::cmp::Ordering;
use std::collections::HashSet;

// Parse a string as an integer, ignoring any non-numeric characters.
fn ignore_junk(word: &str) -> i64 {
    let filt: String = word.chars()
        .filter(|&c| ('0' <= c && c <= '9') || (c == '-'))
        .collect();
    return filt.parse().unwrap_or(0);
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Xyz(i64, i64, i64);

impl Xyz {
    fn add(&mut self, val: &Xyz) {
        self.0 += val.0;
        self.1 += val.1;
        self.2 += val.2;
    }

    fn mag(&self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

#[derive(Debug)]
struct Particle {
    pos: Xyz,
    vel: Xyz,
    acc: Xyz,
}

impl Particle {
    fn new(line: &str) -> Option<Particle> {
        let tmp: Vec<i64> = line.split(',').map(ignore_junk).collect();
        if tmp.len() == 9 {
            Some( Particle {
                pos: Xyz(tmp[0], tmp[1], tmp[2]),
                vel: Xyz(tmp[3], tmp[4], tmp[5]),
                acc: Xyz(tmp[6], tmp[7], tmp[8]),
            } )
        } else {None}
    }

    fn cmp(&self, other: &Particle) -> Ordering {
        let x = (self.acc.mag(), self.vel.mag(), self.pos.mag());
        let y = (other.acc.mag(), other.vel.mag(), other.pos.mag());
        return x.cmp(&y);
    }

    // Simulate one timestep.
    fn step(&mut self) {
        self.vel.add(&self.acc);
        self.pos.add(&self.vel);
    }
}

fn part1(input: &str) -> usize {
    input.lines().filter_map(Particle::new)
        .enumerate()            // Find minimum by value...
        .min_by(|(_,x), (_,y)| x.cmp(y))
        .unwrap().0             // ...and return its index
}

fn part2(input: &str) -> usize {
    let mut particles: Vec<Particle> =
        input.lines().filter_map(Particle::new).collect();
    for _ in 0..1000 {
        // Are there any particles that overlap in position?
        let mut unique: HashSet<Xyz> = HashSet::new();
        let mut collide: HashSet<Xyz> = HashSet::new();
        for p in particles.iter() {
            if !unique.insert(p.pos) {collide.insert(p.pos);}
        }
        // Remove particles that collide in this timestep.
        particles.retain(|p| !collide.contains(&p.pos));
        // Increment timestep;
        for p in particles.iter_mut() {p.step();}
    }
    return particles.len()
}

const TEST1: &str = "\
p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";

const TEST2: &str = "\
p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 20).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(TEST1), 0);
    assert_eq!(part2(TEST2), 1);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
