/// Advent of Code 2023, Day 3
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashMap;
use std::collections::HashSet;

type Rc = (usize, usize);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Number {
    rc: Rc,
    value: u64,
    width: usize,
}

struct Grid {
    numbers: Vec<Number>,
    symbols: HashMap<Rc, char>,
}

impl Number {
    fn has_symbol(&self, grid: &Grid) -> bool {
        // Check the rectangular perimeter around this part-number.
        let rmid = self.rc.0;
        let cmin = self.rc.1 - 1;
        let cmax = self.rc.1 + self.width;
        if grid.symbols.contains_key(&(rmid, cmin)) {return true;}
        if grid.symbols.contains_key(&(rmid, cmax)) {return true;}
        for c in cmin..=cmax {
            if grid.symbols.contains_key(&(rmid-1, c)) {return true;}
            if grid.symbols.contains_key(&(rmid+1, c)) {return true;}
        }
        return false;
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = Grid {numbers: Vec::new(), symbols: HashMap::new()};
        for (r, line) in input.trim().lines().enumerate() {
            let empty = Number {rc:(r+1,0), value:0, width:0};
            let mut tmp: Number = empty;
            for (c, ch) in line.trim().chars().enumerate() {
                if let Some(n) = ch.to_digit(10) {
                    if tmp.width == 0 {tmp.rc.1 = c+1;}
                    tmp.value = tmp.value * 10 + n as u64;
                    tmp.width += 1;
                } else {
                    if tmp.width > 0 {grid.numbers.push(tmp); tmp = empty;}
                    if ch != '.' {grid.symbols.insert((r+1,c+1), ch);}
                }
            }
            if tmp.width > 0 {grid.numbers.push(tmp);}
        }
        return grid;
    }

    // Create lookup table for finding number objects.
    fn nmap(&self) -> HashMap<Rc, Number> {
        let mut nmap = HashMap::new();
        for &num in self.numbers.iter() {
            let rmid = num.rc.0;
            let cmin = num.rc.1;
            let cmax = num.rc.1 + num.width - 1;
            for c in cmin..=cmax {
                nmap.insert((rmid, c), num);
            }
        }
        return nmap;
    }

    // Find all asterisk symbols with two adjacent numbers.
    fn gears(&self) -> Vec<(Number, Number)> {
        let mut gears = Vec::new();
        let nmap = self.nmap();
        for (rc,ch) in self.symbols.iter() {
            // Ignore all other symbols.
            if *ch != '*' {continue;}
            // Identify adjacent numbers.
            let mut adj: HashSet<Number> = HashSet::new();
            for rca in [(rc.0-1,rc.1-1), (rc.0-1,rc.1), (rc.0-1, rc.1+1), 
                        (rc.0+0,rc.1-1),                (rc.0+0, rc.1+1), 
                        (rc.0+1,rc.1-1), (rc.0+1,rc.1), (rc.0+1, rc.1+1)] {
                if let Some(num) = nmap.get(&rca) {adj.insert(*num);}
            }
            // If there are exactly two unique numbers, this is a gear.
            if adj.len() == 2 {
                let tmp: Vec<Number> = adj.into_iter().collect();
                gears.push((tmp[0], tmp[1]));
            }
        }
        return gears;
    }
}

fn part1(input: &str) -> u64 {
    let mut total = 0u64;
    let grid = Grid::new(input);
    for num in grid.numbers.iter() {
        if num.has_symbol(&grid) {total += num.value;}
    }
    return total;
}

fn part2(input: &str) -> u64 {
    let mut total = 0u64;
    let grid = Grid::new(input);
    for (m,n) in grid.gears() {
        total += m.value * n.value;
    }
    return total;
}

const EXAMPLE: &'static str = "\
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2023, 3).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 4361);
    assert_eq!(part2(EXAMPLE), 467835);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
