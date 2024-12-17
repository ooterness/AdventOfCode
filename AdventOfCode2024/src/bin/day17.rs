/// Advent of Code 2024, Day 17
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::VecDeque;

const VERBOSE: bool = false;

fn format(input: &Vec<u8>) -> String {
    input.iter().map(|x| x.to_string())
        .collect::<Vec<_>>().join(",")
}

fn parse<T: std::str::FromStr>(line: &str) -> Vec<T> {
    line.trim().split([' ', ','])
        .filter_map(|s| s.parse().ok()).collect()
}

#[derive(Clone)]
struct State {
    rega: i64,      // Register A, B, C
    regb: i64,
    regc: i64,
    iptr: usize,    // Instruction pointer
}

impl State {
    fn cbo(&self, arg: u8) -> i64 {
        match arg {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.rega,
            5 => self.regb,
            6 => self.regc,
            _ => panic!("Invalid combo operand: {}", arg),
        }
    }

    fn step(&mut self, op:u8, arg:u8) -> Option<u8> {
        if VERBOSE {println!("{} {}: {}, {}, {}",
            op, arg, self.rega, self.regb, self.regc);}
        match op {
            0 => {  // ADV (division rega)
                self.rega = self.rega / 2i64.pow(self.cbo(arg) as u32);
                self.iptr += 2; None},
            1 => {  // BXL (bitwise xor literal)
                self.regb ^= arg as i64;
                self.iptr += 2; None},
            2 => {  // BST (set regb)
                self.regb = self.cbo(arg) & 0x7;
                self.iptr += 2; None},
            3 => {  // JNZ (jump if not zero)
                if self.rega != 0 {self.iptr = arg as usize}
                else {self.iptr += 2}; None},
            4 => {  // BXC (bitwise xor regc)
                self.regb ^= self.regc;
                self.iptr += 2; None},
            5 => {  // OUT (output)
                let tmp = self.cbo(arg) & 0x7;
                self.iptr += 2; Some(tmp as u8)},
            6 => {  // BDV (division regb)
                self.regb = self.rega / 2i64.pow(self.cbo(arg) as u32);
                self.iptr += 2; None},
            7 => {  // CDV (division regc)
                self.regc = self.rega / 2i64.pow(self.cbo(arg) as u32);
                self.iptr += 2; None},
            _ => panic!("Invalid opcode: {}", op),
        }
    }
}

struct Program {
    init: State,
    prog: Vec<u8>,
}

impl Program {
    fn new(input: &str) -> Self {
        let mut iter = input.trim().lines();
        let rega = parse::<i64>(iter.next().unwrap());
        let regb = parse::<i64>(iter.next().unwrap());
        let regc = parse::<i64>(iter.next().unwrap());
        let init = State {
            rega: rega[0],
            regb: regb[0],
            regc: regc[0],
            iptr: 0,
        };
        iter.next();
        let prog = parse::<u8>(iter.next().unwrap());
        return Program { init:init, prog:prog };
    }

    fn run(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut state = self.init.clone();
        while state.iptr + 1 < self.prog.len() {
            let op  = self.prog[state.iptr + 0];
            let arg = self.prog[state.iptr + 1];
            if let Some(x) = state.step(op, arg) {result.push(x);}
        }
        return result;
    }
}

fn part1(input: &str) -> String {
    let prog = Program::new(input);
    return format(&prog.run());
}

// By reverse-engineering, my input program and the example both simplify to
// functions of the 11 LSBs of A, dividing A by 8 each iteration until A = 0.
// Since we know the final state, we can guess one octal digit at a time to
// match the last N digits of the output, until we've matched the entire output.
fn part2(input: &str) -> i64 {
    // Read the desired input.
    let mut prog = Program::new(input);
    let plen = prog.prog.len();
    // Guess-and-check starting from the last digit.
    // Recurse on anything that matches more than the initial guess.
    // Search in ascending order to find smallest solution first.
    let mut queue = VecDeque::from([(0i64, 1usize)]);
    while let Some((prev, chk)) = queue.pop_front() {
        for guess in 0..8 {
            prog.init.rega = 8*prev + guess;
            let result = prog.run();
            if result == prog.prog {return prog.init.rega;}
            if result.len() < chk {continue;}
            if result[result.len()-chk..result.len()] == prog.prog[plen-chk..plen] {
                queue.push_back((prog.init.rega, chk+1));
            }
        }
    }
    panic!("No solution?");
}

const EXAMPLE1: &'static str = "\
    Register A: 2024
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0";

const EXAMPLE2: &'static str = "\
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0";

const EXAMPLE3: &'static str = "\
    Register A: 2024
    Register B: 0
    Register C: 0

    Program: 0,3,5,4,3,0";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 17).unwrap();

    assert_eq!(part1(EXAMPLE1), String::from("4,2,5,6,7,7,7,7,3,1,0"));
    assert_eq!(part1(EXAMPLE2), String::from("4,6,3,5,6,3,5,2,1,0"));
    assert_eq!(part2(EXAMPLE3), 117440);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
