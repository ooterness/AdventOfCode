/// Day 12: https://adventofcode.com/2020/day/12
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

enum Command {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
    Error,
}

impl Command {
    fn parse(cmd:&str) -> Command {
        if let Ok(amt) = cmd[1..].parse() {
            match &cmd[0..1] {
                "N" => Command::North(amt),
                "S" => Command::South(amt),
                "E" => Command::East(amt),
                "W" => Command::West(amt),
                "L" => Command::Left(amt),
                "R" => Command::Right(amt),
                "F" => Command::Forward(amt),
                _   => Command::Error,
            }
        } else {Command::Error}
    }
}

struct SimpleShip {
    x: i64,     // + = East
    y: i64,     // + = North
    d: i64,     // 0 = North, 90 = East, etc.
}

impl SimpleShip {
    fn new() -> SimpleShip {
        SimpleShip {x:0, y:0, d:90}
    }

    fn get_facing(&self, amt:i64) -> Command {
        match self.d.rem_euclid(360) {
            0   => Command::North(amt),
            90  => Command::East(amt),
            180 => Command::South(amt),
            270 => Command::West(amt),
            _   => Command::Error,
        }
    }

    fn move_one(&mut self, cmd:&Command) -> bool {
        match cmd {
            Command::North(amt)     => {self.y += amt; true},
            Command::South(amt)     => {self.y -= amt; true},
            Command::East(amt)      => {self.x += amt; true},
            Command::West(amt)      => {self.x -= amt; true},
            Command::Left(amt)      => {self.d -= amt; true},
            Command::Right(amt)     => {self.d += amt; true},
            Command::Forward(amt)   => self.move_one(&self.get_facing(*amt)),
            _                       => {eprintln!("Bad command"); false},
        }
    }

    fn move_all(&mut self, cmd:&[Command]) -> bool {
        cmd.iter().all(|c| self.move_one(c))
    }

    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

struct WaypointShip {
    sx: i64,    // Ship position (+ = East)
    sy: i64,    // Ship position (+ = North)
    wx: i64,    // Waypoint (+ = East)
    wy: i64,    // Waypoint (+ = North)
}

impl WaypointShip {
    fn new() -> WaypointShip {
        WaypointShip {sx:0, sy:0, wx:10, wy:1}
    }

    fn rotate(&mut self, amt:i64) -> bool{
        let wx = self.wx;
        let wy = self.wy;
        match amt.rem_euclid(360) {
            90  => {self.wx = wy;
                    self.wy = -wx; true},
            180 => {self.wx = -wx;
                    self.wy = -wy; true},
            270 => {self.wx = -wy;
                    self.wy = wx;  true},
            _   => false,
        }
    }

    fn move_one(&mut self, cmd:&Command) -> bool {
        match cmd {
            Command::North(amt)     => {self.wy += amt; true},
            Command::South(amt)     => {self.wy -= amt; true},
            Command::East(amt)      => {self.wx += amt; true},
            Command::West(amt)      => {self.wx -= amt; true},
            Command::Left(amt)      => self.rotate(-*amt),
            Command::Right(amt)     => self.rotate(*amt),
            Command::Forward(amt)   => {self.sx += amt * self.wx;
                                        self.sy += amt * self.wy; true},
            _                       => {eprintln!("Bad command"); false},
        }
    }

    fn move_all(&mut self, cmd:&[Command]) -> bool {
        cmd.iter().all(|c| self.move_one(c))
    }

    fn manhattan(&self) -> i64 {
        self.sx.abs() + self.sy.abs()
    }
}

fn parse(cmd:&[String]) -> Vec<Command> {
    cmd.iter().map(|c| Command::parse(c)).collect()
}

fn part1(cmd:&[Command]) -> i64 {
    let mut ship = SimpleShip::new();
    ship.move_all(cmd);
    ship.manhattan()
}

fn part2(cmd:&[Command]) -> i64 {
    let mut ship = WaypointShip::new();
    ship.move_all(cmd);
    ship.manhattan()
}

pub fn solve() {
    let example = parse(&vec![
        String::from("F10"),
        String::from("N3"),
        String::from("F7"),
        String::from("R90"),
        String::from("F11")]);
    let input = parse(&common::read_strings("input/input12.txt"));

    assert_eq!(25, part1(&example));
    assert_eq!(286, part2(&example));
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
