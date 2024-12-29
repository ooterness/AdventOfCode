/// Advent of Code 2024, Day 21
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

type Rc = (i8, i8);             // Row, column
const KEYPAD_NUM: &'static str = "789\n456\n123\n.0A";
const KEYPAD_DIR: &'static str = ".^A\n<v>";

struct Keypad {
    keys: HashSet<Rc>,          // Valid key locations
    posn: HashMap<char, Rc>,    // Location of each key
}

impl Keypad {
    fn new(input:&str) -> Self {
        let mut keys = HashSet::new();
        let mut posn = HashMap::new();
        for (r,row) in input.lines().enumerate() {
            for (c,ch) in row.chars().enumerate() {
                let rc: Rc = (r as i8, c as i8);
                if ch != '.' {keys.insert(rc); posn.insert(ch, rc);}
            }
        }
        return Keypad { keys:keys, posn:posn };
    }

    // Horizontal-first path from "prev" to "next".
    fn hpath(&self, prev:Rc, next:Rc) -> Option<Vec<char>> {
        let mut cmds = Vec::new();
        let mut posn = prev;
        while posn.1 < next.1 {posn.1 += 1; cmds.push('>'); if !self.keys.contains(&posn) {return None;}}
        while posn.1 > next.1 {posn.1 -= 1; cmds.push('<'); if !self.keys.contains(&posn) {return None;}}
        while posn.0 < next.0 {posn.0 += 1; cmds.push('v'); if !self.keys.contains(&posn) {return None;}}
        while posn.0 > next.0 {posn.0 -= 1; cmds.push('^'); if !self.keys.contains(&posn) {return None;}}
        cmds.push('A');
        return Some(cmds);
    }
    
    // Vertical-first path from "prev" to "next".
    fn vpath(&self, prev:Rc, next:Rc) -> Option<Vec<char>> {
        let mut cmds = Vec::new();
        let mut posn = prev;
        while posn.0 < next.0 {posn.0 += 1; cmds.push('v'); if !self.keys.contains(&posn) {return None;}}
        while posn.0 > next.0 {posn.0 -= 1; cmds.push('^'); if !self.keys.contains(&posn) {return None;}}
        while posn.1 < next.1 {posn.1 += 1; cmds.push('>'); if !self.keys.contains(&posn) {return None;}}
        while posn.1 > next.1 {posn.1 -= 1; cmds.push('<'); if !self.keys.contains(&posn) {return None;}}
        cmds.push('A');
        return Some(cmds);
    }

    // Prepare any valid command sequence to enter the requested keys.
    // (This is sufficient for all by the innermost keypad, since all
    //  sub-sequences start and end at "A" there is no relevant history.)
    fn seq_any(&self, keys: &Vec<char>) -> Vec<char> {
        let mut cmds = Vec::new();
        let mut posn = self.posn[&'A'];     // From home position...
        // Expand the key-sequence, always taking the first valid branch.
        for ch in keys.iter() {
            let next = self.posn[ch];       // Locate the next key...
            if let Some(p) = self.hpath(posn, next) {
                cmds.extend(p.into_iter());
            } else if let Some(p) = self.vpath(posn, next) {
                cmds.extend(p.into_iter());
            } else {
                panic!("No valid path.");
            }
            posn = next;
        }
        return cmds;
    }

    // Prepare all valid command sequences to enter the requested keys.
    // (Brute-force search for the innermost keypad.)
    fn seq_all(&self, keys: &Vec<char>) -> Vec<Vec<char>> {
        let mut cmds: Vec<Vec<char>> = Vec::from([Vec::new()]);
        let mut posn = self.posn[&'A'];     // From home position...
        // For each key, try both the horizontal and vertical options.
        for ch in keys.iter() {
            let next = self.posn[ch];       // Locate the next key...
            let prev_cmds = cmds; cmds = Vec::new();
            for cmd in prev_cmds.into_iter() {
                if let Some(p) = self.hpath(posn, next)
                    { let mut tmp = cmd.clone(); tmp.extend(p.into_iter()); cmds.push(tmp); }
                if let Some(p) = self.vpath(posn, next)
                    { let mut tmp = cmd.clone(); tmp.extend(p.into_iter()); cmds.push(tmp); }
            }
            posn = next;
        }
        return cmds;
    }
}

struct Sequence {
    code: Vec<char>,
    kdir: Keypad,
    knum: Keypad,
}

impl Sequence {
    fn new(input: &str) -> Self {
        Sequence {
            code: input.trim().chars().collect(),
            kdir: Keypad::new(KEYPAD_DIR),
            knum: Keypad::new(KEYPAD_NUM),
        }
    }

    fn solve(&self, layers: usize) -> usize {
        let mut best_len = usize::MAX;
        for init in self.knum.seq_all(&self.code).into_iter() {
            let mut seq = init;
            for _ in 1..layers {seq = self.kdir.seq_any(&seq);}
            if best_len > seq.len() {best_len = seq.len();}
        }
        return best_len;
    }

    fn value(&self) -> usize {
        let mut accum = 0usize;
        for ch in self.code.iter() {
            if let Some(d) = ch.to_digit(10) {
                accum = 10*accum + d as usize;
            }
        }
        return accum;
    }

    fn part1(&self) -> usize {
        self.solve(3) * self.value()
    }
}

fn part1(input: &str) -> usize {
    input.trim().lines()
        .map(|code| Sequence::new(code).part1())
        .sum()
}

fn part2(_input:&str) -> usize {
    0 //???
}

const EXAMPLE: &'static str = "\
    029A
    980A
    179A
    456A
    379A";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 21).unwrap();

    assert_eq!(part1(EXAMPLE), 126384);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
