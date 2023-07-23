/// Advent of Code 2015, Day 7
/// Copyright 2023 by Alex Utter

use std::collections::HashMap;
#[path = "fetch.rs"] mod fetch;

#[derive(Clone,Copy)]
enum Source {
    Fixed(u16),
    Wire(usize),
}

#[derive(Clone,Copy)]
enum Operator {
    Buffer(Source),
    Not(Source),
    And(Source, Source),
    Or(Source, Source),
    Lshift(Source, u8),
    Rshift(Source, u8),
}

struct Circuit {
    labels: HashMap<String, usize>,         // Label for each wire
    gates: HashMap<usize, Operator>,        // Gate driving each wire
    wires: HashMap<usize, u16>,             // Wire value, if known
    verbose: bool,                          // Enable diagnostics?
}

impl Circuit {
    fn new(input: &str, verbose: bool) -> Circuit {
        let mut ckt = Circuit {
            labels: HashMap::new(),
            gates: HashMap::new(),
            wires: HashMap::new(),
            verbose: verbose,
        };
        for line in input.lines() {
            if verbose {println!("Gate: {}", line.trim());}
            ckt.add_gate(line.trim());
        }
        return ckt;
    }

    fn add_gate(&mut self, line: &str) {
        let tok: Vec<&str> = line.split(" ").collect();
        assert!(tok.len() >= 3);
        let output = self.get_wire(tok[tok.len()-1]);
        let gate = match (tok[0], tok[1], tok[2]) {
            (x, "->", _)        => Operator::Buffer(self.get_source(x)),
            ("NOT", x, "->")    => Operator::Not(self.get_source(x)),
            (x, "AND", y)       => Operator::And(self.get_source(x), self.get_source(y)),
            (x, "OR",  y)       => Operator::Or(self.get_source(x), self.get_source(y)),
            (x, "LSHIFT", y)    => Operator::Lshift(self.get_source(x), y.parse().unwrap()),
            (x, "RSHIFT", y)    => Operator::Rshift(self.get_source(x), y.parse().unwrap()),
            (_, _, _)           => panic!("Invalid circuit."),
        };
        self.gates.insert(output, gate);
    }

    fn force(&mut self, lbl: &str, val: u16) {
        // Purge cache and force designated circuit to a specific value.
        let idx = self.get_wire(lbl);
        self.wires.clear();
        self.wires.insert(idx, val);
    }

    fn get_source(&mut self, lbl: &str) -> Source {
        if let Some(value) = lbl.parse().ok() {
            Source::Fixed(value)
        } else {
            Source::Wire(self.get_wire(lbl))
        }
    }

    fn get_wire(&mut self, lbl: &str) -> usize {
        if let Some(idx) = self.labels.get(lbl) {
            return *idx;        // Existing wire
        } else {
            let new_idx = self.labels.len();
            if self.verbose {println!("Wire: {} = #{}", lbl, new_idx);}
            self.labels.insert(String::from(lbl), new_idx);
            return new_idx;     // Create new wire
        }
    }

    fn solve(&mut self, wire: Source) -> u16 {
        match wire {
            Source::Fixed(x) => x,
            Source::Wire(w) => self.solve_idx(w),
        }
    }

    fn solve_idx(&mut self, wire: usize) -> u16 {
        // Return cached value if available.
        if let Some(value) = self.wires.get(&wire) {return *value;}
        // Recursively calculate the output.
        if self.verbose {println!("Solving #{}...", wire);}
        let gate: Operator = self.gates.get(&wire).unwrap().clone();
        let value: u16 = match gate {
            Operator::Buffer(x)     => self.solve(x),
            Operator::Not(x)        => !self.solve(x),
            Operator::And(x, y)     => self.solve(x) & self.solve(y),
            Operator::Or(x, y)      => self.solve(x) | self.solve(y),
            Operator::Lshift(x, y)  => self.solve(x) << y,
            Operator::Rshift(x, y)  => self.solve(x) >> y,
        };
        // Update cache for next time around.
        if self.verbose {println!("Solve #{} = {}", wire, value);}
        self.wires.insert(wire, value);
        return value;
    }

    fn solve_lbl(&mut self, wire: &str) -> u16 {
        let idx = self.get_wire(wire);
        self.solve(Source::Wire(idx))
    }
}

fn part1(input: &str) -> u16
{
    let mut ckt = Circuit::new(input, false);
    return ckt.solve_lbl("a");
}

fn part2(input: &str) -> u16
{
    let mut ckt = Circuit::new(input, false);
    let val = ckt.solve_lbl("a");
    ckt.force("b", val);
    return ckt.solve_lbl("a");
}

const TEST: &'static str = "\
    123 -> x
    456 -> y
    x AND y -> d
    x OR y -> e
    x LSHIFT 2 -> f
    y RSHIFT 2 -> g
    NOT x -> h
    NOT y -> i";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 7).unwrap();

    // Unit tests on provided example.
    let mut test = Circuit::new(TEST, false);
    assert_eq!(test.solve_lbl("d"), 72);
    assert_eq!(test.solve_lbl("e"), 507);
    assert_eq!(test.solve_lbl("f"), 492);
    assert_eq!(test.solve_lbl("g"), 114);
    assert_eq!(test.solve_lbl("h"), 65412);
    assert_eq!(test.solve_lbl("i"), 65079);
    assert_eq!(test.solve_lbl("x"), 123);
    assert_eq!(test.solve_lbl("y"), 456);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
