/// Day 19: https://adventofcode.com/2021/day/19
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::collections::HashMap;
use std::collections::HashSet;

const VERBOSE:bool = true;

// An XYZ coordinate (usually relative coordinates of a Beacon)
#[derive(Clone, Eq, Hash, PartialEq)]
struct Xyz {
    x: i64,
    y: i64,
    z: i64,
}

impl Xyz {
    fn new(line:&str) -> Xyz {
        let xyz = common::split_str_as::<i64>(line, ',');
        assert_eq!(xyz.len(), 3);
        Xyz { x:xyz[0], y:xyz[1], z:xyz[2] }
    }

    // Squared-distance to another point.
    fn dist_sq(&self, arg:&Xyz) -> u64 {
        let dx = (self.x - arg.x).abs() as u64;
        let dy = (self.y - arg.y).abs() as u64;
        let dz = (self.z - arg.z).abs() as u64;
        dx*dx + dy*dy + dz*dz
    }

    // Add or subtract coordinates.
    fn add(&self, arg:&Xyz) -> Xyz {
        Xyz { x:self.x+arg.x, y:self.y+arg.y, z:self.z+arg.z }
    }
    fn sub(&self, arg:&Xyz) -> Xyz {
        Xyz { x:self.x-arg.x, y:self.y-arg.y, z:self.z-arg.z }
    }

    // All 24 possible rotations:
    //  * Pick forward axis (new +x)
    //  * Pick upward axis (new +y)
    //  * Remaining axis by right-hand-rule
    fn rotate(&self, r:usize) -> Xyz {
        match r {
            0  => Xyz {x: self.x, y: self.y, z: self.z},
            1  => Xyz {x: self.x, y:-self.y, z:-self.z},
            2  => Xyz {x: self.x, y: self.z, z:-self.y},
            3  => Xyz {x: self.x, y:-self.z, z: self.y},
            4  => Xyz {x:-self.x, y: self.y, z:-self.z},
            5  => Xyz {x:-self.x, y:-self.y, z: self.z},
            6  => Xyz {x:-self.x, y: self.z, z: self.y},
            7  => Xyz {x:-self.x, y:-self.z, z:-self.y},
            8  => Xyz {x: self.y, y: self.x, z:-self.z},
            9  => Xyz {x: self.y, y:-self.x, z: self.z},
            10 => Xyz {x: self.y, y: self.z, z: self.x},
            11 => Xyz {x: self.y, y:-self.z, z:-self.x},
            12 => Xyz {x:-self.y, y: self.x, z: self.z},
            13 => Xyz {x:-self.y, y:-self.x, z:-self.z},
            14 => Xyz {x:-self.y, y: self.z, z:-self.x},
            15 => Xyz {x:-self.y, y:-self.z, z: self.x},
            16 => Xyz {x: self.z, y: self.x, z: self.y},
            17 => Xyz {x: self.z, y:-self.x, z:-self.y},
            18 => Xyz {x: self.z, y: self.y, z:-self.x},
            19 => Xyz {x: self.z, y:-self.y, z: self.x},
            20 => Xyz {x:-self.z, y: self.x, z:-self.y},
            21 => Xyz {x:-self.z, y:-self.x, z: self.y},
            22 => Xyz {x:-self.z, y: self.y, z: self.x},
            _  => Xyz {x:-self.z, y:-self.y, z:-self.x},
        }
    }
}

// A scanner is a list of relative beacon coordinates.
#[derive(Clone)]
struct Scanner {
    idx:  usize,
    beac: HashSet<Xyz>,
}

impl Scanner {
    fn new<'a>(idx:usize, lines: &mut impl Iterator<Item=&'a String>) -> Option<Scanner> {
        // Check for header, e.g., "--- scanner 3 ---"
        if let Some(line) = lines.next() {
            let ch = line.chars().next().unwrap_or('x');
            if ch != '-' {return None;}
        } else {return None;}
        // Read beacons until we reach a blank line.
        let mut beac = HashSet::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {break;}
            beac.insert(Xyz::new(line));
        }
        Some( Scanner { idx:idx, beac:beac } )
    }

    // Count beacons in this combined scan.
    fn count(&self) -> usize {
        self.beac.len()
    }

    // Distance sets provide a rotation-invariant check for common points.
    fn dmap(&self) -> HashSet<u64> {
        let mut dist = HashSet::new();
        for (na,a) in self.beac.iter().enumerate() {
            for (nb,b) in self.beac.iter().enumerate() {
                if nb > na {dist.insert(a.dist_sq(b));}
            }
        }
        dist
    }

    // Apply operation to all beacons in the set.
    fn add(&self, arg:&Xyz) -> Scanner {
        let beac = self.beac.iter().map(|b| b.add(arg));
        Scanner { idx:self.idx, beac:beac.collect() }
    }
    fn rotate(&self, r:usize) -> Scanner {
        let beac = self.beac.iter().map(|b| b.rotate(r));
        Scanner { idx:self.idx, beac:beac.collect() }
    }

    // Attempt to align and merge a new Scanner with this one.
    fn merge(&mut self, scan: &Scanner) -> bool {
        // Trial and error voting on every possible pairwise alignment.
        type Delta = (usize,Xyz);
        let mut votes: HashMap<Delta,usize> = HashMap::new();
        let mut vbest: Delta = (0, Xyz {x:0, y:0, z:0});
        let mut vmax:  usize = 0;
        for r in 0..24usize {
            for a in self.beac.iter() {
                for b in scan.beac.iter() {
                    let delta = (r, a.sub(&b.rotate(r)));
                    let entry = votes.entry(delta.clone()).or_insert(0usize);
                    *entry += 1; // Increment vote count
                    if *entry > vmax {vmax = *entry; vbest = delta;}
                }
            }
        }
        // Apply the most popular alignment.
        let align = scan.rotate(vbest.0).add(&vbest.1);
        let count = self.beac.intersection(&align.beac).count();
        if count >= 12 {
            if VERBOSE {
                println!("Merging scanner #{} @{} C{}",
                    scan.idx, vbest.0, count);
            }
            for b in align.beac.into_iter() {self.beac.insert(b);}
            true        // Success!
        } else {false}  // No match found
    }
}

// Read input file as a list of Scanner objects.
fn read_file(filename: &str) -> Vec<Scanner> {
    let file = common::read_lines(filename);
    let mut lines = file.iter();
    let mut scans = Vec::new();
    while let Some(scan) = Scanner::new(scans.len(), &mut lines) {
        scans.push(scan);
    }
    scans
}

// Merge all scanners into a single coordinate frame.
fn part1(scans: &Vec<Scanner>) -> Option<Scanner> {
    // Calculate distance maps for each scanner.
    let dmaps: Vec<HashSet<u64>> = scans.iter().map(|s| s.dmap()).collect();
    // Arbitrarily start from scanner #0.
    let mut common_beac = scans[0].clone();
    let mut common_dmap = dmaps[0].clone(); 
    // Attempt to merge each remaining scanner input into the common set.
    let mut consumed = vec![false; scans.len()];
    let mut pending  = scans.len() - 1;
    while pending > 0 {
        // Count successful merge events each iteration.
        let mut mcount = 0usize;
        for n in 1..scans.len() {
            // Skip any that have already been merged.
            if consumed[n] {continue;}
            // Compare distance maps: Are there enough points in common?
            // (Expect at least 12 common points = 12*11/2 common distances.)
            let cmp = common_dmap.intersection(&dmaps[n]).count();
            if cmp < 66 {continue;}
            // Possible match, attempt to merge.
            if common_beac.merge(&scans[n]) {
                // Merge successful, update common state.
                consumed[n] = true;
                mcount += 1;
                pending -= 1;
                for d in dmaps[n].iter() {common_dmap.insert(*d);}
                break;
            }
        }
        // If we couldn't merge anything, abort to avoid infinite loop.
        if mcount == 0 {return None;}
    }
    return Some(common_beac)
}

pub fn solve() {
    let test = read_file("input/test19.txt");
    let data = read_file("input/input19.txt");

    // Part 1 solution aligns the partial scans.
    let test1 = part1(&test).unwrap();
    assert_eq!(test1.count(), 79);
    let data1 = part1(&data).unwrap();
    println!("Part1: {}", data1.count());
}
