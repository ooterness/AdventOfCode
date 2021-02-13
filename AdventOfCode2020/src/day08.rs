/// Day 8: https://adventofcode.com/2020/day/8
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

enum Opcode {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

struct Program(Vec<Opcode>);
type CompiledProgram = Result<Program, String>;

struct ProgState {
    acc: i64,
    line: i64,
}

impl Opcode {
    /// Parse one line from a larger program.
    fn compile(line: &String) -> Result<Opcode, String> {
        let error = Err(line.clone());
        if let Some((cmd,arg)) = common::split2(line, " ") {
            let arg = arg.parse::<i64>();
            match (cmd, arg) {
                ("nop", Ok(n))  => Ok(Opcode::Nop(n)),
                ("acc", Ok(n))  => Ok(Opcode::Acc(n)),
                ("jmp", Ok(n))  => Ok(Opcode::Jmp(n)),
                _               => error,
            }
        } else {error}
    }

    /// Execute this opcode.
    fn execute(&self, st: &ProgState) -> ProgState {
        match self {
            Opcode::Nop(_) => ProgState {acc: st.acc,   line: st.line+1},
            Opcode::Acc(x) => ProgState {acc: st.acc+x, line: st.line+1},
            Opcode::Jmp(x) => ProgState {acc: st.acc,   line: st.line+x},
        }
    }

    /// Swap Nop to Jmp or vice-versa.
    fn swap(&self) -> Option<Opcode> {
        match self {
            Opcode::Nop(x) if *x != 0 => Some(Opcode::Jmp(*x)),
            Opcode::Jmp(x) => Some(Opcode::Nop(*x)),
            _ => None,
        }
    }
}

impl Clone for Opcode {
    fn clone(&self) -> Opcode {
        match self {
            Opcode::Nop(x) => Opcode::Nop(*x),
            Opcode::Acc(x) => Opcode::Acc(*x),
            Opcode::Jmp(x) => Opcode::Jmp(*x),
        }
    }
}

impl Program {
    /// Compile a vector of strings into a program.
    fn compile(lines: &Vec<String>) -> CompiledProgram {
        let mut prog:Vec<Opcode> = vec![];
        for line in lines {
            match Opcode::compile(line) {
                Ok(x) => prog.push(x),
                Err(x) => return Err(x),
            }
        }
        Ok(Program(prog))
    }

    /// Mutate program if possible.
    fn mutate(&self, line:usize) -> Option<Program> {
        // Can we swap the given line?
        if let Some(newop) = self.0[line].swap() {   
            let mut copy = Program(self.0.clone());
            copy.0[line] = newop;
            Some(copy)  // Return mutated program
        } else {
            None        // No modification possible
        }
    }

    /// Try all possible mutations until we terminate normally.
    fn mutate_all(&self) -> Option<i64> {
        // Try mutating each line...
        for n in 0..self.0.len() {
            if let Some(prog) = self.mutate(n) {
                // Run program. If it reaches last line, success!
                let result = prog.run_until_repeat();
                if result.line == self.0.len() as i64 {
                    return Some(result.acc);
                }
            }
        }
        None
    }

    /// Run a program until it reaches the end or repeats itself.
    fn run_until_repeat(&self) -> ProgState {
        // Keep running until we jump out of bounds or to
        // an instruction we've already executed.
        let lmax = self.0.len() as i64;
        let mut seen = vec![false; self.0.len()];
        let mut st   = ProgState {acc:0, line:0};
        while 0 <= st.line && st.line < lmax {
            let line = st.line as usize;
            if seen[line] {break}
            seen[line] = true;
            st = self.0[line].execute(&st)
        }
        return st
    }
}

/// Print Part1 solution (run program until repeat)
fn print_part1(lbl: &str, prog: &CompiledProgram) {
    match prog {
        Ok(prog) => println!("{}: Result = {}", lbl, prog.run_until_repeat().acc),
        Err(msg) => eprintln!("{}: Compile error @ {}", lbl, msg),
    }
}

/// Print Part1 solution (modify program to terminate normally)
fn print_part2(lbl: &str, prog: &CompiledProgram) {
    // Did we compile success
    match prog {
        Ok(prog) => {
            if let Some(n) = prog.mutate_all() {
                println!("{}: Result = {}", lbl, n)
            } else {
                eprintln!("{}: No mutation successful.", lbl)
            }
        },
        Err(msg) => eprintln!("{}: Compile error @ {}", lbl, msg),
    }
}

pub fn solve() {
    let example = vec![
        String::from("nop +0"),
        String::from("acc +1"),
        String::from("jmp +4"),
        String::from("acc +3"),
        String::from("jmp -3"),
        String::from("acc -99"),
        String::from("acc +1"),
        String::from("jmp -4"),
        String::from("acc +6"),
    ];

    // Compile and run the example program.
    let test1 = Program::compile(&example);
    print_part1("Test1", &test1);
    print_part2("Test2", &test1);

    // Compile and run the Part1 program.
    let input = common::read_strings("input/input08.txt");
    let prog1 = Program::compile(&input);
    print_part1("Part1", &prog1);
    print_part2("Part2", &prog1);
}
