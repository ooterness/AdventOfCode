/// Advent of Code 2024, Day 24
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

const VERBOSE: usize = 0;

#[derive(Debug, Eq, PartialEq)]
enum Logic {And, Or, Xor}

struct Gate {
    in0: usize,                     // Wire-index of 1st input
    in1: usize,                     // Wire-index of 2nd input
    out: usize,                     // Wire-index of output
    typ: Logic,                     // Logic function
}

struct Circuit {
    index: HashMap<String, usize>,  // Wire-label to index
    label: Vec<String>,             // Index to wire-label
    gates: Vec<Gate>,               // Vector of all logic gates
    swaps: HashSet<usize>,          // List of swapped items
    value: Vec<Option<bool>>,       // Vector of wire values
    wires: Vec<HashSet<usize>>,     // Vector of wire connections
    xwire: HashMap<usize, usize>,   // Special input wires (x00, x01, ...)
    ywire: HashMap<usize, usize>,   // Special input wires (y00, y01, ...)
    zwire: HashMap<usize, usize>,   // Special output wires (z00, z01, ...)
}

impl Gate {
    fn new(in0:usize, fun:&str, in1:usize, out:usize) -> Self {
        let typ = match fun {
            "AND"   => Logic::And,
            "OR"    => Logic::Or,
            "XOR"   => Logic::Xor,
            _       => panic!("Invalid logic gate: {}", fun),
        };
        Gate { in0:in0, in1:in1, out:out, typ:typ }
    }

    fn value(&self, value:&Vec<Option<bool>>) -> Option<bool> {
        match (value[self.in0], value[self.in1], &self.typ) {
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
            swaps: HashSet::new(),
            value: Vec::new(),
            wires: Vec::new(),
            xwire: HashMap::new(),
            ywire: HashMap::new(),
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
            for &gidx in ckt.wires[idx].iter() {
                let gate = &ckt.gates[gidx];
                if let Some(x) = gate.value(&ckt.value) {
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
        self.gates.push(Gate::new(in0, tok[1], in1, out));
        self.wires[in0].insert(idx);
        self.wires[in1].insert(idx);
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
            self.wires.push(HashSet::new());
            // Is this a special input or output wire?
            if lbl.starts_with('x') {
                let bidx: usize = lbl[1..].parse().unwrap();
                self.xwire.insert(bidx, tmp);
            }
            if lbl.starts_with('y') {
                let bidx: usize = lbl[1..].parse().unwrap();
                self.ywire.insert(bidx, tmp);
            }
            if lbl.starts_with('z') {
                let bidx: usize = lbl[1..].parse().unwrap();
                self.zwire.insert(bidx, tmp);
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

    // Human-readable description of a given gate.
    fn gate_label(&self, gidx:usize) -> String {
        let gate = &self.gates[gidx];
        return format!("{} {:?} {} -> {}",
            self.label[gate.in0],
            gate.typ,
            self.label[gate.in1],
            self.label[gate.out]);
    }

    // Find a gate connected to the specified input wire.
    fn find_gate(&self, typ:Logic, input:usize) -> Option<usize> {
        for (n,gate) in self.gates.iter().enumerate() {
            if gate.typ == typ && self.wires[input].contains(&n) {return Some(n);}
        }
        return None;
    }

    // Find the gate currently connected to the given circuit.
    fn find_source(&self, wire:usize) -> usize {
        for (n,gate) in self.gates.iter().enumerate() {
            if gate.out == wire {return n;}
        }
        panic!("No source for output #{}", wire);
    }

    // Connect a gate to the specified output, swapping as needed with another gate.
    fn set_output(&mut self, gidx:usize, wire:usize) {
        if self.gates[gidx].out != wire {
            let gtmp = self.find_source(wire);
            let wtmp = self.gates[gidx].out;
            assert!(self.swaps.insert(wtmp));
            assert!(self.swaps.insert(wire));
            self.gates[gtmp].out = wtmp;
            self.gates[gidx].out = wire;
        }
    }

    // The expected input circuit is a ripple-carry adder.
    // For the least significant bit (LSB):
    //  z[0] = (x[0] XOR y[0])                          (G0)
    //  c[0] = (x[0] AND y[0])                          (G1)
    // For all intermediate bits:
    //  p[n] = (x[n] XOR y[n])                          (G0)
    //  c[n] = (x[n] AND y[n]) OR (p[n] AND c[n-1])     (G1) G2 (G3)
    //  z[n] =                    (p[n] XOR c[n-1])             (G4)
    // For the most significant bit (MSB):
    //  z[n] = c[n-1]
    // All expected gates are present and their input connections are
    // correct. Fix output connections as we go, noting discrepancies.
    fn find_problems(&mut self) {
        // Find all gates tied directly to the inputs (G0, G1).
        let msb = self.xwire.len();
        let g0: Vec<usize> = (0..msb)
            .map(|b| self.find_gate(Logic::Xor, self.xwire[&b]).unwrap())
            .collect();
        let g1: Vec<usize> = (0..msb)
            .map(|b| self.find_gate(Logic::And, self.xwire[&b]).unwrap())
            .collect();

        // The first XOR gate should be tied directly to the output LSB.
        self.set_output(g0[0], self.zwire[&0]);

        // Find the gates tied directly to the outputs (G4).
        let mut g4: HashMap<usize,usize> = HashMap::new();
        for b in 1..msb {
            // First pass: Find the obvious XOR gates tied directly to output.
            // (Confirmed by hand that this assumption is safe for my input.)
            let g = self.find_source(self.zwire[&b]);
            if self.gates[g].typ == Logic::Xor && !g0.contains(&g) {
                g4.insert(b, g);
            }
        }
        for b in 1..msb {
            // Second pass: Assign remaining outputs by swapping with an
            //  unassigned XOR gate that's attached to the output of G0[b].
            // (Cross fingers that gate's output hasn't also been swapped.)
            if g4.contains_key(&b) {continue;}  // Already found.
            let src = self.gates[g0[b]].out;    // Expected connection
            let g = self.find_gate(Logic::Xor, src).unwrap();
            self.set_output(g, self.zwire[&b]);
            g4.insert(b, g);
        }

        // Assign each AND gate G3 based on shared inputs with G4.
        let g3: HashMap<usize,usize> = (1..msb)
            .map(|b| (b, self.find_gate(Logic::And, self.gates[g4[&b]].in0).unwrap()))
            .collect();

        // Optional diagnostic logging?
        if VERBOSE > 1 {
            for b in 1..msb {
                println!("Bit {:02}: {}  {}  {}  {}", b,
                    self.gate_label(g0[b]),
                    self.gate_label(g1[b]),
                    self.gate_label(g3[&b]),
                    self.gate_label(g4[&b]));
            }
        }

        // Finally, identify carry bits and assign each OR gate G2.
        let mut wc: HashMap<usize,usize> = HashMap::new();  // Carry output from stage
        let mut wp: HashMap<usize,usize> = HashMap::new();  // Parity bit (X[n] XOR Y[n])
        let mut g2: HashMap<usize,usize> = HashMap::new();
        for b in 1..msb {
            // Cross-reference outputs from G1[n] or G3[n].
            let x = self.find_gate(Logic::Or, self.gates[g1[b]].out);
            let y = self.find_gate(Logic::Or, self.gates[g3[&b]].out);
            // Cross-reference carry input to G3[n+1] or G4[n+1].
            let z = if b+1 < msb {
                // Lookup both inputs. (G3 and G4 should be the same,
                //  but we don't know which input wire is the carry.)
                let w0 = self.gates[g4[&(b+1)]].in0;
                let w1 = self.gates[g4[&(b+1)]].in1;
                let g0 = self.find_source(w0);
                let g1 = self.find_source(w1);
                // Identify which input is which by the driving gate type(s).
                let w0_is_carry = (self.gates[g0].typ == Logic::Or) || (self.gates[g1].typ == Logic::Xor);
                let w1_is_carry = (self.gates[g1].typ == Logic::Or) || (self.gates[g0].typ == Logic::Xor);
                if w0_is_carry && !w1_is_carry {
                    wc.insert(b, w0); wp.insert(b+1, w1); g0
                } else if w1_is_carry && !w0_is_carry {
                    wc.insert(b, w1); wp.insert(b+1, w0); g1
                } else {
                    panic!("Cannot identify carry[{}]", b);
                }
            } else {
                self.find_source(self.zwire[&msb])
            };
            // Cross fingers that at least two references agree.
            if x == y {
                g2.insert(b, x.unwrap());
            } else if x == Some(z) {
                g2.insert(b, z);
            } else if y == Some(z) {
                g2.insert(b, z);
            } else {
                panic!("Cannot agree on G2[{}]. {:?} {:?} {}", b, x, y, z);
            }
        }

        // The final output should be tied to the last carry bit.
        self.set_output(g2[&(msb-1)], self.zwire[&msb]);

        // Sanity-check outputs from each gate.
        for b in 1..msb {
            if let Some(w) = wp.get(&b) {self.set_output(g0[b], *w)};
            if let Some(w) = wc.get(&b) {self.set_output(g2[&b], *w)};
            if self.gates[g2[&b]].in0 == self.gates[g1[b]].out
                { self.set_output(g3[&b], self.gates[g2[&b]].in1); }
            if self.gates[g2[&b]].in1 == self.gates[g1[b]].out
                { self.set_output(g3[&b], self.gates[g2[&b]].in0); }
            if self.gates[g2[&b]].in0 == self.gates[g3[&b]].out
                { self.set_output(g1[b], self.gates[g2[&b]].in1); }
            if self.gates[g2[&b]].in1 == self.gates[g3[&b]].out
                { self.set_output(g1[b], self.gates[g2[&b]].in0); }
        }

        // Optional diagnostic logging?
        if VERBOSE > 0 {
            println!("Bit 00: {}  {}",
                self.gate_label(g0[0]),
                self.gate_label(g1[0]));
            for b in 1..msb {
                println!("Bit {:02}: {}  {}  {}  {}  {}", b,
                    self.gate_label(g0[b]),
                    self.gate_label(g1[b]),
                    self.gate_label(g2[&b]),
                    self.gate_label(g3[&b]),
                    self.gate_label(g4[&b]));
            }
        }
    }
}

fn part1(input: &str) -> usize {
    Circuit::new(input).value()
}

fn part2(input:&str) -> String {
    let mut ckt = Circuit::new(input);
    ckt.find_problems();
    let mut lbl: Vec<String> = ckt.swaps.iter()
        .map(|&wire| ckt.label[wire].clone()).collect();
    lbl.sort();
    return lbl.join(",");
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
