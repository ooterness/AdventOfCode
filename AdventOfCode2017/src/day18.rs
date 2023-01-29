/// Advent of Code 2017, Day 18
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::VecDeque;

enum Opcode {
    Snd(usize),
    Rcv(usize),
    SetI(usize, i64),
    SetR(usize, usize),
    AddI(usize, i64),
    AddR(usize, usize),
    MulI(usize, i64),
    MulR(usize, usize),
    ModI(usize, i64),
    ModR(usize, usize),
    JgzI(usize, i64),
    JgzR(usize, usize),
}

fn parse(line: &str) -> Option<Opcode> {
    let words: Vec<&str> = line.split(' ').collect();
    let op = words[0];
    let aa = words[1].chars().next().unwrap();
    let ar = if aa > 'a' {aa as usize - 'a' as usize} else {0};
    if words.len() < 3 {
        return match op {
            "snd" => Some(Opcode::Snd(ar)),
            "rcv" => Some(Opcode::Rcv(ar)),
            _     => None,
        }
    } else {
        let bb = words[2].chars().next().unwrap();
        let br = if bb > 'a' {bb as usize - 'a' as usize} else {0};
        let bp = words[2].parse::<i64>();
        return match (op, bp) {
            ("set", Ok(bi)) => Some(Opcode::SetI(ar, bi)),
            ("set", _)      => Some(Opcode::SetR(ar, br)),
            ("add", Ok(bi)) => Some(Opcode::AddI(ar, bi)),
            ("add", _)      => Some(Opcode::AddR(ar, br)),
            ("mul", Ok(bi)) => Some(Opcode::MulI(ar, bi)),
            ("mul", _)      => Some(Opcode::MulR(ar, br)),
            ("mod", Ok(bi)) => Some(Opcode::ModI(ar, bi)),
            ("mod", _)      => Some(Opcode::ModR(ar, br)),
            ("jgz", Ok(bi)) => Some(Opcode::JgzI(ar, bi)),
            ("jgz", _)      => Some(Opcode::JgzR(ar, br)),
            _               => None,
        }
    }
}

struct Program {
    prog: Vec<Opcode>,      // Program listing
    regs: Vec<i64>,         // Current register state
    send: VecDeque<i64>,    // Transmit queue
    sent: usize,            // Transmit count
    line: i64,              // Current line number
    chkr: bool,             // Check register before RCV?
}

impl Program {
    // Create a new program from the provided source code.
    fn new(input: &str, pid: i64, chkr: bool) -> Program {
        let prog = input.trim().lines().filter_map(parse).collect();
        let mut regs = vec![0i64; 26];
        regs[15] = pid; // Register 'p' = Index 15
        Program { prog:prog, regs: regs, send:VecDeque::new(), sent: 0, line:0, chkr:chkr }
    }

    // Given the head of the RCV queue, attempt to run one opcode.
    //  Return (true, true) for an RCV opcode.
    //  Return (true, false) for any other opcode.
    //  Return (false, false) if execution is blocked for any reason.
    fn step(&mut self, rcv: Option<&i64>) -> (bool, bool) {
        // Sanity check: Is line number in-bounds?
        if (self.line < 0) || (self.line as usize >= self.prog.len())
            { return (false, false); }
        // Decode the current instruction...
        return match self.prog[self.line as usize] {
            Opcode::Snd(x) => {
                self.send.push_back(self.regs[x]);
                self.sent += 1;
                self.line += 1;
                (true, false)},
            Opcode::Rcv(x) => {
                if self.chkr && self.regs[x] == 0 {
                    self.line += 1; 
                    (false, false)  // No-op (X = 0)
                } else if let Some(r) = rcv {
                    self.line += 1;
                    self.regs[x] = *r;
                    (true, true)    // Receive OK
                } else {
                    (false, false)  // Receive blocked
                } },
            Opcode::SetI(x, y) => {
                self.regs[x] = y;
                self.line += 1;
                (true, false)},
            Opcode::SetR(x, y) => {
                self.regs[x] = self.regs[y];
                self.line += 1;
                (true, false)},
            Opcode::AddI(x, y) => {
                self.regs[x] += y;
                self.line += 1;
                (true, false)},
            Opcode::AddR(x, y) => {
                self.regs[x] += self.regs[y];
                self.line += 1;
                (true, false)},
            Opcode::MulI(x, y) => {
                self.regs[x] *= y;
                self.line += 1;
                (true, false)},
            Opcode::MulR(x, y) => {
                self.regs[x] *= self.regs[y];
                self.line += 1;
                (true, false)},
            Opcode::ModI(x, y) => {
                self.regs[x] = self.regs[x] % y;
                self.line += 1;
                (true, false)},
            Opcode::ModR(x, y) => {
                self.regs[x] = self.regs[x] % self.regs[y];
                self.line += 1;
                (true, false)},
            Opcode::JgzI(x, y) => {
                self.line += if self.regs[x] > 0 {y} else {1};
                (true, false)},
            Opcode::JgzR(x, y) => {
                self.line += if self.regs[x] > 0 {self.regs[y]} else {1};
                (true, false)},
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut prog = Program::new(input, 0, true);
    prog.send.push_back(0);
    loop {
        let (stp, rcv) = prog.step(Some(&0));
        if rcv || !stp {break;} // Run until RCV or halt
    }
    return *prog.send.back().unwrap();
}

fn part2(input: &str) -> usize {
    let mut prog0 = Program::new(input, 0, false);
    let mut prog1 = Program::new(input, 1, false);
    loop {
        let (stp0, rcv0) = prog0.step(prog1.send.front());
        let (stp1, rcv1) = prog1.step(prog0.send.front());
        if rcv0 {prog1.send.pop_front();}
        if rcv1 {prog0.send.pop_front();}
        if !(stp0 || stp1) {break;} // Run until both programs halt
    }
    return prog1.sent
}

const TEST1: &str = "\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

const TEST2: &str = "\
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

fn main() {
    // Fetch problem input from server.
    let input = fetch::get_data(2017, 18).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(TEST1), 4);
    assert_eq!(part2(TEST2), 3);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
