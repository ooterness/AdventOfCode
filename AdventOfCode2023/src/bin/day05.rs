/// Advent of Code 2023, Day 5
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;

const DEBUG: bool = false;

struct Range {
    dst: usize,
    src: usize,
    len: usize,
}

struct Map {
    src: String,
    dst: String,
    ranges: Vec<Range>,
}

struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<String, Map>,
}

impl Range {
    // Create a range object from a line of text (dst, src, len).
    fn new(line: &str) -> Self {
        let tok: Vec<usize> = line.trim().split(' ')
            .map(|x| x.parse().unwrap()).collect();
        Range { dst: tok[0], src: tok[1], len: tok[2] }
    }

    // Forward conversion, if value falls within this range.
    fn fwd_one(&self, val: usize) -> Option<usize> {
        if self.src <= val && val < self.src + self.len {
            Some(self.dst + val - self.src)
        } else {
            None
        }
    }
}

impl Map {
    // Read labels to create an empty map (e.g., "seed-to-soil map:").
    fn new(line: &str) -> Self {
        let tok: Vec<&str> = line.trim().split(&['-', ' ']).collect();
        Map { src: tok[0].to_string(), dst: tok[2].to_string(), ranges: Vec::new() }
    }

    // Normalize this mapping by sorting the segments.
    fn normalize(&mut self) {
        self.ranges.sort_by(|a,b| a.src.partial_cmp(&b.src).unwrap());
    }

    // Apply this numeric map to a single value.
    fn fwd_one(&self, src: usize) -> usize {
        // Brute-force search across each range...
        for r in self.ranges.iter() {
            if let Some(dst) = r.fwd_one(src) {return dst;}
        }
        return src; // No match = passthrough
    }

    // Apply this numeric map to a range of values.
    fn fwd_range(&self, src: (usize,usize)) -> Vec<(usize,usize)> {
        let mut result = Vec::new();
        let mut src_min = src.0;
        let src_max = src.0 + src.1;
        for range in self.ranges.iter() {
            // Skip this rule if input range is entirely to the right.
            if src_min >= range.src + range.len {continue;}
            // Process any portions to the left of the current rule.
            if src_max <= range.src {
                // Passthrough for the entire remaining input.
                if DEBUG {println!("A {}-{}", src_min, src_max);}
                result.push((src_min, src_max - src_min));
                src_min = src_max; break;
            } else if src_min < range.src {
                // Passthrough for the leading portion only.
                if DEBUG {println!("B {}-{}", src_min, src_max);}
                result.push((src_min, range.src - src_min));
                src_min = range.src;
            }
            // Process any portions that fall within the current rule.
            let dst_min = range.dst + src_min - range.src;
            if src_max <= range.src + range.len {
                // Map the remaining portion and stop.
                if DEBUG {println!("C {}-{}", src_min, src_max);}
                result.push((dst_min, src_max - src_min));
                src_min = src_max; break;
            } else {
                // Map up to the end of the current rule and continue.
                if DEBUG {println!("D {}-{}", src_min, src_max);}
                result.push((dst_min, range.src + range.len - src_min));
                src_min = range.src + range.len;
            }
        }
        // Passthrough for any trailing portion.
        if src_min < src_max {result.push((src_min, src_max - src_min));}
        return result;
    }
}

impl Almanac {
    // Read the entire almanac.
    fn new(input: &str) -> Self {
        let mut almanac = Almanac { seeds: Vec::new(), maps: HashMap::new() };
        let mut section = None;
        for line in input.trim().lines() {
            if line.is_empty() {continue;}
            if line.starts_with("seeds:") {
                // Read the list of seeds, e.g., "seeds: 79 14 55 13"
                almanac.seeds.extend(
                    line.trim().split(' ').skip(1)
                    .map(|x| x.parse::<usize>().unwrap()));
            } else if line.contains("-to-") {
                // Start of a new map section.
                let tmp = Map::new(line);
                section = Some(almanac.maps.entry(tmp.src.clone()).or_insert(tmp));
            } else if let Some(sec) = &mut section {
                // Continue the current map section.
                sec.ranges.push(Range::new(line));
            }
        }
        for sec in almanac.maps.values_mut() {sec.normalize();}
        return almanac;
    }

    // Apply the designated mapping to in-place individual values.
    fn fwd_one(&self, typ: &mut String, idx: &mut Vec<usize>) {
        let map = &self.maps[typ];
        for x in idx.iter_mut() {*x = map.fwd_one(*x);}
        *typ = map.dst.clone();
    }

    // Apply the designated mapping to a list of ranges.
    fn fwd_range(&self, typ: &mut String, idx: &mut Vec<(usize,usize)>) {
        if DEBUG {println!("{}: {:?}", typ, idx);}
        let map = &self.maps[typ];
        *idx = idx.into_iter().map(|x| map.fwd_range(*x)).flatten().collect();
        *typ = map.dst.clone();
    }
}

fn part1(input: &str) -> usize {
    // Set initial conditions.
    let almanac = Almanac::new(input);
    let mut idx = almanac.seeds.clone();
    let mut typ = "seed".to_string();
    // Search until we find locations.
    while typ != "location" {almanac.fwd_one(&mut typ, &mut idx);}
    return idx.into_iter().min().unwrap();
}

// Solve using Part-2 rules: Win additional cards
fn part2(input: &str) -> usize {
    // Set initial conditions.
    let almanac = Almanac::new(input);
    let seed0 = almanac.seeds.iter().cloned().step_by(2);
    let seed1 = almanac.seeds.iter().cloned().skip(1).step_by(2);
    let mut idx = seed0.zip(seed1).collect();
    let mut typ = "seed".to_string();
    // Search until we find locations.
    while typ != "location" {almanac.fwd_range(&mut typ, &mut idx);}
    return idx.into_iter().map(|x| x.0).min().unwrap();
}

const EXAMPLE: &'static str = "\
    seeds: 79 14 55 13
    seed-to-soil map:
    50 98 2
    52 50 48
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    water-to-light map:
    88 18 7
    18 25 70
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    temperature-to-humidity map:
    0 69 1
    1 0 69
    humidity-to-location map:
    60 56 37
    56 93 4";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 5).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 35);
    assert_eq!(part2(EXAMPLE), 46);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
