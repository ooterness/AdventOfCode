/// Advent of Code 2023, Day 13
/// Copyright 2023 by Alex Utter

use aocfetch;

struct Maze {
    cols: Vec<u64>,
    rows: Vec<u64>,
}

// Count differences assuming a mirror between index N and N+1.
fn mirror_diff(input: &Vec<u64>, idx: usize) -> usize {
    let chk = core::cmp::min(idx, input.len() - idx);
    return (0..chk)
        .map(|c| input[idx-c-1] ^ input[idx+c])
        .map(|x| x.count_ones() as usize).sum();
}

impl Maze {
    fn new<'a,T>(lines: &mut T) -> Option<Self>
        where T: Iterator<Item = &'a str>
    {
        // Read row-by-row until we reach end-of-input or a blank line.
        let mut rows = Vec::new();
        let mut width = 0usize;
        while let Some(line) = lines.next() {
            let count = line.trim().chars().count();
            if count == 0 {break;}
            let mut row = 0u64;
            for (c,ch) in line.trim().chars().enumerate() {
                if ch == '#' {row += 1u64 << c;}
            }
            rows.push(row);
            width = count;
        }
        if rows.is_empty() {return None;}
        // Transpose operation.
        let mut cols = Vec::new();
        for c in 0..width {
            let mut col = 0u64;
            for (r,row) in rows.iter().enumerate() {
                col += ((row >> c) & 1) << r;
            }
            cols.push(col);
        }
        Some( Maze { cols:cols, rows:rows } )
    }

    fn score(&self, target: usize) -> usize {
        let mut total = 0usize;
        for c in 1..self.cols.len() {
            if mirror_diff(&self.cols, c) == target {total += c;}
        }
        for r in 1..self.rows.len() {
            if mirror_diff(&self.rows, r) == target {total += 100*r;}
        }
        return total;
    }
}

fn parse(input: &str) -> Vec<Maze> {
    let mut lines = input.trim().lines();
    let mut mazes = Vec::new();
    while let Some(maze) = Maze::new(&mut lines) {
        mazes.push(maze);
    }
    return mazes;
}

fn part1(input: &str) -> usize {
    let mazes = parse(input);
    mazes.iter().map(|m| m.score(0)).sum()
}

fn part2(input: &str) -> usize {
    let mazes = parse(input);
    mazes.iter().map(|m| m.score(1)).sum()
}

const EXAMPLE: &'static str = "\
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.\n
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 13).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 405);
    assert_eq!(part2(EXAMPLE), 400);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
