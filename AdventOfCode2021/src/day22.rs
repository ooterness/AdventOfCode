/// Day 22: https://adventofcode.com/2021/day/22
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

fn clamp(n:i64) -> i64 {
    if n < -50 {-50} else if n > 50 {50} else {n}
}

struct Xyz {
    x: i64,
    y: i64,
    z: i64,
}

impl Xyz {
    fn clamp(&self) -> Xyz {
        Xyz { x:clamp(self.x), y:clamp(self.y), z:clamp(self.z) }
    }

    fn to_idx(&self) -> usize {
        let x = (self.clamp().x + 50) as usize;
        let y = (self.clamp().y + 50) as usize;
        let z = (self.clamp().z + 50) as usize;
        x + 101*(y + 101*z)
    }
}

struct Command {
    range: (Xyz, Xyz),
    value: bool,
}

impl Command {
    // Parse a line such as "on x=10..12,y=10..12,z=10..12"
    fn new(line: &str) -> Command {
        // Parse the string into six integers.
        let mut bb: bool = false;           // Value (on/off)
        let mut ss: i64 = 1;                // Sign of tt
        let mut tt: i64 = 0;                // Temporary integer
        let mut vv: Vec<i64> = Vec::new();  // List of integers
        for (n,ch) in line.chars().enumerate() {
            if n == 1 && ch == 'n' {bb = true;} // "on" or "off"
            if let Some(v) = ch.to_digit(10) {tt = 10*tt + v as i64;}
            if ch == '-' {ss = -1;}
            if ch == '.' || ch == ',' {vv.push(ss*tt); ss = 1; tt = 0;}
        }
        vv.push(ss*tt);
        // Convert those integers to a min/max range.
        assert_eq!(vv.len(), 9);
        assert!(vv[0] <= vv[2] && vv[3] <= vv[5] && vv[6] <= vv[8]);
        let v0 = Xyz {x:vv[0], y:vv[3], z:vv[6]};
        let v1 = Xyz {x:vv[2], y:vv[5], z:vv[8]};
        Command { value:bb, range:(v0,v1) }
    }

    fn outside(&self) -> bool {
        (self.range.0.x < -50 && self.range.1.x < -50) ||
        (self.range.0.y < -50 && self.range.1.y < -50) ||
        (self.range.0.z < -50 && self.range.1.z < -50) ||
        (self.range.0.x > 50 && self.range.1.x > 50) ||
        (self.range.0.y > 50 && self.range.1.y > 50) ||
        (self.range.0.z > 50 && self.range.1.z > 50)
    }
}

const NCUBES:usize = 101*101*101;

struct Cubes {
    cubes: Vec<bool>,
}

impl Cubes {
    fn new() -> Cubes {
        Cubes { cubes:vec![false;NCUBES] }
    }

    fn apply(&mut self, cmd: &Command) {
        if cmd.outside() {return;}
        let v0 = cmd.range.0.clamp();
        let v1 = cmd.range.1.clamp();
        for x in v0.x..v1.x+1 {
            for y in v0.y..v1.y+1 {
                for z in v0.z..v1.z+1 {
                    let vv = Xyz {x:x, y:y, z:z};
                    self.cubes[vv.to_idx()] = cmd.value;
                }
            }
        }
    }

    fn count(&self) -> usize {
        self.cubes.iter().filter(|&b| *b).count()
    }
}

fn read_commands(filename: &str) -> Vec<Command>
{
    let lines = common::read_lines(filename);
    lines.iter().map(|l| Command::new(l)).collect()
}

fn part1(cmds: &Vec<Command>) -> usize {
    let mut cubes = Cubes::new();
    for cmd in cmds.iter() {cubes.apply(cmd);}
    cubes.count()
}

pub fn solve() {
    let test1 = read_commands("input/test22a.txt");
    let test2 = read_commands("input/test22b.txt");
    let input = read_commands("input/input22.txt");

    assert_eq!(part1(&test1), 39);
    assert_eq!(part1(&test2), 590784);
    println!("Part1: {}", part1(&input));
}
