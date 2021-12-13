/// Day 13: https://adventofcode.com/2021/day/13
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::collections::HashSet;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Dot {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Paper {
    dots: HashSet<Dot>,
    cmd0: usize,
}

impl Paper {
    fn new(lines: &Vec<String>) -> Paper {
        // Read only the "x,y" lines and ignore the rest.
        let mut paper = Paper {dots: HashSet::new(), cmd0: 0};
        for (n,line) in lines.iter().enumerate() {
            let xy = common::split_str_as::<usize>(line, ',');
            if xy.len() == 2 {
                // Each valid line is added to the list.
                paper.dots.insert(Dot {x:xy[0], y:xy[1]} );
            } else if line.len() > 0 {
                // Note index of first command line.
                paper.cmd0 = n; break;
            }
        }
        paper
    }

    fn count(&self) -> usize {
        self.dots.len()
    }

    fn fold_x(&self, fold: usize) -> Paper {
        let mut next = Paper {dots: HashSet::new(), cmd0: self.cmd0};
        for dot in self.dots.iter() {
            let dx = if dot.x < fold {dot.x} else {2*fold - dot.x};
            next.dots.insert(Dot {x:dx, y:dot.y} );
        }
        next
    }

    fn fold_y(&self, fold: usize) -> Paper {
        let mut next = Paper {dots: HashSet::new(), cmd0: self.cmd0};
        for dot in self.dots.iter() {
            let dy = if dot.y < fold {dot.y} else {2*fold - dot.y};
            next.dots.insert(Dot {x:dot.x, y:dy} );
        }
        next
    }

    // Parse the command, e.g., "fold along y=7"
    fn fold_cmd(&self, line: &str) -> Paper {
        let split:Vec<&str> = line.split('=').collect();
        assert_eq!(split.len(), 2);
        let cmd = split[0];
        let num = split[1].parse::<usize>().ok();
        match (cmd,num) {
            ("fold along x", Some(n)) => self.fold_x(n),
            ("fold along y", Some(n)) => self.fold_y(n),
            _ => self.clone(),
        }
    }

    // Print the current state.
    fn print(&self) {
        let cols = self.dots.iter().map(|d| d.x).max().unwrap_or(0) + 1;
        let rows = self.dots.iter().map(|d| d.y).max().unwrap_or(0) + 1;
        for r in 0..rows {
            let row: String = (0..cols)
                .map(|c| Dot {x:c, y:r} )           // Create Dot object
                .map(|d| self.dots.contains(&d))    // Check current state
                .map(|h| if h {'#'} else {'.'})     // Highlight this cell?
                .collect();
            println!("{}", row);
        }
    }
}

pub fn solve() {
    let test = common::read_lines("input/test13.txt");
    let data = common::read_lines("input/input13.txt");
    let ptest = Paper::new(&test);
    let pdata = Paper::new(&data);

    // Tests using the example input
    assert_eq!(ptest.count(), 18);
    assert_eq!(ptest.fold_y(7).count(), 17);
    assert_eq!(ptest.fold_y(7).fold_x(5).count(), 16);

    // Part 1 executes only the first fold.
    let cmds = &data[pdata.cmd0..];
    let part1 = pdata.fold_cmd(&cmds[0]);
    println!("Part1: {}", part1.count());

    // Part 2 executes the remaining folds and prints the result.
    let mut part2 = pdata.clone();
    for cmd in cmds.iter() {
        part2 = part2.fold_cmd(&cmd);
    }
    part2.print();
}
