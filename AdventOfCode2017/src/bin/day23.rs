/// Advent of Code 2017, Day 23
/// Copyright 2023 by Alex Utter

extern crate aocfetch;

enum Opcode {
    SetI(usize, i64),
    SetR(usize, usize),
    SubI(usize, i64),
    SubR(usize, usize),
    MulI(usize, i64),
    MulR(usize, usize),
    IsPrime(usize, usize),
    JnzI(usize, i64),
    JnzR(usize, usize),
    Goto(i64),
}

fn parse(line: &str) -> Option<Opcode> {
    let words: Vec<&str> = line.split(' ').collect();
    let op = words[0];
    let aa = words[1].chars().next().unwrap();
    let ar = if aa > 'a' {aa as usize - 'a' as usize} else {0};
    let ap = words[1].parse::<i64>();
    let bb = words[2].chars().next().unwrap();
    let br = if bb > 'a' {bb as usize - 'a' as usize} else {0};
    let bp = words[2].parse::<i64>();
    if let Ok(ai) = ap {    // Special case for JNZ [int] [int]
        let rel = if ai != 0 {bp.unwrap()} else {1};
        return Some(Opcode::Goto(rel));
    } else {                // All other opcodes...
        return match (op, bp) {
            ("set", Ok(bi)) => Some(Opcode::SetI(ar, bi)),
            ("set", _)      => Some(Opcode::SetR(ar, br)),
            ("sub", Ok(bi)) => Some(Opcode::SubI(ar, bi)),
            ("sub", _)      => Some(Opcode::SubR(ar, br)),
            ("mul", Ok(bi)) => Some(Opcode::MulI(ar, bi)),
            ("mul", _)      => Some(Opcode::MulR(ar, br)),
            ("jnz", Ok(bi)) => Some(Opcode::JnzI(ar, bi)),
            ("jnz", _)      => Some(Opcode::JnzR(ar, br)),
            _               => None,
        }
    }
}

// Test if a given number is prime.
fn is_prime(x: i64) -> bool {
    let mut y = 2i64;
    while y * y <= x {
        if x % y == 0 { return false; }
        y += 1;
    }
    return true;
}

struct Program {
    prog: Vec<Opcode>,      // Program listing
    regs: Vec<i64>,         // Current register state
    line: i64,              // Current line number
    count_mul: usize,       // Count MUL instructions
}

impl Program {
    // Create a new program from the provided source code.
    fn new(input: &str) -> Program {
        let prog = input.trim().lines().filter_map(parse).collect();
        let regs = vec![0i64; 8];
        Program { prog:prog, regs: regs, line:0, count_mul:0 }
    }

    // Attempt to run one opcode, return true if successful.
    fn step(&mut self) -> bool {
        // Sanity check: Is line number in-bounds?
        if (self.line < 0) || (self.line as usize >= self.prog.len())
            { return false; }
        // Decode the current instruction...
        self.line += match self.prog[self.line as usize] {
            Opcode::SetI(x, y) => { self.regs[x] = y; 1 }
            Opcode::SetR(x, y) => { self.regs[x] = self.regs[y]; 1 }
            Opcode::SubI(x, y) => { self.regs[x] -= y; 1 }
            Opcode::SubR(x, y) => { self.regs[x] -= self.regs[y]; 1 }
            Opcode::MulI(x, y) => {
                self.regs[x] *= y;
                self.count_mul += 1; 1 },
            Opcode::MulR(x, y) => {
                self.regs[x] *= self.regs[y];
                self.count_mul += 1; 1 },
            Opcode::IsPrime(x, y) => {
                self.regs[x] = if is_prime(self.regs[y]) {1} else {0}; 1 },
            Opcode::JnzI(x, y) => if self.regs[x] != 0 {y} else {1},
            Opcode::JnzR(x, y) => if self.regs[x] != 0 {self.regs[y]} else {1},
            Opcode::Goto(y) =>    y,
        };
        return true;
    }
}

fn part1(input: &str) -> usize {
    let mut prog = Program::new(input);
    while prog.step() {}    // Run until halt
    return prog.count_mul;
}

fn part2(input: &str) -> i64 {
    let mut prog = Program::new(input);
    prog.regs[0] = 1;       // Toggle debug flag
    // Replace inner loop with the "is_prime" function.
    // TODO: This is hand-optimized for my input, not generalized.
    prog.prog[8] = Opcode::IsPrime(5, 1);       // F = Is B prime?
    prog.prog[9] = Opcode::Goto(15);            // Skip inner loop
    while prog.step() {}    // Run until halt
    return prog.regs[7];
}

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 23).unwrap();

    // Unit tests for the is_prime function.
    assert_eq!(is_prime(17), true);
    assert_eq!(is_prime(18), false);
    assert_eq!(is_prime(19), true);
    assert_eq!(is_prime(34), false);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
