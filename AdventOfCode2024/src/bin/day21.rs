/// Advent of Code 2024, Day 21
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

type Rc = (i8, i8);                 // Row, column (posn or delta)

struct Keypad {
    keys: HashSet<Rc>,              // Valid key locations
    posn: HashMap<char, Rc>,        // Location of each key
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

    fn contains(&self, key:&Rc) -> bool {
        self.keys.contains(key)
    }
}

struct Solver {
    code:   Vec<char>,
    value:  usize,
    dirpad: Keypad,
    numpad: Keypad,
    cache:  HashMap<(char,char,usize), usize>,
}

impl Solver {
    fn new(input: &str) -> Self {
        // Read the code from the input string.
        let code: Vec<char> = input.trim().chars().collect();
        // Calculate numeric value of that code.
        let mut accum = 0usize;
        for ch in code.iter() {
            if let Some(d) = ch.to_digit(10) {
                accum = 10*accum + d as usize;
            }
        }
        return Solver {
            code:   code,
            value:  accum,
            dirpad: Keypad::new(".^A\n<v>"),
            numpad: Keypad::new("789\n456\n123\n.0A"),
            cache:  HashMap::new(),
        };
    }

    fn keypad(&self, dpad:bool) -> &Keypad {
        if dpad {&self.dirpad} else {&self.numpad}
    }

    fn vmove(&self, delta:i8) -> char {
        if delta < 0 {'^'} else {'v'}
    }

    fn hmove(&self, delta:i8) -> char {
        if delta < 0 {'<'} else {'>'}
    }

    // Calculate cost of horizontal-first path from "prev" to "next".
    fn cost_hfirst(&mut self, dpad:bool, prev:Rc, next:Rc, lvl:usize) -> usize {
        // Does this move have a horizontal component?
        if prev.1 == next.1 {return usize::MAX;}
        // Does this move go out of bounds?
        let corner = (prev.0, next.1);
        if !self.keypad(dpad).contains(&corner) {return usize::MAX;}
        // Otherwise, press required key(s) and return to home position.
        let vmove = self.vmove(next.0 - prev.0);
        let hmove = self.hmove(next.1 - prev.1);
        let count = (next.0 - prev.0).abs() as usize
                  + (next.1 - prev.1).abs() as usize;
        if prev.0 == next.0 {   // Two-part move (H, A)
            return self.cost(true, 'A',   hmove, lvl-1)
                 + self.cost(true, hmove, 'A',   lvl-1) + count;
        } else {                // Three-part move (H, V, A)
            return self.cost(true, 'A',   hmove, lvl-1)
                 + self.cost(true, hmove, vmove, lvl-1)
                 + self.cost(true, vmove, 'A',   lvl-1) + count;
        }
    }

    // Calculate cost of vertical-first path from "prev" to "next".
    fn cost_vfirst(&mut self, dpad:bool, prev:Rc, next:Rc, lvl:usize) -> usize {
        // Does this move have a vertical component?
        if prev.0 == next.0 {return usize::MAX;}
        // Does this move go out of bounds?
        let corner = (next.0, prev.1);
        if !self.keypad(dpad).contains(&corner) {return usize::MAX;}
        // Otherwise, press required key(s) and return to home position.
        let vmove = self.vmove(next.0 - prev.0);
        let hmove = self.hmove(next.1 - prev.1);
        let count = (next.0 - prev.0).abs() as usize
                  + (next.1 - prev.1).abs() as usize;
        if prev.1 == next.1 {   // Two-part move (V, A)
            return self.cost(true, 'A',   vmove, lvl-1)
                 + self.cost(true, vmove, 'A',   lvl-1) + count;
        } else {                // Three-part move (V, H, A)
            return self.cost(true, 'A',   vmove, lvl-1)
                 + self.cost(true, vmove, hmove, lvl-1)
                 + self.cost(true, hmove, 'A',   lvl-1) + count;
        }
    }

    // Return the minimum cost of a given move, with memoization.
    // (Note: Includes return of all preceding layers to the "A" key.)
    fn cost(&mut self, dpad:bool, prev:char, next:char, lvl:usize) -> usize {
        let key = (prev, next, lvl);
        if lvl == 0 {
            return 0;
        } else if let Some(&cost) = self.cache.get(&key) {
            return cost;
        } else {
            let prev_rc = self.keypad(dpad).posn[&prev];
            let next_rc = self.keypad(dpad).posn[&next];
            let cost = std::cmp::min(
                self.cost_hfirst(dpad, prev_rc, next_rc, lvl),
                self.cost_vfirst(dpad, prev_rc, next_rc, lvl));
            assert!(cost < usize::MAX);
            if dpad {self.cache.insert(key, cost);}
            return cost;
        }
    }
    fn solve(&mut self, layers: usize) -> usize {
        let mut posn = 'A';         // Numpad starts at 'A'
        let mut accum = 0usize;     // Total cost so far
        for &ch in self.code.clone().iter() {
            // Move to each numpad key, then press 'A'.
            accum += self.cost(false, posn, ch, layers) + 1;
            posn = ch;
        }
        return accum * self.value;
    }
}

fn part1(input: &str) -> usize {
    input.trim().lines()
        .map(|code| Solver::new(code).solve(3))
        .sum()
}

fn part2(input:&str) -> usize {
    input.trim().lines()
        .map(|code| Solver::new(code).solve(26))
        .sum()
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
