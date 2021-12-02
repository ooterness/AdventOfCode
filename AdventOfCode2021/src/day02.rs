/// Day 2: https://adventofcode.com/2021/day/2
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

// A command consists of a direction and a magnitude.
enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl Command {
    // Create a command from a string like "forward 5".
    fn from_str(line: &str) -> Option<Command> {
        let mut parts = line.split(' ');
        if let Some(cmd) = parts.next() {
            if let Some(arg) = parts.next() {
                let mag:Option<i64> = arg.parse().ok();
                match (cmd, mag) {
                    ("forward", Some(n)) => Some(Command::Forward(n)),
                    ("up",      Some(n)) => Some(Command::Up(n)),
                    ("down",    Some(n)) => Some(Command::Down(n)),
                    _                    => None,
                }
            } else {None}
        } else {None}
    }
}

// Track a submarine's position.
struct Submarine {
    x: i64,
    d: i64,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {x:0, d:0}
    }

    fn command(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(n) => self.x += n,     // Horizontal motion
            Command::Up(n)      => self.d -= n,     // Decrease depth
            Command::Down(n)    => self.d += n,     // Increase depth
        };
    }

    fn run(&mut self, filename: &str) {
        for line in common::read_lines(filename).iter() {
            if let Some(cmd) = Command::from_str(&line) {
                self.command(&cmd);
            }
        }
    }

    fn score(&self) -> i64 {
        self.x * self.d
    }
}

pub fn solve() {
    // Run the Part-1 example.
    let mut test1 = Submarine::new();
    test1.run("input/test02.txt");
    assert_eq!(test1.x, 15);
    assert_eq!(test1.d, 10);
    assert_eq!(test1.score(), 150);

    // Run the problem input.
    let mut part1 = Submarine::new();
    part1.run("input/input02.txt");
    println!("Part 1: {}", part1.score());
}
