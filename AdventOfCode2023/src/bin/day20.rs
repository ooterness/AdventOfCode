/// Advent of Code 2023, Day 20
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::VecDeque;

const DEBUG: usize = 0;

// All possible types for a given module.
type SrcMap = HashMap<usize, usize>;
type Pulse = (usize, usize, bool);
enum Action {
    Flop(usize),    // Flip-flop module, toggle on low pulse
    Conj(SrcMap),   // Conjunction, NAND previous pulse each input
    Bcast,          // Broadcast, repeat input to all outputs
}

struct Module {
    label: String,                  // Label for this module
    index: usize,                   // Index of this module
    mtype: Action,                  // Module type
    outputs: Vec<usize>,            // Output indices
}

struct State {
    states: Vec<bool>,              // State of all modules
    count0: usize,                  // Cumulative low pulses
    count1: usize,                  // Cumulative high pulses
}

struct Network {
    labels: HashMap<String, usize>, // Map labels to module index
    modules: Vec<Module>,           // Modules in this network
    states: usize,                  // Number of internal states
}

impl Module {
    // Parse descriptor -> Type, label, and output list.
    fn parse<'a>(line: &'a str) -> (char, &'a str, &'a str) {
        let tok: Vec<&str> = line.trim().split(" -> ").collect();
        return match tok[0].chars().nth(0) {
            Some('%') => ('%', &tok[0][1..], tok[1]),
            Some('&') => ('&', &tok[0][1..], tok[1]),
            _         => ('B', &tok[0][0..], tok[1]),
        };
    }

    // Create a placeholder module from its description.
    fn create(line: &str, net: &mut Network) {
        // Parse the type and label; ignore outputs for now.
        let (ch, lbl, _) = Module::parse(line.trim());
        let idx = net.modules.len();
        let typ = match ch {
            '%' => {net.states += 1; Action::Flop(net.states-1)},
            '&' => Action::Conj(SrcMap::new()),
            _   => Action::Bcast,
        };
        net.labels.insert(lbl.to_string(), idx);
        net.modules.push( Module {
            label: lbl.to_string(),
            index: idx, mtype: typ,
            outputs: Vec::new(),
        } );
    }

    // Register output connections.
    fn connect(&mut self, line: &str, labels: &HashMap<String,usize>) -> Vec<usize> {
        // Parse the output list and lookup each destination index.
        let (_, _, dstr) = Module::parse(line);
        for lbl in dstr.split(',') {
            let didx = labels.get(lbl.trim()).cloned();
            self.outputs.push(didx.unwrap_or(usize::MAX));
        }
        return self.outputs.clone();
    }

    // If applicable, register an input connection.
    fn accept(&mut self, src: usize, idx: &mut usize) {
        match &mut self.mtype {
            Action::Conj(map) => {map.insert(src, *idx); *idx += 1;}
            _ => (),
        }
    }

    // Update system state based on the given input pulse.
    fn pulse(&self, state:&mut State, src:usize, val:bool) -> Vec<Pulse> {
        // Update state and generate pulse if applicable.
        let mut out: Option<bool> = None;
        match &self.mtype {
            Action::Flop(st) => {
                // Flip-flop: 
                if !val {
                    let new_state = !state.states[*st];
                    state.states[*st] = new_state;
                    out = Some(new_state);
                }
            },
            Action::Conj(st) => {
                state.states[st[&src]] = val;
                out = Some(!st.values().all(|s| state.states[*s]));
            },
            Action::Bcast => {
                out = Some(val);
            },
        }
        // If a pulse was generated, propagate it to all outputs.
        if let Some(x) = out {
            return self.outputs.iter().map(|dst| (self.index, *dst, x)).collect();
        } else {
            return Vec::new();
        }
    }
}

impl Network {
    // Create a network from its description.
    fn new(input: &str) -> Self {
        let mut net = Network { labels:HashMap::new(), modules:Vec::new(), states:0usize };
        // First pass creates labels and placeholder modules.
        for line in input.trim().lines() {
            Module::create(line, &mut net);
        }
        // Second pass registers input and output connections.
        for (n,line) in input.trim().lines().enumerate() {
            for dst in net.modules[n].connect(line, &net.labels).into_iter() {
                if dst < net.modules.len() {
                    net.modules[dst].accept(n, &mut net.states);
                }
            }
        }
        return net;
    }

    // Create the initial state for this network.
    fn init(&self) -> State {
        State {
            states: vec![false; self.states],
            count0: 0, count1: 0,
        }
    }

    // Fetch label by module index.
    fn module_name<'a>(&'a self, index: usize) -> &'a str {
        if index < self.modules.len() {
            &self.modules[index].label
        } else {
            "output"
        }
    }

    // Press the button and run to completion.
    fn press(&self, state:&mut State) {
        let bcast: usize = *self.labels.get("broadcaster").unwrap();
        let mut queue = VecDeque::<Pulse>::new();
        queue.push_back((bcast, bcast, false));
        while let Some((src, dst, val)) = queue.pop_front() {
            if val {state.count1 += 1;}
            else   {state.count0 += 1;}
            if DEBUG >= 2 {
                println!("{} {} -> {}",
                    self.module_name(src),
                    if val {"hi"} else {"lo"},
                    self.module_name(dst),
                );
            }
            if dst < self.modules.len() {
                queue.extend(self.modules[dst].pulse(state, src, val));
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let net = Network::new(input);
    let mut state = net.init();
    for _ in 0..1000 {net.press(&mut state);}
    if DEBUG >= 1 {println!("TALLY: lo x {}, hi x {}", state.count0, state.count1);}
    return state.count0 * state.count1;
}

fn part2(_input: &str) -> usize {
    0
}

const EXAMPLE1: &'static str = "\
    broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a";

const EXAMPLE2: &'static str = "\
    broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 20).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE1), 32000000);
    assert_eq!(part1(EXAMPLE2), 11687500);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
