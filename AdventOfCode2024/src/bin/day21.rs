/// Advent of Code 2024, Day 21
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;

const VERBOSE: bool = true;

type Rc = (i8, i8);             // Row, column
const KEYPAD_NUM: &'static str = "789\n456\n123\n.0A";
const KEYPAD_DIR: &'static str = ".^A\n<v>";

struct Keypad {
    posn: HashMap<char, Rc>,    // Location of each key
}

impl Keypad {
    fn new(input: &str) -> Self {
        let mut posn = HashMap::new();
        for (r,row) in input.lines().enumerate() {
            for (c,ch) in row.chars().enumerate() {
                let rc: Rc = (r as i8, c as i8);
                posn.insert(ch, rc);
            }
        }
        return Keypad { posn:posn };
    }

    // Calculate movements required to enter a sequence of keys,
    // with option to prefer row-first or column-first movement.
    fn seq(&self, keys: &Vec<char>) -> Vec<char> {
        let mut cmds = Vec::new();
        let mut posn = self.posn[&'A'];     // From home position...
        let gap = self.posn[&'.'];          // Also note gap position.
        // Expand the key-sequence.
        for ch in keys.iter() {
            let next = self.posn[ch];       // Locate the next key...
            let r1st = posn.0 == gap.0;     // At risk of hitting gap?
            if r1st {                       // Row-first or column-first?
                while posn.0 < next.0 {posn.0 += 1; cmds.push('v'); if posn == gap {panic!("Gap");}}
                while posn.0 > next.0 {posn.0 -= 1; cmds.push('^'); if posn == gap {panic!("Gap");}}
                while posn.1 < next.1 {posn.1 += 1; cmds.push('>'); if posn == gap {panic!("Gap");}}
                while posn.1 > next.1 {posn.1 -= 1; cmds.push('<'); if posn == gap {panic!("Gap");}}
            } else {
                while posn.1 < next.1 {posn.1 += 1; cmds.push('>'); if posn == gap {panic!("Gap");}}
                while posn.1 > next.1 {posn.1 -= 1; cmds.push('<'); if posn == gap {panic!("Gap");}}
                while posn.0 < next.0 {posn.0 += 1; cmds.push('v'); if posn == gap {panic!("Gap");}}
                while posn.0 > next.0 {posn.0 -= 1; cmds.push('^'); if posn == gap {panic!("Gap");}}
            }
            cmds.push('A');                 // Push requested key
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

    fn solve(&self, layers: usize) -> Vec<char> {
        let mut seq = self.knum.seq(&self.code);
        for _ in 1..layers {seq = self.kdir.seq(&seq);}
        return seq;
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
        if VERBOSE { println!("{:?} -> {} * {} -> {:?}",
            self.code, self.solve(3).len(), self.value(), self.solve(3)); }
        self.solve(3).len() * self.value()
    }
}

fn part1(input: &str) -> usize {
    // 182810 = too high???
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
