/// Advent of Code 2024, Day 25
/// Copyright 2024 by Alex Utter

use aocfetch;

struct LockOrKey {
    is_key: bool,
    height: [usize;5],
}

impl LockOrKey {
    fn new<'a>(input: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
        // Read the 7x5 grid, counting filled slots in each column.
        let mut is_key = true;
        let mut count_col = [0usize;5];
        let mut first_row = true;
        while let Some(next) = input.next() {
            let row: Vec<char> = next.trim().chars().collect();
            if row.is_empty() {break;}
            if row.len() != 5 {return None;}
            for (c,&ch) in row.iter().enumerate() {
                if ch == '#' && first_row {is_key = false;}
                if ch == '#' {count_col[c] += 1;}
            }
            first_row = false;
        }
        if first_row {return None;}
        Some(LockOrKey { is_key: is_key, height: count_col } )
    }

    fn fits(&self, other: &LockOrKey) -> bool {
        (0..5).all(|n| self.height[n] + other.height[n] <= 7)
    }
}

struct Schematics {
    locks: Vec<LockOrKey>,
    keys:  Vec<LockOrKey>,
}

impl Schematics {
    fn new(input: &str) -> Self {
        let mut list = Schematics { locks: Vec::new(), keys: Vec::new() };
        let mut iter = input.trim().lines();
        while let Some(sch) = LockOrKey::new(&mut iter) {
            if sch.is_key {list.keys.push(sch);}
                     else {list.locks.push(sch);}
        }
        return list;
    }

    fn count_pairs(&self) -> usize {
        let mut count = 0usize;
        for lock in self.locks.iter() {
            count += self.keys.iter()
                .filter(|k| lock.fits(k)).count()
        }
        return count;
    }
}

fn part1(input: &str) -> usize {
    Schematics::new(input).count_pairs()
}

const EXAMPLE: &'static str = "\
    #####
    .####
    .####
    .####
    .#.#.
    .#...
    .....

    #####
    ##.##
    .#.##
    ...##
    ...#.
    ...#.
    .....

    .....
    #....
    #....
    #...#
    #.#.#
    #.###
    #####

    .....
    .....
    #.#..
    ###..
    ###.#
    ###.#
    #####

    .....
    .....
    .....
    #....
    #.#..
    #.#.#
    #####";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 25).unwrap();

    assert_eq!(part1(EXAMPLE), 3);

    println!("Part 1: {}", part1(&input));
}
