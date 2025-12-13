/// Advent of Code 2025, Day 10
/// Copyright 2025 by Alex Utter

use aocfetch;
use std::collections::HashMap;
type Mask = u64;            // Bit-mask of buttons or lights.
type Joltage = Vec<i64>;    // Vector of voltages
type Cache = HashMap<Joltage, usize>;

const MASK_NONE: Mask = 0u64;

fn idx2mask(b: usize) -> Mask {
    1u64 << b
}

fn parity(jolt: &Joltage) -> Mask {
    let mut mask = MASK_NONE;
    for (b, x) in jolt.iter().enumerate() {
        if x % 2 > 0 { mask |= idx2mask(b); }
    }
    return mask;
}

fn unparen(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();       // Drop first
    chars.next_back();  // Drop last
    return chars.as_str();
}

struct Buttons {
    goal: Mask,                             // Target for Part 1
    buttons: Vec<Mask>,                     // Button index -> Output mask
    joltage: Joltage,                       // Target for Part 2
    effect: Vec<(usize, Mask, Joltage)>,    // Effect of any button combo
}

impl Buttons {
    fn new(line: &str) -> Self {
        // Tokenize the string.
        let tok: Vec<&str> = line.trim().split(' ').collect();
        let jidx = tok.len() - 1;
        let jstr = unparen(tok[jidx]);
        // Parse each component...
        let mut result = Buttons {
            goal: MASK_NONE,
            buttons: Vec::new(),
            joltage: Vec::new(),
            effect: Vec::new(),
        };
        for (c,ch) in tok[0].chars().enumerate() {
            if c > 0 && ch == '#' { result.goal |= idx2mask(c-1); }
        }
        for bstr in &tok[1..jidx] {
            let bmask = unparen(bstr).split(',')
                .filter_map( |s| s.parse::<usize>().ok() )
                .fold(MASK_NONE, |acc,n| acc | idx2mask(n) );
            result.buttons.push(bmask);
        }
        for jlt in jstr.split(',') {
            if let Ok(j) = jlt.parse::<i64>() {
                result.joltage.push(j);
            }
        }
        // Precalculate effects of all 0/1 button combos.
        let max_mask = idx2mask(result.buttons.len());
        for btn_mask in 0..max_mask {
            let count = btn_mask.count_ones() as usize;
            let mut light = MASK_NONE;
            let mut joltage = result.jzero();
            for b in 0..result.buttons.len() {
                if btn_mask & idx2mask(b) > 0 {
                    light ^= result.buttons[b];
                    for j in 0..result.joltage.len() {
                        if result.buttons[b] & idx2mask(j) > 0 {
                            joltage[j] += 1;
                        }
                    }
                }
            }
            result.effect.push((count, light, joltage));
        }
        return result;
    }

    // Create a Joltage vector of the appropriate size.
    fn jzero(&self) -> Joltage {
        vec![0i64; self.joltage.len()]
    }

    // Recursive search for Part 2 using Russian Peasant method:
    // [https://old.reddit.com/r/adventofcode/comments/1pk87hl/]
    // A given Joltage vector is the sum of even and odd components.
    // The odd parity is equivalent to a light-mask, requiring no
    // more than one press per button. The even residue can be divided
    // in two, always requiring an even number of presses per button.
    // Though we must check *all* options to produce a given parity,
    // this still limits the search space enough to be practical.
    fn search(&self, cache: &mut Cache, target: &Joltage) -> usize {
        // Is this target already cached?
        if let Some(count) = cache.get(target) { return *count; }
        // Find candidates matching the odd-parity light mask.
        let pmask = parity(target);
        let mut min_count = usize::MAX;
        for (c,l,j) in self.effect.iter() {
            if *l != pmask { continue; }
            let mut itmp = target.iter().zip(j.iter());
            if itmp.any( |(a,b)| a < b ) { continue; }
            let residue: Joltage = target.iter().zip(j.iter())
                .map( |(x,y)| (x - y) / 2 ).collect();
            let rcount = self.search(cache, &residue);
            if rcount < usize::MAX {
                let new_count = c + 2 * rcount;
                if new_count < min_count { min_count = new_count; }
            }
        }
        // If applicable, cache the result for future searches.
        cache.insert(target.clone(), min_count);
        return min_count;
    }

    fn part1(&self) -> usize {
        // Find minimal combination of buttons matching the goal.
        // (No button will ever be pressed more than once.)
        self.effect.iter()
            .filter( |x| x.1 == self.goal )
            .map( |(c,_,_)| c )
            .min().unwrap().clone()
    }

    fn part2(&mut self) -> usize {
        // Seed the cache with the goal state (residue = 0).
        let mut cache = Cache::new();
        cache.insert(self.jzero(), 0);
        // Execute recursive search.
        return self.search(&mut cache, &self.joltage);
    }
}

fn part1(input: &str) -> usize {
    input.trim().lines()
        .map( |line| Buttons::new(line).part1() )
        .sum()
}

fn part2(input: &str) -> usize {
    input.trim().lines()
        .map( |line| Buttons::new(line).part2() )
        .sum()
}

const EXAMPLE: &'static str = "\
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 10).unwrap();

    assert_eq!(part1(EXAMPLE), 7);
    assert_eq!(part2(EXAMPLE), 33);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
