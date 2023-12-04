/// Advent of Code 2017, Day 25
/// Copyright 2023 by Alex Utter

extern crate aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

type Tape = HashSet<i64>;

struct Action {
    write: bool,    // Write 1 or 0?
    right: bool,    // Move left or right?
    state: char,    // Label for next state?
}

impl Action {
    fn new(input: &[&str]) -> Action
    {
        // Parse the parameter we need from each line.
        assert_eq!(input.len(), 4);
        let write = input[1].chars().rev().nth(1).unwrap() == '1';
        let right = input[2].chars().rev().nth(2).unwrap() == 'h';
        let state = input[3].chars().rev().nth(1).unwrap();
        Action { write, right, state }
    }

    fn apply(&self, posn: &mut i64, tape: &mut Tape) -> char {
        if self.write {     // Write a '1' or a '0'?
            tape.insert(posn.clone());
        } else {
            tape.remove(posn);
        }
        *posn += if self.right {1} else {-1};
        return self.state;
    }
}

struct State {
    label: char,
    action0: Action,
    action1: Action,
}

impl State {
    fn new(input: &[&str]) -> State
    {
        // Read the label, then both actions.
        assert_eq!(input.len(), 9);
        let label = input[0].chars().rev().nth(1).unwrap();
        let action0 = Action::new(&input[1..5]);
        let action1 = Action::new(&input[5..9]);
        State { label, action0, action1 }
    }

    fn apply(&self, posn: &mut i64, tape: &mut Tape) -> char {
        if tape.contains(posn) {
            self.action1.apply(posn, tape)
        } else {
            self.action0.apply(posn, tape)
        }
    }
}

struct Turing {
    init: char,
    steps: usize,
    states: HashMap<char, State>,
}

impl Turing {
    fn new(input: &str) -> Turing {
        // Split by lines.  Header is 2 lines, then 10 per state.
        let lines: Vec<&str> = input.lines().collect();
        let num_states = (lines.len() - 2) / 10;
        let init: char = lines[0].chars().rev().nth(1).unwrap();
        let steps: usize = lines[1].split(' ')
            .nth(5).unwrap()        // Get the Nth word
            .parse().unwrap();      // Parse as an integer
        let states = (0..num_states)
            .map(|n| State::new(&lines[10*n+3 .. 10*n+12]))
            .map(|s| (s.label, s))  // Key + Value
            .collect();             // Construct dictionary
        Turing { init, steps, states }
    }

    fn run(&self) -> Tape {
        let mut state = self.init;
        let mut posn = 0i64;
        let mut tape = Tape::new();
        for _ in 0..self.steps {
            state = self.states[&state].apply(&mut posn, &mut tape);
        }
        return tape;
    }
}

fn part1(input: &str) -> usize {
    let program = Turing::new(input);
    program.run().len() // Count the number of '1's
}

const TEST: &str = "\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 25).unwrap();

    // Unit tests based on the provided example.
    assert_eq!(part1(TEST), 3);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
}
