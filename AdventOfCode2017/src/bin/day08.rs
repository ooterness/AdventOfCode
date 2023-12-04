/// Advent of Code 2017, Day 8
/// Copyright 2023 by Alex Utter

extern crate aocfetch;
use std::collections::HashMap;

enum Compare {
    Gt(i64),    // >  Greater than
    Lt(i64),    // <  Less than
    Eq(i64),    // == Equal
    Neq(i64),   // != Not equal
}

struct Instruction {
    reg_mod: String,
    offset:  i64,
    reg_cmp: String,
    compare: Compare,
}

struct Program {
    instructions: Vec<Instruction>,
}

type Registers = HashMap<String, i64>;

impl Compare {
    fn new(op: &str, val: &str) -> Option<Compare> {
        match (op, val.parse::<i64>()) {
            (">",  Ok(x)) => Some(Compare::Gt(x)),
            ("<",  Ok(x)) => Some(Compare::Lt(x)),
            ("==", Ok(x)) => Some(Compare::Eq(x)),
            (">=", Ok(x)) => Some(Compare::Gt(x-1)),
            ("<=", Ok(x)) => Some(Compare::Lt(x+1)),
            ("!=", Ok(x)) => Some(Compare::Neq(x)),
            _             => None,
        }
    }

    fn cmp(&self, x: &i64) -> bool {
        match self {
            Compare::Gt(y)  => x > y,
            Compare::Lt(y)  => x < y,
            Compare::Eq(y)  => x == y,
            Compare::Neq(y) => x != y,
        }
    }
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        // Example: "b inc 5 if a > 1"
        let words: Vec<&str> = line.split(' ').collect();
        assert_eq!(words.len(), 7);
        let reg_mod = words[0].to_string();
        let sign    = if words[1] == "inc" {1} else {-1};
        let offset  = words[2].parse::<i64>().unwrap() * sign;
        let reg_cmp = words[4].to_string();
        let compare = Compare::new(&words[5], &words[6]).unwrap();
        return Instruction { reg_mod, offset, reg_cmp, compare };
    }

    fn exec(&self, regs: &mut Registers) {
        let val_mod = regs.get(&self.reg_mod).unwrap_or(&0);
        let val_cmp = regs.get(&self.reg_cmp).unwrap_or(&0);
        if self.compare.cmp(val_cmp) {
            regs.insert(self.reg_mod.clone(), val_mod + self.offset);
        }
    }
}

impl Program {
    fn new(input: &str) -> Program {
        let instructions = input.trim().lines().map(Instruction::new).collect();
        return Program { instructions };
    }
}

fn part1(prog: &Program) -> i64
{
    let mut regs = Registers::new();
    for x in prog.instructions.iter() {
        x.exec(&mut regs);
    }
    return *regs.values().max().unwrap_or(&0);
}

fn part2(prog: &Program) -> i64
{
    let mut vmax = 0i64;
    let mut regs = Registers::new();
    for x in prog.instructions.iter() {
        x.exec(&mut regs);
        let rmax = *regs.values().max().unwrap_or(&0);
        vmax = std::cmp::max(vmax, rmax);
    }
    return vmax;
}

const TEST: &str = "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

fn main() {
    // Fetch problem input from server.
    let test  = Program::new(&TEST);
    let input = Program::new(&aocfetch::get_data(2017, 8).unwrap());

    // Unit tests on provided example.
    assert_eq!(part1(&test), 1);
    assert_eq!(part2(&test), 10);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
