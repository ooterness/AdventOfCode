/// Day 14: https://adventofcode.com/2020/day/14
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
#[path = "common.rs"] mod common;

struct Program {
    mask0:  u64,
    mask1:  u64,
    maskx:  Vec<u64>,
    mem:    HashMap<u64, u64>,
    mode:   bool,
    debug:  bool,
}

impl Program {
    fn new(mode:bool, debug:bool) -> Program {
        Program {
            mask0:  0u64,
            mask1:  0u64,
            maskx:  Vec::new(),
            mem:    HashMap::new(),
            mode:   mode,
            debug:  debug,
        }
    }

    fn set_mask(&mut self, arg:&str) {
        self.mask0 = 0;
        self.mask1 = 0;
        self.maskx = Vec::new();
        for (n,c) in arg.chars().enumerate() {
            let m = 1u64 << (35-n);
            match c {
                '0' => self.mask0 |= m,
                '1' => self.mask1 |= m,
                'X' => self.maskx.push(m),
                _   => (),
            };
        }
    }

    fn multi_mem(&mut self, mask:&[u64], pos:u64, val:u64) {
        if let Some(m) = mask.first() {
            // Try each variant of the next mask bit.
            let pos0 = pos & !m;
            let pos1 = pos |  m;
            self.multi_mem(&mask[1..], pos0, val);
            self.multi_mem(&mask[1..], pos1, val);
        } else {
            // Base case: Write to designated address.
            if self.debug {println!("Pos {} = {}", pos, val);}
            self.mem.insert(pos, val);
        }
    }

    fn set_mem(&mut self, pos:&str, val:&str) {
        let pos:u64     = pos.parse().unwrap();
        let val:u64     = val.parse().unwrap();
        if self.mode {
            // Part 2: Masked address.
            let maskx = self.maskx.clone();
            self.multi_mem(&maskx, pos | self.mask1, val);
        } else {
            // Part 1: Masked value.
            self.mem.insert(pos, (val | self.mask1) & (!self.mask0));
        }
    }

    fn run_line(&mut self, line:&String) {
        let vec:Vec<&str> = line.split(" = ")
            .flat_map(|x| x.split(&['[',']'][..])).collect();
        if self.debug {println!("VEC = {:#?}", vec);}
        match (vec.get(0), vec.get(1), vec.get(3)) {
            (Some(cmd), Some(arg), None)
                if cmd == &"mask"   => self.set_mask(arg),
            (Some(cmd), Some(pos), Some(val))
                if cmd == &"mem"    => self.set_mem(pos,val),
            _ => eprintln!("Bad command: {}", line),
                
        }
    }

    fn run(prog:&[String], mode:bool) -> Program {
        let mut p = Program::new(mode, false);
        for line in prog.iter() {
            p.run_line(line);
        }
        p
    }

    fn sum(&self) -> u64 {
        if self.debug {println!("{:#?}", self.mem);}
        self.mem.values().map(|x| *x).sum()
    }
}

pub fn solve() {
    let example1 = vec![
        String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
        String::from("mem[8] = 11"),
        String::from("mem[7] = 101"),
        String::from("mem[8] = 0"),
    ];
    let example2 = vec![
        String::from("mask = 000000000000000000000000000000X1001X"),
        String::from("mem[42] = 100"),
        String::from("mask = 00000000000000000000000000000000X0XX"),
        String::from("mem[26] = 1"),
    ];
    let input = common::read_strings("input/input14.txt");

    let test1 = Program::run(&example1, false);
    assert_eq!(165, test1.sum());

    let test2 = Program::run(&example2, true);
    assert_eq!(208, test2.sum());

    let part1 = Program::run(&input, false);
    println!("Part1: {}", part1.sum());

    let part2 = Program::run(&input, true);
    println!("Part1: {}", part2.sum());
}
