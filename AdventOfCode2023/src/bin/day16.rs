/// Advent of Code 2023, Day 16
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Rc(isize, isize);
const DIR_N: Rc = Rc(-1, 0);
const DIR_S: Rc = Rc( 1, 0);
const DIR_W: Rc = Rc(0, -1);
const DIR_E: Rc = Rc(0,  1);

impl Rc {
    fn add(&self, other: &Rc) -> Self {
        Rc(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Beam(Rc, Rc);

impl Beam {
    fn next(&self) -> Self {
        Beam(self.0.add(&self.1), self.1)
    }

    fn ml(&self) -> Self {
        Beam(self.0, Rc(-self.1.1, -self.1.0)).next()
    }

    fn mr(&self) -> Self {
        Beam(self.0, Rc(self.1.1, self.1.0)).next()
    }
}

enum Optic {
    Empty,      // Empty space
    MirrorL,    // Left mirror (slash)
    MirrorR,    // Right mirror (backslash)
    SplitH,     // Horizontal splitter (hyphen)
    SplitV,     // Vertical splitter(pipe)
}

impl Optic {
    fn new(input: char) -> Option<Self> {
        match input {
            '.'  => Some(Optic::Empty),
            '/'  => Some(Optic::MirrorL),
            '\\' => Some(Optic::MirrorR),
            '-'  => Some(Optic::SplitH),
            '|'  => Some(Optic::SplitV),
            _    => None,
        }
    }

    fn next(&self, beam: &Beam) -> Vec<Beam> {
        match (&self, beam.1) {
            (Optic::MirrorL, _)     => vec![beam.ml()],
            (Optic::MirrorR, _)     => vec![beam.mr()],
            (Optic::SplitH, DIR_N)  => vec![beam.ml(), beam.mr()],
            (Optic::SplitH, DIR_S)  => vec![beam.ml(), beam.mr()],
            (Optic::SplitV, DIR_W)  => vec![beam.ml(), beam.mr()],
            (Optic::SplitV, DIR_E)  => vec![beam.ml(), beam.mr()],
            (_, _)                  => vec![beam.next()],
        }
    }
}

struct Grid {
    optics: HashMap<Rc, Optic>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = Grid { optics: HashMap::new() };
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                let rc = Rc(r as isize, c as isize);
                let typ = Optic::new(ch).unwrap();
                grid.optics.insert(rc, typ);
            }
        }
        return grid;
    }

    fn edges(&self) -> Vec<Beam> {
        let rmax = self.optics.keys().map(|rc| rc.0 as isize).max().unwrap();
        let cmax = self.optics.keys().map(|rc| rc.1 as isize).max().unwrap();
        let mut tmp = Vec::new();
        tmp.extend((0..=cmax).map(|c| Beam(Rc(0,    c), DIR_S)));
        tmp.extend((0..=cmax).map(|c| Beam(Rc(rmax, c), DIR_N)));
        tmp.extend((0..=rmax).map(|r| Beam(Rc(r,    0), DIR_E)));
        tmp.extend((0..=rmax).map(|r| Beam(Rc(r, cmax), DIR_W)));
        return tmp;
    }

    fn simulate(&self, start: Beam) -> HashSet<Beam> {
        let mut stack: Vec<Beam> = vec![start];
        let mut visit: HashSet<Beam> = HashSet::new();
        visit.insert(start);
        while let Some(beam) = stack.pop() {
            for next in self.optics[&beam.0].next(&beam).into_iter() {
                if self.optics.contains_key(&next.0) && !visit.contains(&next) {
                    stack.push(next);
                    visit.insert(next);
                }
            }
        }
        return visit;
    }

    fn energized(&self, start: Beam) -> HashSet<Rc> {
        self.simulate(start).iter().map(|b| b.0).collect()
    }
}

fn part1(input: &str) -> usize {
    const BEAM_START: Beam = Beam(Rc(0,0), DIR_E);
    Grid::new(input).energized(BEAM_START).len()
}

fn part2(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.edges().into_iter().map(|b| grid.energized(b).len()).max().unwrap()
}

const EXAMPLE: &'static str = "\
    .|...\\....
    |.-.\\.....
    .....|-...
    ........|.
    ..........
    .........\\
    ..../.\\\\..
    .-.-/..|..
    .|....-|.\\
    ..//.|....";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 16).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 46);
    assert_eq!(part2(EXAMPLE), 51);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
