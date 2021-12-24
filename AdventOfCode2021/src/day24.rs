/// Day 24: https://adventofcode.com/2021/day/24
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

const VERBOSE:bool = true;

// ALU contains four registers (W, X, Y, Z)
type Regs = [i64;4];

// Helpers for command parsing.
fn str2num(x:Option<&str>) -> Option<i64> {
    if let Some(y) = x {y.parse().ok()} else {None}
}

fn str2reg(x:Option<&str>) -> Option<usize> {
    match x {
        Some("w") => Some(0),
        Some("x") => Some(1),
        Some("y") => Some(2),
        Some("z") => Some(3),
        _         => None,
    }
}

// Set of all ALU commands
enum Command {
    Input(usize),               // Pull next input
    AddNum(usize, i64),         // Add literal
    AddReg(usize, usize),       // Add register
    MulNum(usize, i64),         // Multiply literal
    MulReg(usize, usize),       // Multiply register
    DivNum(usize, i64),         // Divide literal
    DivReg(usize, usize),       // Divide register
    ModNum(usize, i64),         // Modulo literal
    ModReg(usize, usize),       // Modulo register
    EqlNum(usize, i64),         // Equality-test literal
    EqlReg(usize, usize),       // Equality-test register
}

impl Command {
    fn new(line: &str) -> Option<Command> {
        // All commands have an opcode and an output register.
        // Optional third argument may be a literal or an input register.
        let mut iter = line.split(' ');
        let opcode = iter.next().unwrap();
        let outreg = str2reg(iter.next()).unwrap();
        let intemp = iter.next();
        let inval  = str2num(intemp);
        let inreg  = str2reg(intemp);
        // Pattern-match each possible command.
        match (opcode, inval, inreg) {
            ("inp",_,_)       => Some(Command::Input(outreg)),
            ("add",Some(x),_) => Some(Command::AddNum(outreg, x)),
            ("add",_,Some(x)) => Some(Command::AddReg(outreg, x)),
            ("mul",Some(x),_) => Some(Command::MulNum(outreg, x)),
            ("mul",_,Some(x)) => Some(Command::MulReg(outreg, x)),
            ("div",Some(x),_) => Some(Command::DivNum(outreg, x)),
            ("div",_,Some(x)) => Some(Command::DivReg(outreg, x)),
            ("mod",Some(x),_) => Some(Command::ModNum(outreg, x)),
            ("mod",_,Some(x)) => Some(Command::ModReg(outreg, x)),
            ("eql",Some(x),_) => Some(Command::EqlNum(outreg, x)),
            ("eql",_,Some(x)) => Some(Command::EqlReg(outreg, x)),
            _                 => None,
        }
    }

    fn exec(&self, reg:&Regs, digit:i64) -> Regs {
        let mut result = reg.clone();
        match self {
            Command::Input(a)     => result[*a] = digit,
            Command::AddNum(a, b) => result[*a] = reg[*a] + *b,
            Command::AddReg(a, b) => result[*a] = reg[*a] + reg[*b],
            Command::MulNum(a, b) => result[*a] = reg[*a] * *b,
            Command::MulReg(a, b) => result[*a] = reg[*a] * reg[*b],
            Command::DivNum(a, b) => result[*a] = reg[*a] / *b,
            Command::DivReg(a, b) => result[*a] = reg[*a] / reg[*b],
            Command::ModNum(a, b) => result[*a] = reg[*a] % *b,
            Command::ModReg(a, b) => result[*a] = reg[*a] % reg[*b],
            Command::EqlNum(a, b) => result[*a] = if reg[*a] == *b {1} else {0},
            Command::EqlReg(a, b) => result[*a] = if reg[*a] == reg[*b] {1} else {0},
        };
        result
    }
}

// A program is a list of commands.
struct Program {
    cmds: Vec<Command>,
}

impl Program {
    fn new(filename: &str) -> Program {
        let lines = common::read_lines(filename);
        let cmds = lines.into_iter()
            .map(|l| Command::new(&l).unwrap())
            .collect();
        Program { cmds:cmds }
    }

    fn run(&self) -> RegState {
        let mut state = RegState::new();
        for (n,cmd) in self.cmds.iter().enumerate() {
            state = state.exec(cmd);
            if VERBOSE {
                println!("  Cmd {}/{}: {} states",
                    n, self.cmds.len(), state.states.len());
            }
        }
        state
    }
}

// For each possible register state, track the highest and lowest
// cumulative input that can reach that state.
type MinMax = (i64,i64);
const MM_INIT: MinMax = (i64::MAX, i64::MIN);

struct RegState {
    states: HashMap<Regs,MinMax>,
}

impl RegState {
    fn new() -> RegState {
        let mut states = HashMap::new();
        states.insert([0,0,0,0], (0,0));
        RegState { states:states }
    }

    fn exec(&self, cmd:&Command) -> RegState {
        let mut states = HashMap::new();
        for (prev,input) in self.states.iter() {
            if let Command::Input(_) = cmd {
                // Input creates a new branch for each input.
                // The output states are guaranteed unique, so direct insert OK.
                for digit in 1..10i64 {
                    let regs = cmd.exec(prev, digit);
                    let next = (10*input.0 + digit, 10*input.1 + digit);
                    states.insert(regs, next);
                }
            } else {
                // All other commands execute in place.
                // As we insert, check min/max for various paths to the same state.
                let regs = cmd.exec(prev, 0);
                let next = states.entry(regs).or_insert(MM_INIT);
                *next = (min(next.0, input.0), max(next.1, input.1));
            }
        }
        RegState { states:states }
    }
}

// Find the minimum and maximum values that satisfy the Z=0 constraint.
fn monad(regs: &RegState) -> MinMax {
    let mut result = MM_INIT;
    for (state,input) in regs.states.iter() {
        if state[3] == 0 {
            result.0 = min(result.0, input.0);
            result.1 = max(result.1, input.1);
        }
    }
    result
}

pub fn solve() {
    // Run the test program.
    let test = Program::new("input/test24.txt").run();
    assert_eq!(test.states.get(&[0,0,0,1]), Some(&(1,1)));
    assert_eq!(test.states.get(&[0,0,1,0]), Some(&(2,2)));
    assert_eq!(test.states.get(&[0,0,1,1]), Some(&(3,3)));
    assert_eq!(test.states.get(&[0,1,0,0]), Some(&(4,4)));
    assert_eq!(test.states.get(&[0,1,0,1]), Some(&(5,5)));
    assert_eq!(test.states.get(&[0,1,1,0]), Some(&(6,6)));
    assert_eq!(test.states.get(&[0,1,1,1]), Some(&(7,7)));
    assert_eq!(test.states.get(&[1,0,0,0]), Some(&(8,8)));
    assert_eq!(test.states.get(&[1,0,0,1]), Some(&(9,9)));
    assert_eq!(test.states.get(&[1,0,1,0]), None);

    // Apply the Z=0 constraint to the test result.
    assert_eq!(monad(&test), (2,8));

    // Run the main program and apply Z=0 constraint.
    let data = Program::new("input/input24.txt").run();
    let soln = monad(&data);
    println!("Part1: {}", soln.1);  // Maximum
    println!("Part2: {}", soln.0);  // Minimum
}
