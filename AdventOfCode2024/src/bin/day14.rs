/// Advent of Code 2024, Day 14
/// Copyright 2024 by Alex Utter

use aocfetch;

struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

struct Grid {
    sx: i64,
    sy: i64,
    bots: Vec<Robot>,
}

impl Robot {
    fn new(line: &str) -> Self {
        let tok: Vec<i64> = line.trim()
            .split([' ', '=', ','])
            .filter_map(|s| s.parse().ok())
            .collect();
        Robot {
            px: tok[0],
            py: tok[1],
            vx: tok[2],
            vy: tok[3],
        }
    }

    fn predict(&self, sx:i64, sy:i64, t:i64) -> Self {
        Robot {
            px: (self.px + t*self.vx).rem_euclid(sx),
            py: (self.py + t*self.vy).rem_euclid(sy),
            vx: self.vx,
            vy: self.vy,
        }
    }
}

impl Grid {
    fn new(input:&str, sx:i64, sy:i64) -> Self {
        let bots = input.trim().lines().map(Robot::new).collect();
        Grid {
            sx: sx,
            sy: sy,
            bots: bots,
        }
    }

    fn predict(&self, t:i64) -> Self {
        let bots = self.bots.iter()
            .map(|b| b.predict(self.sx, self.sy, t));
        Grid {
            sx: self.sx,
            sy: self.sy,
            bots: bots.collect(),
        }
    }

    fn count_char(&self, x:i64, y:i64) -> char {
        let ct: usize = self.bots.iter().filter(|b| b.px == x && b.py == y).count();
        return if ct == 0 {'.'}
          else if ct >= 9 {'9'}
          else {char::from_digit(ct as u32, 10).unwrap()};
    }

    fn print(&self) {
        for r in 0..self.sy {
            let row: String = (0..self.sx)
                .map(|c| self.count_char(c,r)).collect();
            println!("{:3}: {}", r, row);
        }
    }

    // Safety score heuristic specified for Part 1.
    fn safety(&self) -> usize {
        let mx = (self.sx - 1) / 2;
        let my = (self.sy - 1) / 2;
        let mut ct = [0usize;4];
        for bot in self.bots.iter() {
            if bot.px < mx && bot.py < my {ct[0] += 1;}
            if bot.px < mx && bot.py > my {ct[1] += 1;}
            if bot.px > mx && bot.py < my {ct[2] += 1;}
            if bot.px > mx && bot.py > my {ct[3] += 1;}
        }
        return ct.iter().product();
    }

    // Ad-hoc heuristics for detecting a coherent image for Part 2.
    // (The image is a solid blob plus a rectangular outline.)
    fn score(&self) -> f64 {
        // Calculate the centroid.
        let sumx: i64 = self.bots.iter().map(|b| b.px).sum();
        let sumy: i64 = self.bots.iter().map(|b| b.py).sum();
        let cx: f64 = (sumx as f64) / (self.bots.len() as f64);
        let cy: f64 = (sumy as f64) / (self.bots.len() as f64);
        // Calculate mean-square distance from centroid.
        let sumsq: f64 = self.bots.iter()
            .map(|b| (b.px as f64 - cx, b.py as f64 - cy))
            .map(|(dx,dy)| dx*dx + dy*dy).sum();
        return -sumsq;
    }
}

fn part1_smol(input: &str) -> usize {
    Grid::new(input, 11, 7).predict(100).safety()
}

fn part1(input: &str) -> usize {
    Grid::new(input, 101, 103).predict(100).safety()
}

fn part2(input: &str) -> usize {
    let grid = Grid::new(input, 101, 103);
    let mut best_score = f64::NEG_INFINITY;
    let mut best_time = 0i64;
    for t in 1..10000 {
        let score = grid.predict(t).score();
        if score > best_score {
            best_score = score;
            best_time = t;
        }
    }
    grid.predict(best_time).print();
    return best_time as usize;
}

const EXAMPLE: &'static str = "\
    p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 14).unwrap();

    assert_eq!(part1_smol(EXAMPLE), 12);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
