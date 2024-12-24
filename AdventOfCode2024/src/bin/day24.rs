/// Advent of Code 2024, Day 24
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;

enum Logic {And, Or, Xor}

struct Gate {
    in0: Option<bool>,              // Wire-index of 1st input
    in1: Option<bool>,              // Wire-index of 2nd input
    out: usize,                     // Wire-index of output
    typ: Logic,                     // Logic function
}

struct Circuit {
    index: HashMap<String, usize>,  // Wire-label to index
    label: Vec<String>,             // Index to wire-label
    gates: Vec<Gate>,               // Vector of all logic gates
    value: Vec<Option<bool>>,       // Vector of wire values
    wires: Vec<Vec<usize>>,         // Vector of wire connections
    zwire: HashMap<usize, usize>,   // Special output wires (z00, z01, ...)
}

impl Gate {
    fn new(fun: &str, out:usize) -> Self {
        let typ = match fun {
            "AND"   => Logic::And,
            "OR"    => Logic::Or,
            "XOR"   => Logic::Xor,
            _       => panic!("Invalid logic gate: {}", fun),
        };
        Gate { in0:None, in1:None, out:out, typ:typ }
    }

    fn value(&self) -> Option<bool> {
        match (self.in0, self.in1, &self.typ) {
            (Some(a), Some(b), &Logic::And) => Some(a && b),
            (Some(a), Some(b), &Logic::Or)  => Some(a || b),
            (Some(a), Some(b), &Logic::Xor) => Some(a ^ b),
            _                               => None,
        }
    }
}

impl Circuit {
    fn new(input: &str) -> Self {
        // Parse the input.
        let mut ckt = Circuit {
            index: HashMap::new(),
            label: Vec::new(),
            gates: Vec::new(),
            value: Vec::new(),
            wires: Vec::new(),
            zwire: HashMap::new(),
        };
        let mut queue: Vec<usize> = Vec::new();
        for line in input.trim().lines() {
            if line.contains(":") {
                queue.push(ckt.add_input(line));
            } else if line.contains("->") {
                ckt.add_gate(line);
            }
        }

        // Propagate the initial state.
        while let Some(idx) = queue.pop() {
            let val = ckt.value[idx].unwrap();
            for gidx in ckt.wires[idx].iter() {
                let gate = &mut ckt.gates[gidx / 2];
                if gidx % 2 == 0 {gate.in0 = Some(val);}
                if gidx % 2 == 1 {gate.in1 = Some(val);}
                if let Some(x) = gate.value() {
                    ckt.value[gate.out] = Some(x);
                    queue.push(gate.out);
                }
            }
        }
        return ckt;
    }

    fn add_input(&mut self, line: &str) -> usize {
        // New initial value: "x00: 1"
        let tok: Vec<&str> = line.trim().split([':']).collect();
        let idx: usize = self.get_wire(tok[0]);
        let val: usize = tok[1].trim().parse().unwrap();
        self.value[idx] = Some(val > 0);
        return idx;
    }

    fn add_gate(&mut self, line: &str) {
        // New logic gate: "ntg XOR fgs -> mjb"
        let tok: Vec<&str> = line.trim().split([' ']).collect();
        let idx: usize = self.gates.len();
        let in0: usize = self.get_wire(tok[0]);
        let in1: usize = self.get_wire(tok[2]);
        let out: usize = self.get_wire(tok[4]);
        self.gates.push(Gate::new(tok[1], out));
        self.wires[in0].push(2*idx+0);
        self.wires[in1].push(2*idx+1);
    }

    fn get_wire(&mut self, lbl: &str) -> usize {
        if let Some(idx) = self.index.get(lbl)  {
            return *idx;    // Existing index
        } else {
            // Create the new wire.
            let tmp = self.wires.len();
            self.index.insert(String::from(lbl), tmp);
            self.label.push(String::from(lbl));
            self.value.push(None);
            self.wires.push(Vec::new());
            // Is this a special output wire?
            if lbl.starts_with('z') {
                let zidx: usize = lbl[1..].parse().unwrap();
                self.zwire.insert(zidx, tmp);
            }
            return tmp;     // New index
        }
    }

    fn value(&self) -> usize {
        let mut accum = 0usize;
        for (b,n) in self.zwire.iter() {
            let value = self.value[*n].unwrap();
            accum += (1usize << b) * (value as usize);
        }
        return accum;
    }
}

fn part1(input: &str) -> usize {
    Circuit::new(input).value()
}

fn part2(input:&str) -> usize {
    0 //???
}

const EXAMPLE1: &'static str = "\
    x00: 1
    x01: 1
    x02: 1
    y00: 0
    y01: 1
    y02: 0

    x00 AND y00 -> z00
    x01 XOR y01 -> z01
    x02 OR y02 -> z02";

const EXAMPLE2: &'static str = "\
    x00: 1
    x01: 0
    x02: 1
    x03: 1
    x04: 0
    y00: 1
    y01: 1
    y02: 1
    y03: 1
    y04: 1

    ntg XOR fgs -> mjb
    y02 OR x01 -> tnw
    kwq OR kpj -> z05
    x00 OR x03 -> fst
    tgd XOR rvg -> z01
    vdt OR tnw -> bfw
    bfw AND frj -> z10
    ffh OR nrd -> bqk
    y00 AND y03 -> djm
    y03 OR y00 -> psh
    bqk OR frj -> z08
    tnw OR fst -> frj
    gnj AND tgd -> z11
    bfw XOR mjb -> z00
    x03 OR x00 -> vdt
    gnj AND wpb -> z02
    x04 AND y00 -> kjc
    djm OR pbm -> qhw
    nrd AND vdt -> hwm
    kjc AND fst -> rvg
    y04 OR y02 -> fgs
    y01 AND x02 -> pbm
    ntg OR kjc -> kwq
    psh XOR fgs -> tgd
    qhw XOR tgd -> z09
    pbm OR djm -> kpj
    x03 XOR y03 -> ffh
    x00 XOR y04 -> ntg
    bfw OR bqk -> z06
    nrd XOR fgs -> wpb
    frj XOR qhw -> z04
    bqk OR frj -> z07
    y03 OR x01 -> nrd
    hwm AND bqk -> z03
    tgd XOR rvg -> z12
    tnw OR pbm -> gnj";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 24).unwrap();

    assert_eq!(part1(EXAMPLE1), 4);
    assert_eq!(part1(EXAMPLE2), 2024);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
