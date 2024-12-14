/// Advent of Code 2024, Day 13
/// Copyright 2024 by Alex Utter

use aocfetch;

fn tokenize(line: &str) -> Vec<i64> {
    line.trim().split([' ', '+', '=', ','])
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

struct Game {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl Game {
    fn new(input:&str, offset:bool) -> Vec<Self> {
        let delta = (offset as i64) * 10000000000000i64;
        let mut games = Vec::new();
        let mut lines = input.trim().lines();
        while let Some(line1) = lines.next() {
            let line2 = lines.next().unwrap();
            let line3 = lines.next().unwrap();
            lines.next();   // Skip blank line
            let tok1 = tokenize(line1);
            let tok2 = tokenize(line2);
            let tok3 = tokenize(line3);
            games.push( Game {
                ax:tok1[0], ay:tok1[1],
                bx:tok2[0], by:tok2[1],
                px:tok3[0] + delta, py:tok3[1] + delta,
            } );
        }
        return games;
    }

    fn parallel(&self) -> bool {
        (self.ax * self.by - self.ay * self.bx) == 0
    }

    // Minimum tokens to reach prize, if possible.
    fn solve(&self) -> Option<i64> {
        // Parallel basis vectors need a more complex solver.
        if self.parallel() {panic!("Parallel basis");}
        // Otherwise, there is always a unique algebraic solution.
        let ka = (self.px * (self.bx + self.by) - self.bx * (self.px + self.py))
               / (self.ax * (self.bx + self.by) - self.bx * (self.ax + self.ay));
        let dx = self.px - self.ax * ka;
        let dy = self.py - self.ay * ka;
        let kb = (dx + dy) / (self.bx + self.by);
        if ka >= 0 && kb >= 0 && self.bx*kb == dx && self.by*kb == dy {
            return Some(3*ka + kb);
        } else {
            return None;
        }
    }
}

fn part1(input: &str) -> i64 {
    Game::new(input, false).iter().filter_map(|game| game.solve()).sum()
}

fn part2(input: &str) -> i64 {
    Game::new(input, true).iter().filter_map(|game| game.solve()).sum()
}

const EXAMPLE: &'static str = "\
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 13).unwrap();

    assert_eq!(part1(EXAMPLE), 480);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
