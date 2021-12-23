/// Day 22: https://adventofcode.com/2021/day/22
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

const VERBOSE:bool = true;

// Given 4 elements, create a sorted de-duplicated set.
fn sortify(a:i64, b:i64, c:i64, d:i64) -> Vec<i64> {
    let mut abcd = vec![a,b,c,d];
    abcd.sort();    // Sort the vector
    abcd.dedup();   // Remove consecutive duplicates
    abcd
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Xyz {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Cube {
    a: Xyz,
    b: Xyz,
}

type Cubes = Vec<Cube>;

impl Cube {
    // Is a point inside this cuboid?
    fn contains(&self, p:&Xyz) -> bool {
        self.a.x <= p.x && p.x < self.b.x &&
        self.a.y <= p.y && p.y < self.b.y &&
        self.a.z <= p.z && p.z < self.b.z
    }

    // Volume of this cuboid.
    fn volume(&self) -> u64 {
        let dx = (self.b.x - self.a.x) as u64;
        let dy = (self.b.y - self.a.y) as u64;
        let dz = (self.b.z - self.a.z) as u64;
        dx * dy * dz
    }

    // Subtract a cuboid from the current volume to several smaller volumes.
    fn sub(&self, other:&Cube) -> Cubes {
        // Quickly check for the no-overlap case.
        if (self.b.x < other.a.x) || (self.b.y < other.a.y) || (self.b.z < other.a.z) ||
           (other.b.x < self.a.x) || (other.b.y < self.a.y) || (other.b.z < self.a.z) {
            return vec![self.clone()];
        }
        // Find the set of ordered unique bounding coordinates.
        // This forms a set of up to 27 possible sub-volumes.
        let xx = sortify(self.a.x, self.b.x, other.a.x, other.b.x);
        let yy = sortify(self.a.y, self.b.y, other.a.y, other.b.y);
        let zz = sortify(self.a.z, self.b.z, other.a.z, other.b.z);
        // Test a point in each sub-volume to see if we should keep it.
        let mut result = Vec::new();
        for nx in 1..xx.len() {
            for ny in 1..yy.len() {
                for nz in 1..zz.len() {
                    let p0 = Xyz { x:xx[nx-1], y:yy[ny-1], z:zz[nz-1] };
                    if self.contains(&p0) && !other.contains(&p0) {
                        let p1 = Xyz { x:xx[nx], y:yy[ny], z:zz[nz] };
                        result.push(Cube {a:p0, b:p1} )
                    }
                }
            }
        }
        return result
    }
}

fn sub(xx:&Cubes, y:&Cube) -> Cubes {
    let mut result = Vec::new();
    for x in xx {
        result.append(&mut x.sub(y));
    }
    return result
}

struct Command {
    range: Cube,
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
        // Increment upper coordinates by one to simplify edge cases.
        assert_eq!(vv.len(), 9);
        assert!(vv[0] <= vv[2] && vv[3] <= vv[5] && vv[6] <= vv[8]);
        let v0 = Xyz {x:vv[0]+0, y:vv[3]+0, z:vv[6]+0};
        let v1 = Xyz {x:vv[2]+1, y:vv[5]+1, z:vv[8]+1};
        Command { range:Cube{a:v0, b:v1}, value:bb }
    }

    // Apply this command to generate a new set of cubes.
    fn apply(&self, cubes:&Cubes) -> Cubes {
        if self.value {
            // "On" command: Add unique part(s) of this cube.
            // (i.e., Subtract all other cubes to find the new bits.)
            let mut result = vec![self.range];
            for cube in cubes {
                result = sub(&result, cube);
            }
            result.append(&mut cubes.clone()); result
        } else {
            // "Off" command: Subtract this cube from all others.
            sub(cubes, &self.range)
        }
    }
}

fn run(cmds: &Vec<Command>) -> Cubes {
    // Starting from the empty set, apply each command.
    let mut cubes = Vec::new();
    for (n,cmd) in cmds.iter().enumerate() {
        cubes = cmd.apply(&cubes);
        if VERBOSE {eprintln!("Step {}: {} / {}", n, cubes.len(), part2(&cubes));}
    }
    return cubes
}

fn read_commands(filename: &str) -> Vec<Command>
{
    let lines = common::read_lines(filename);
    lines.iter().map(|l| Command::new(l)).collect()
}

// Find total volume in the designated bounding box.
fn part1(x: &Cubes) -> u64 {
    let bound = Cube { a:Xyz {x:-50, y:-50, z:-50} ,
                       b:Xyz {x: 51, y: 51, z: 51} };
    let y = sub(x, &bound); // Subtract the bounding box
    part2(x) - part2(&y)    // Difference is the inner set
}

// Find total volume of all cubes.
fn part2(x: &Cubes) -> u64 {
    x.iter().map(|c| c.volume()).sum()
}

pub fn solve() {
    let test1 = run(&read_commands("input/test22a.txt"));
    let test2 = run(&read_commands("input/test22b.txt"));
    let test3 = run(&read_commands("input/test22c.txt"));
    let input = run(&read_commands("input/input22.txt"));

    assert_eq!(part1(&test1), 39);
    assert_eq!(part1(&test2), 590784);
    assert_eq!(part1(&test3), 474140);
    assert_eq!(part2(&test3), 2758514936282235);
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
