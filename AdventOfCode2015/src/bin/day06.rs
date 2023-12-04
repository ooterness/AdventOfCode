/// Advent of Code 2015, Day 6
/// Copyright 2023 by Alex Utter

use aocfetch;

enum Action {
    Set(bool),
    Toggle,
}

struct Rule {
    action: Action,
    xmin: usize,
    ymin: usize,
    xmax: usize,
    ymax: usize,
}

impl Rule {
    fn new(line: &str) -> Rule {
        let tok: Vec<&str> = line.split(&[' ', ',']).collect();
        if tok[0] == "toggle" {
            assert_eq!(tok.len(), 6);
            let xmin: usize = tok[1].parse().unwrap();
            let ymin: usize = tok[2].parse().unwrap();
            let xmax: usize = tok[4].parse().unwrap();
            let ymax: usize = tok[5].parse().unwrap();
            return Rule {action: Action::Toggle,
                xmin:xmin, ymin:ymin, xmax:xmax, ymax:ymax};
        } else {
            assert_eq!(tok.len(), 7);
            let val: bool = tok[1] == "on";
            let xmin: usize = tok[2].parse().unwrap();
            let ymin: usize = tok[3].parse().unwrap();
            let xmax: usize = tok[5].parse().unwrap();
            let ymax: usize = tok[6].parse().unwrap();
            return Rule {action: Action::Set(val),
                xmin:xmin, ymin:ymin, xmax:xmax, ymax:ymax}
        }
    }
}

const SIZE: usize = 1000;

struct Grid {
    lit: Vec<Vec<usize>>,
}

impl Grid {
    fn new() -> Grid {
        let mut lit = Vec::new();
        for _ in 0..SIZE {lit.push(vec![0;SIZE]);}
        Grid { lit:lit }
    }

    fn apply1(&mut self, rule: &Rule) {
        for y in rule.ymin..rule.ymax+1 {
            for x in rule.xmin..rule.xmax+1 {
                let w = self.lit[y][x];
                self.lit[y][x] = match rule.action {
                    Action::Set(z) => if z {1} else {0},
                    Action::Toggle => 1 - w,
                }
            }
        }
    }

    fn apply2(&mut self, rule: &Rule) {
        for y in rule.ymin..rule.ymax+1 {
            for x in rule.xmin..rule.xmax+1 {
                let w = self.lit[y][x];
                self.lit[y][x] = match rule.action {
                    Action::Set(true)   => w + 1,
                    Action::Set(false)  => if w > 0 {w - 1} else {0},
                    Action::Toggle      => w + 2,
                }
            }
        }
    }

    fn count(&self) -> usize {
        self.lit.iter()
            .map(|row| row.iter().sum::<usize>())
            .sum()
    }
}

fn part1(input: &str) -> usize
{
    let mut grid = Grid::new();
    for rule in input.lines().map(Rule::new) {
        grid.apply1(&rule);
    }
    return grid.count();
}

fn part2(input: &str) -> usize
{
    let mut grid = Grid::new();
    for rule in input.lines().map(Rule::new) {
        grid.apply2(&rule);
    }
    return grid.count();
}

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2015, 6).unwrap();

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
