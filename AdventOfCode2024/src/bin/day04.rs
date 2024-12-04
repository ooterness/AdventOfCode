/// Advent of Code 2024, Day 4
/// Copyright 2024 by Alex Utter

use aocfetch;

// A grid of letters.
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = Vec::new();
        for line in input.trim().lines() {
            grid.push(line.trim().chars().collect());
        }
        return Grid {grid: grid};
    }

    fn rows(&self) -> usize {self.grid.len()}
    fn cols(&self) -> usize {self.grid[0].len()}

    fn get(&self, r: usize, c: usize, dr: isize, dc: isize) -> Option<char> {
        let rr = r.overflowing_add_signed(dr).0;
        let cc = c.overflowing_add_signed(dc).0;
        if rr >= self.rows() {return None;}
        if cc >= self.cols() {return None;}
        return Some(self.grid[rr][cc]);
    }

    fn count_xmas(&self, r: usize, c: usize) -> usize {
        // Ignore anything that doesn't start with an 'X'.
        if self.grid[r][c] != 'X' {return 0;}
        // Otherwise, try searching in each of eight directions.
        let mut count = 0usize;
        for d in [(-1,0), (-1,1), (0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1)] {
            if self.get(r, c, 1*d.0, 1*d.1) == Some('M') &&
               self.get(r, c, 2*d.0, 2*d.1) == Some('A') &&
               self.get(r, c, 3*d.0, 3*d.1) == Some('S')
               {count += 1;}
        }
        return count;
    }

    fn count_x_mas(&self, r: usize, c: usize) -> usize {
        // Ignore anything where the center isn't an 'A'.
        if self.grid[r][c] != 'A' {return 0;}
        // Otherwise, try searching in each of eight orientations.
        let mut count = 0usize;
        for d in [(-1,-1), (-1,1), (1,-1), (1,1)] {
            if self.get(r, c, -d.0, -d.1) == Some('M') &&
               self.get(r, c, -d.1,  d.0) == Some('M') &&
               self.get(r, c,  d.1, -d.0) == Some('S') &&
               self.get(r, c,  d.0,  d.1) == Some('S')
               {count += 1;}
        }
        return count;
    }
}

// Find all XMAS in the word-search grid.
fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut count = 0usize;
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            count += grid.count_xmas(r, c);
        }
    }
    return count;
}

// Find all X-MAS in the word-search grid.
fn part2(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut count = 0usize;
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            count += grid.count_x_mas(r, c);
        }
    }
    return count;
}

const EXAMPLE: &'static str = "\
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 4).unwrap();

    assert_eq!(part1(EXAMPLE), 18);
    assert_eq!(part2(EXAMPLE), 9);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
