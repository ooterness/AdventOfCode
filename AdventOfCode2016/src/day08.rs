/// Advent of Code 2016, Day 8
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

const COLS: usize = 50;
const ROWS: usize = 6;
const TOKENS: [char;4] = [' ', '=', 'x', 'y'];

fn count_bool(row: &[bool]) -> usize {
    row.iter().filter(|&b| *b).count()
}

struct Screen {
    pix: [[bool;COLS];ROWS],
}

impl Screen {
    fn new() -> Self {
        Screen {pix: [[false;COLS];ROWS]}
    }

    fn apply(&self, line: &str) -> Self {
        let mut next = self.pix.clone();
        let cmd: Vec<&str> = line.split(&TOKENS).collect();
        if cmd[0] == "rect" {
            let cc: usize = cmd[1].parse().unwrap();
            let rr: usize = cmd[2].parse().unwrap();
            for r in 0..rr {
                for c in 0..cc {
                    next[r][c] = true;
                }
            }
        } else if cmd[1] == "row" {
            let rr: usize = cmd[4].parse().unwrap();
            let dd: usize = cmd[7].parse().unwrap();
            for c in 0..COLS {
                next[rr][(c+dd)%COLS] = self.pix[rr][c];
            }
        } else if cmd[1] == "column" {
            let cc: usize = cmd[4].parse().unwrap();
            let dd: usize = cmd[7].parse().unwrap();
            for r in 0..ROWS {
                next[(r+dd)%ROWS][cc] = self.pix[r][cc];
            }
        }
        return Screen {pix: next};
    }

    fn count(&self) -> usize {
        self.pix.iter().map(|&row| count_bool(&row)).sum()
    }

    fn render(&self) -> String {
        let mut result = String::new();
        for row in self.pix.iter() {
            for col in row.iter() {
                result.push(if *col {'#'} else {' '});
            }
            result.push('\n');
        }
        return result;
    }
}

fn execute(input: &str, verbose: bool) -> Screen {
    let mut state = Screen::new();
    for line in input.trim().lines() {
        state = state.apply(line);
        if verbose {println!("{}:\n{}", line, state.render())};
    }
    return state;
}

fn part1(input: &str) -> usize {
    execute(input, false).count()
}

fn part2(input: &str) -> String {
    execute(input, false).render()
}

const TEST_IN: &str = "\
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 8).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST_IN), 6);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2:\n{}", part2(&input));
}
