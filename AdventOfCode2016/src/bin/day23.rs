/// Advent of Code 2016, Day 23
/// Copyright 2023 by Alex Utter

use aocfetch;

fn parse_int(x: Option<&&str>) -> Option<i64> {
    if let Some(y) = x {y.parse().ok()} else {None}
}

fn parse_reg(x: Option<&&str>) -> Option<usize> {
    match x {
        Some(&"a")  => Some(0),
        Some(&"b")  => Some(1),
        Some(&"c")  => Some(2),
        Some(&"d")  => Some(3),
        _           => None,
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    DecR(usize),
    IncR(usize),
    TglR(usize),
    CpyIR(i64, usize),
    CpyRR(usize, usize),
    JnzII(i64, i64),
    JnzIR(i64, usize),
    JnzRI(usize, i64),
    JnzRR(usize, usize),
}

#[derive(Clone, Debug)]
struct State {
    regs: [i64;4],      // Register value
    pctr: i64,          // Program counter (current instruction)
    tgct: Vec<usize>,   // Toggle counter for each instruction
}

struct Program {
    instructions: Vec<Instruction>,
}

impl Instruction {
    fn new(line: &str) -> Option<Self> {
        let tokens: Vec<&str> = line.trim().split(' ').collect();
        let int1 = parse_int(tokens.get(1));
        let int2 = parse_int(tokens.get(2));
        let reg1 = parse_reg(tokens.get(1));
        let reg2 = parse_reg(tokens.get(2));
        match (tokens[0], int1, reg1, int2, reg2) {
            ("dec", _, Some(x), _, _      ) => Some(Instruction::DecR(x)),
            ("inc", _, Some(x), _, _      ) => Some(Instruction::IncR(x)),
            ("tgl", _, Some(x), _, _      ) => Some(Instruction::TglR(x)),
            ("cpy", Some(x), _, _, Some(y)) => Some(Instruction::CpyIR(x, y)),
            ("cpy", _, Some(x), _, Some(y)) => Some(Instruction::CpyRR(x, y)),
            ("jnz", Some(x), _, Some(y), _) => Some(Instruction::JnzII(x, y)),
            ("jnz", Some(x), _, _, Some(y)) => Some(Instruction::JnzIR(x, y)),
            ("jnz", _, Some(x), Some(y), _) => Some(Instruction::JnzRI(x, y)),
            ("jnz", _, Some(x), _, Some(y)) => Some(Instruction::JnzRR(x, y)),
            _ => None,
        }
    }

    fn exec(&self, st: &mut State) {
        let tog = self.toggle(st.tgct[st.pctr as usize]);
        match tog {
            None => {st.pctr += 1;}, // Ignore invalid instructions
            Some(Instruction::DecR(x)) => {
                st.pctr += 1; st.regs[x] -= 1;},
            Some(Instruction::IncR(x)) => {
                st.pctr += 1; st.regs[x] += 1;},
            Some(Instruction::TglR(x)) => {
                let y = st.pctr + st.regs[x];
                if 0 <= y && (y as usize) < st.tgct.len() {
                    st.tgct[y as usize] += 1;
                }
                st.pctr += 1;},
            Some(Instruction::CpyIR(x, y)) => {
                st.pctr += 1; st.regs[y] = x;},
            Some(Instruction::CpyRR(x, y)) => {
                st.pctr += 1; st.regs[y] = st.regs[x];},
            Some(Instruction::JnzII(x, y)) => {
                st.pctr += if x != 0 {y} else {1};},
            Some(Instruction::JnzIR(x, y)) => {
                st.pctr += if x != 0 {st.regs[y]} else {1};},
            Some(Instruction::JnzRI(x, y)) => {
                st.pctr += if st.regs[x] != 0 {y} else {1};},
            Some(Instruction::JnzRR(x, y)) => {
                st.pctr += if st.regs[x] != 0 {st.regs[y]} else {1};},
        }
    }

    fn toggle(&self, tgct: usize) -> Option<Self> {
        // No toggles -> Return original instruction.
        if tgct == 0 {return Some(self.clone());}
        match (tgct % 2, self) {
            // Double toggle -> Loopback except for TGL.
            (0, Instruction::TglR(x))       => Some(Instruction::DecR(*x)),
            (0, _)                          => Some(self.clone()),
            // Toggle mode for all other instructions...
            (_, Instruction::IncR(x))       => Some(Instruction::DecR(*x)),
            (_, Instruction::DecR(x))       => Some(Instruction::IncR(*x)),
            (_, Instruction::TglR(x))       => Some(Instruction::IncR(*x)),
            (_, Instruction::CpyIR(x, y))   => Some(Instruction::JnzIR(*x, *y)),
            (_, Instruction::CpyRR(x, y))   => Some(Instruction::JnzRR(*x, *y)),
            (_, Instruction::JnzII(_, _))   => None,
            (_, Instruction::JnzIR(x, y))   => Some(Instruction::CpyIR(*x, *y)),
            (_, Instruction::JnzRI(_, _))   => None,
            (_, Instruction::JnzRR(x, y))   => Some(Instruction::CpyRR(*x, *y)),
        }
    }
}

impl State {
    fn new(prog: &Program) -> Self {
        let tgct = vec![0; prog.len()];
        State { regs:[0;4], pctr:0, tgct:tgct }
    }
}

impl Program {
    fn new(input: &str) -> Self {
        let lines = input.trim().lines().filter_map(Instruction::new).collect();
        Program { instructions: lines }
    }

    fn len(&self) -> usize {
        self.instructions.len()
    }

    fn run(&self, st: &mut State) {
        while 0 <= st.pctr {
            let pctr = self.instructions.get(st.pctr as usize);
            if let Some(x) = pctr {x.exec(st);} else {break;}
        }
    }
}

fn part1(input: &str) -> i64 {
    let prog = Program::new(input);
    let mut state = State::new(&prog);
    state.regs[0] = 7;      // Input from keypad
    prog.run(&mut state);
    return state.regs[0];   // Final value of register A
}

fn part2(input: &str) -> i64 {
    let prog = Program::new(input);
    let mut state = State::new(&prog);
    state.regs[0] = 12;     // Input from keypad
    prog.run(&mut state);
    return state.regs[0];   // Final value of register A
}

const TEST: &str = "\
cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 23).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), 3);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
