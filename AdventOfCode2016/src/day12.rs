/// Advent of Code 2016, Day 12
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

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

enum Instruction {
    CpyI(i64, usize),
    CpyR(usize, usize),
    IncR(usize),
    DecR(usize),
    JnzI(i64, i64),
    JnzR(usize, i64),
}

struct State {
    regs: [i64;4],
    pctr: i64,
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
            ("cpy", Some(x), _, _, Some(y)) => Some(Instruction::CpyI(x, y)),
            ("cpy", _, Some(x), _, Some(y)) => Some(Instruction::CpyR(x, y)),
            ("inc", _, Some(x), _, _      ) => Some(Instruction::IncR(x)),
            ("dec", _, Some(x), _, _      ) => Some(Instruction::DecR(x)),
            ("jnz", Some(x), _, Some(y), _) => Some(Instruction::JnzI(x, y)),
            ("jnz", _, Some(x), Some(y), _) => Some(Instruction::JnzR(x, y)),
            _                => None
        }
    }

    fn exec(&self, st: &mut State) {
        match self {
            Instruction::CpyI(x, y) => {
                st.pctr += 1; st.regs[*y] = *x;},
            Instruction::CpyR(x, y) => {
                st.pctr += 1; st.regs[*y] = st.regs[*x];},
            Instruction::IncR(x)    => {
                st.pctr += 1; st.regs[*x] += 1;},
            Instruction::DecR(x)    => {
                st.pctr += 1; st.regs[*x] -= 1;},
            Instruction::JnzI(x, y) => {
                st.pctr += if *x != 0 {*y} else {1};},
            Instruction::JnzR(x, y) => {
                st.pctr += if st.regs[*x] != 0 {*y} else {1};},
        }
    }
}

impl State {
    fn new() -> Self {
        State { regs: [0;4], pctr: 0 }
    }
}

impl Program {
    fn new(input: &str) -> Self {
        let lines = input.trim().lines().filter_map(Instruction::new).collect();
        Program { instructions: lines }
    }

    fn run(&self, st: &mut State) {
        while 0 <= st.pctr {
            let pctr = self.instructions.get(st.pctr as usize);
            if let Some(x) = pctr {x.exec(st);} else {break;}
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut state = State::new();
    Program::new(input).run(&mut state);
    return state.regs[0];
}

fn part2(input: &str) -> i64 {
    let mut state = State::new();
    state.regs[2] = 1;
    Program::new(input).run(&mut state);
    return state.regs[0];
}

const TEST: &str = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 12).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), 42);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
