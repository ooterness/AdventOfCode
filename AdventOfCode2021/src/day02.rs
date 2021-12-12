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
    x: i64,     // Horizontal position
    d: i64,     // Current depth
    a: i64,     // Current "aim"
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {x:0, d:0, a:0}
    }

    fn command1(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(n) => self.x += n,     // Horizontal motion
            Command::Up(n)      => self.d -= n,     // Decrease depth
            Command::Down(n)    => self.d += n,     // Increase depth
        };
    }

    fn command2(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(n) =>                  // Forward motion
                {self.x += n; self.d += self.a * n},
            Command::Up(n)      => self.a -= n,     // Decrease aim
            Command::Down(n)    => self.a += n,     // Increase aim
        };
    }

    fn run(filename: &str, part1: bool) -> Submarine {
        let mut sub = Submarine::new();
        for line in common::read_lines(filename).iter() {
            if let Some(cmd) = Command::from_str(&line) {
                if part1 {sub.command1(&cmd)}
                else     {sub.command2(&cmd)};
            }
        }
        sub
    }

    fn score(&self) -> i64 {
        self.x * self.d
    }
}

pub fn solve() {
    // Run the short example in each mode.
    let test1 = Submarine::run("input/test02.txt", true);
    assert_eq!(test1.x, 15);
    assert_eq!(test1.d, 10);
    assert_eq!(test1.score(), 150);
    let test2 = Submarine::run("input/test02.txt", false);
    assert_eq!(test2.x, 15);
    assert_eq!(test2.d, 60);
    assert_eq!(test2.score(), 900);

    // Run the problem input in each mode.
    let part1 = Submarine::run("input/input02.txt", true);
    println!("Part1: {}", part1.score());
    let part2 = Submarine::run("input/input02.txt", false);
    println!("Part2: {}", part2.score());
}
