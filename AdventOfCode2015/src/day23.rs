/// Advent of Code 2015, Day 23
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

enum Opcode {
    HalfA,              // Halve designated register
    HalfB,              // Halve designated register
    TripleA,            // Triple designated register
    TripleB,            // Triple designated register
    IncrA,              // Increment designated register
    IncrB,              // Increment designated register
    Jump(i64),          // Jump unconditional
    JumpEvenA(i64),     // Jump if register is even
    JumpEvenB(i64),     // Jump if register is even
    JumpOneA(i64),      // Jump if register is one
    JumpOneB(i64),      // Jump if register is one
}

impl Opcode {
    fn new(input: &str) -> Self {
        match &input.trim()[..5] {
            "hlf a" => Opcode::HalfA,
            "hlf b" => Opcode::HalfB,
            "tpl a" => Opcode::TripleA,
            "tpl b" => Opcode::TripleB,
            "inc a" => Opcode::IncrA,
            "inc b" => Opcode::IncrB,
            "jie a" => Opcode::JumpEvenA(Opcode::get_offset(input)),
            "jie b" => Opcode::JumpEvenB(Opcode::get_offset(input)),
            "jio a" => Opcode::JumpOneA(Opcode::get_offset(input)),
            "jio b" => Opcode::JumpOneB(Opcode::get_offset(input)),
            _       => Opcode::Jump(Opcode::get_offset(input)),
        }
    }

    fn get_offset(input: &str) -> i64 {
        let words: Vec<&str> = input.split(' ').collect();
        return words[words.len()-1].parse().unwrap();
    }
}

struct State {
    pctr: i64,
    reg_a: usize,
    reg_b: usize,
}

impl State {
    fn new() -> Self {
        State { pctr:0, reg_a:0, reg_b:0 }
    }

    fn next(&self, n:i64, a:usize, b:usize) -> Self {
        State { pctr:self.pctr+n, reg_a:a, reg_b:b }
    }

    fn exec(&self, opcode: &Opcode) -> Self {
        match opcode {
            Opcode::HalfA =>
                self.next(1, self.reg_a/2, self.reg_b),
            Opcode::HalfB =>
                self.next(1, self.reg_a, self.reg_b/2),
            Opcode::TripleA =>
                self.next(1, self.reg_a*3, self.reg_b),
            Opcode::TripleB =>
                self.next(1, self.reg_a, self.reg_b*3),
            Opcode::IncrA =>
                self.next(1, self.reg_a+1, self.reg_b),
            Opcode::IncrB =>
                self.next(1, self.reg_a, self.reg_b+1),
            Opcode::Jump(n) =>
                self.next(*n, self.reg_a, self.reg_b),
            Opcode::JumpEvenA(n) =>
                self.next(if self.reg_a % 2 == 0 {*n} else {1}, self.reg_a, self.reg_b),
            Opcode::JumpEvenB(n) =>
                self.next(if self.reg_b % 2 == 0 {*n} else {1}, self.reg_a, self.reg_b),
            Opcode::JumpOneA(n) =>
                self.next(if self.reg_a == 1 {*n} else {1}, self.reg_a, self.reg_b),
            Opcode::JumpOneB(n) =>
                self.next(if self.reg_b == 1 {*n} else {1}, self.reg_a, self.reg_b),
        }
    }
}

struct Program {
    prog: Vec<Opcode>,
    state: State,
}

impl Program {
    fn new(input: &str) -> Self {
        let prog = input.trim().lines().map(Opcode::new).collect();
        Program { prog: prog, state: State::new() }
    }

    fn next(&self) -> Option<State> {
        if self.state.pctr < 0 {
            None
        } else if let Some(op) = self.prog.get(self.state.pctr as usize) {
            Some(self.state.exec(op))
        } else {
            None
        }
    }

    fn run(&mut self) {
        while let Some(st) = self.next() {
            self.state = st;
        }
    }
}

fn part1(input: &str) -> usize {
    let mut prog = Program::new(input);
    prog.run();
    return prog.state.reg_b;
}

fn part2(input: &str) -> usize {
    let mut prog = Program::new(input);
    prog.state.reg_a = 1;
    prog.run();
    return prog.state.reg_b;
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 23).unwrap();

    // Unit tests based on the provided examples:
    let test = "inc b\n jio b, +2\n tpl b\n inc b";
    assert_eq!(part1(test), 2);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
