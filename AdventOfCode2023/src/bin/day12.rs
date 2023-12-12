/// Advent of Code 2023, Day 12
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;

struct Cache {
    repeat: usize,
    cache: HashMap<(usize,usize),usize>,
}

struct Puzzle {
    width: usize,
    mask0: u64,
    mask1: u64,
    runs: Vec<usize>,
    rsum: usize,
}

impl Cache {
    fn new(repeat: usize) -> Self {
        Cache {repeat:repeat, cache:HashMap::new()}
    }
}

impl Puzzle {
    fn new(line: &str) -> Self {
        let tok: Vec<&str> = line.trim().split(' ').collect();
        // First, parse the ".?#" mask, left = LSB.
        let width = tok[0].trim().chars().count();
        assert!(width < 64);
        let mut mask0 = 0u64;
        let mut mask1 = 0u64;
        for (n,ch) in tok[0].trim().chars().enumerate() {
            if ch == '.' {mask0 |= 1 << n;}
            if ch == '#' {mask1 |= 1 << n;}
        }
        // Parse the comma-delimited list of contiguous runs.
        let runs: Vec<usize> = tok[1].split(',')
            .map(|s| s.parse().unwrap()).collect();
        let rsum = runs.iter().sum();
        return Puzzle {width:width, mask0:mask0, mask1:mask1, runs:runs, rsum:rsum};
    }

    fn mask(&self, bidx:usize) -> u64 {
        1u64 << (bidx % (self.width + 1))
    }

    fn rnext(&self, ridx:usize) -> usize {
        self.runs[ridx % self.runs.len()]
    }

    // Is a run of zeros or ones consistent with the provided masks?
    fn consistent0(&self, bmin:usize, bmax:usize) -> bool {
        (bmin..bmax).all(|b| self.mask1 & self.mask(b) == 0)
    }
    fn consistent1(&self, bmin:usize, bmax:usize) -> bool {
        (bmin..bmax).all(|b| self.mask0 & self.mask(b) == 0)
    }

    // Count the number of solutions from the given initial state:
    //  * cache = Shared cache object.
    //  * bidx = Leftmost starting position for the next run.
    //  * ridx = Number of runs already placed.
    //  * rsum = Total width of runs placed so far. 
    fn search(&self, cache: &mut Cache, bidx:usize, ridx:usize, rsum:usize) -> usize {
        // Has this solution been cached?
        if let Some(x) = cache.cache.get(&(bidx, ridx)) {return *x;}
        // Precalculate various quantities...
        let rmax = cache.repeat * self.runs.len();      // Number of runs
        let rrem = cache.repeat * self.rsum - rsum;     // Remaining run size
        let wmax = cache.repeat * (self.width+1) - 1;   // Output width
        let wrem = rrem + rmax - ridx - 1;              // Including min gaps
        // Quickly eliminate impossible states.
        if bidx + wrem > wmax {return 0;}               // Negative slack :(
        // How many options for placing the next run?
        let slack = wmax - (bidx + wrem);
        // Test each possible position against the mask requirements...
        let mut total = 0usize;
        let rnext = self.rnext(ridx);
        for bskip in 0..=slack {
            let bmid = bidx + bskip;    // Start of run
            let bmax = bmid + rnext;    // End of run
            if !self.consistent0(bidx, bmid) {break;}   // Stop early?
            if !self.consistent1(bmid, bmax) {continue;}
            if ridx + 1 == rmax {                       // Final run?
                if self.consistent0(bmax, wmax) {total += 1;}
            } else if self.consistent0(bmax, bmax+1) {  // Gap OK?
                total += self.search(cache, bmax+1, ridx+1, rsum+rnext);
            }
        }
        cache.cache.insert((bidx, ridx), total);
        return total;
    }

    // Shortcut for starting a full search.
    fn count(&self, repeat:usize) -> usize {
        self.search(&mut Cache::new(repeat), 0, 0, 0)
    }
}

fn part1(input: &str) -> usize {
    input.trim().lines().map(|s| Puzzle::new(s).count(1)).sum()
}

fn part2(input: &str) -> usize {
    input.trim().lines().map(|s| Puzzle::new(s).count(5)).sum()
}

const EXAMPLE: &'static str = "\
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 12).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 21);
    assert_eq!(part2(EXAMPLE), 525152);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
