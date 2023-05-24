/// Advent of Code 2016, Day 21
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

type VChar = Vec<char>;

enum Step {
    SwapP(usize, usize),    // Swap letters by index
    SwapL(char, char),      // Swap letters by value
    RotateIL(usize),        // Rotate left by index
    RotateIR(usize),        // Rotate right by index
    RotatePL(char),         // Rotate left by position
    RotatePR(char),         // Rotate right by position
    Reverse(usize, usize),  // Reverse substring by index
    Move(usize, usize),     // Move letter by index
}

fn rotate(input: &VChar, shift: usize, left: bool) -> VChar {
    let mut tmp = input.clone();
    if left {   // Rotate left
        for n in 0..input.len() {
            tmp[n] = input[(n+shift)%input.len()];
        }
    } else {    // Rotate right
        for n in 0..input.len() {
            tmp[(n+shift)%input.len()] = input[n];
        }
    }
    return tmp;
}

impl Step {
    fn new(line: &str) -> Option<Self> {
        let tokens: Vec<&str> = line.trim().split(' ').collect();
        match (tokens[0], tokens[1]) {
            ("swap", "position") => {
                let a: usize = tokens[2].parse().unwrap();
                let b: usize = tokens[5].parse().unwrap();
                Some(Step::SwapP(a, b))},
            ("swap", "letter") => {
                let a: char = tokens[2].chars().nth(0).unwrap();
                let b: char = tokens[5].chars().nth(0).unwrap();
                Some(Step::SwapL(a, b))},
            ("rotate", "left") => {
                let a: usize = tokens[2].parse().unwrap();
                Some(Step::RotateIL(a))},
            ("rotate", "right") => {
                let a: usize = tokens[2].parse().unwrap();
                Some(Step::RotateIR(a))},
            ("rotate", "based") => {
                let a: char = tokens[6].chars().nth(0).unwrap();
                Some(Step::RotatePR(a))},
            ("reverse", _) => {
                let a: usize = tokens[2].parse().unwrap();
                let b: usize = tokens[4].parse().unwrap();
                Some(Step::Reverse(a, b))},
            ("move", _) => {
                let a: usize = tokens[2].parse().unwrap();
                let b: usize = tokens[5].parse().unwrap();
                Some(Step::Move(a, b))},
            _ => None,
        }
    }

    fn apply(&self, input: &VChar) -> VChar {
        let mut tmp = input.clone();
        match self {
            Step::SwapP(a, b) => {      // Swap letters by index
                tmp[*a] = input[*b];
                tmp[*b] = input[*a];},
            Step::SwapL(a, b) => {      // Swap letters by value
                for n in 0..input.len() {
                    if input[n] == *a {tmp[n] = *b;}
                    if input[n] == *b {tmp[n] = *a;}
                }},
            Step::RotateIL(a) => {      // Rotate left by index
                tmp = rotate(input, *a, true);},
            Step::RotateIR(a) => {      // Rotate right by index
                tmp = rotate(input, *a, false);},
            Step::RotatePL(a) => {      // Rotate left by position
                // Unique solution is not guaranteed -> guess and check.
                for n in (0..input.len()).rev() {
                    let shift = 1 + n + (n >= 4) as usize;
                    tmp = rotate(input, shift, true);
                    if tmp[n] == *a {break;}
                }},
            Step::RotatePR(a) => {      // Rotate right by position
                let mut m = 1 + input.iter().position(|c| c==a).unwrap();
                if m >= 5 {m += 1;}
                for n in 0..input.len() {
                    tmp[(n+m)%input.len()] = input[n];
                }},
            Step::Reverse(a, b) => {    // Reverse substring by index
                let len = 1 + b - a;
                for n in 0..len {
                    tmp[n+a] = input[b-n];
                }},
            Step::Move(a, b) => {       // Move letter by index
                let c = tmp.remove(*a);
                tmp.insert(*b, c);},
        };
        return tmp;
    }

    fn invert(&self) -> Self {
        match self {
            Step::SwapP(a, b) => Step::SwapP(*a, *b),
            Step::SwapL(a, b) => Step::SwapL(*a, *b),
            Step::RotateIL(a) => Step::RotateIR(*a),
            Step::RotateIR(a) => Step::RotateIL(*a),
            Step::RotatePL(a) => Step::RotatePR(*a),
            Step::RotatePR(a) => Step::RotatePL(*a),
            Step::Reverse(a, b) => Step::Reverse(*a, *b),
            Step::Move(a, b) => Step::Move(*b, *a),
        }
    }
}

fn scramble(steps: &str, password: &str) -> String {
    let mut result: VChar = password.chars().collect();
    for line in steps.trim().lines() {
        let step = Step::new(line).unwrap();
        result = step.apply(&result);
    }
    return result.iter().collect();
}

fn unscramble(steps: &str, password: &str) -> String {
    let mut result: VChar = password.chars().collect();
    for line in steps.trim().lines().rev() {
        let step = Step::new(line).unwrap().invert();
        result = step.apply(&result);
    }
    return result.iter().collect();
}

fn part1(input: &str) -> String {
    scramble(input, "abcdefgh")
}

fn part2(input: &str) -> String {
    unscramble(input, "fbgdceah")
}

const TEST: &str = "\
    swap position 4 with position 0
    swap letter d with letter b
    reverse positions 0 through 4
    rotate left 1 step
    move position 1 to position 4
    move position 3 to position 0
    rotate based on position of letter b
    rotate based on position of letter d";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 21).unwrap();

    // Unit tests on provided examples
    assert_eq!(scramble(TEST, "abcde"), "decab");
    assert_eq!(unscramble(TEST, "decab"), "abcde");

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
