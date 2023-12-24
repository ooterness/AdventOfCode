/// Advent of Code 2023, Day 24
/// Copyright 2023 by Alex Utter

use aocfetch;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Xyz<T>(T, T, T);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Hail {
    pos: Xyz<i64>,
    vel: Xyz<i64>,
}

impl Hail {
    fn new(input: &str) -> Self {
        let tok: Vec<i64> = input.split(&[',', '@'])
            .map(|s| s.trim().parse::<i64>().unwrap()).collect();
        Hail {
            pos: Xyz(tok[0], tok[1], tok[2]),
            vel: Xyz(tok[3], tok[4], tok[5]),
        }
    }

    fn cross_xy(&self, other: &Hail) -> Option<f64> {
        let dp0 = other.vel.0 * (self.pos.1 - other.pos.1)
                - other.vel.1 * (self.pos.0 - other.pos.0);
        let dp1 = self.vel.0 * (other.pos.1 - self.pos.1)
                - self.vel.1 * (other.pos.0 - self.pos.0);
        let dv = self.vel.0 * other.vel.1 - self.vel.1 * other.vel.0;
        if dv != 0 {
            let t0 = dp0 as f64 / dv as f64;
            let t1 = dp1 as f64 / dv as f64;
            if t0 > 0.0 && t1 < 0.0 {return Some(t0)};
        }
        return None;
    }

    fn predict(&self, t: f64) -> Xyz<f64> {
        Xyz(self.pos.0 as f64 + t * self.vel.0 as f64,
            self.pos.1 as f64 + t * self.vel.1 as f64,
            self.pos.2 as f64 + t * self.vel.2 as f64)
    }
}

struct HailStorm {
    hail: Vec<Hail>,
    xmin: i64,
    xmax: i64,
}

impl HailStorm {
    fn new(input:&str, xmin:i64, xmax:i64) -> Self {
        let hail = input.trim().lines().map(Hail::new).collect();
        HailStorm { hail:hail, xmin:xmin, xmax:xmax }
    }

    fn part1(&self) -> usize {
        let mut count = 0usize;
        for m in 0..self.hail.len()-1 {
            for n in m+1..self.hail.len() {
                if let Some(t) = self.hail[m].cross_xy(&self.hail[n]) {
                    let pos = self.hail[m].predict(t);
                    if self.xmin as f64 <= pos.0 &&
                       self.xmax as f64 >= pos.0 &&
                       self.xmin as f64 <= pos.1 &&
                       self.xmax as f64 >= pos.1 {
                        count += 1;
                    }
                }
            }
        }
        return count;
    }
}

fn part1(input: &str) -> usize {
    HailStorm::new(input, 200000000000000, 400000000000000).part1()
}

fn part2(_input: &str) -> usize {
    0
}

const EXAMPLE: &'static str = "\
    19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 24).unwrap();

    // Unit tests on provided examples
    let example = HailStorm::new(EXAMPLE, 7, 27);
    assert_eq!(example.part1(), 2);
    assert_eq!(part2(EXAMPLE), 0);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
