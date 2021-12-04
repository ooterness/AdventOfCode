/// Day 4: https://adventofcode.com/2021/day/4
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::collections::HashMap;

// Local definitions:
const BOARD_SIZE: usize = 5;    // Size of Bingo board = N x N
type Callout = u8;              // Callout number = 0-99
type Row = Vec<Callout>;        // A single row from a Bingo board

// A row/column pair
struct RowCol {
    r: usize,
    c: usize,
}

impl RowCol {
    fn idx(&self) -> usize {
        BOARD_SIZE * self.r + self.c
    }
}

// State for a 5x5 Bingo board
struct Board {
    done: bool,
    grid: HashMap<Callout,RowCol>,
    grid_marked: Vec<bool>,
    cols_marked: Vec<usize>,
    rows_marked: Vec<usize>,
}

impl Board {
    // Read many Boards from a string iterator (i.e., until end of file).
    fn read_all<'a>(iter: &mut impl Iterator<Item=&'a String>) -> Vec<Board> {
        let mut boards = Vec::new();
        while let Some(board) = Board::read_board(iter) {
            boards.push(board);
        }
        return boards
    }

    // Read the next few lines from a string iterator to form one Board.
    fn read_board<'a>(iter: &mut impl Iterator<Item=&'a String>) -> Option<Board> {
        // Read from iterator until we get BOARD_SIZE valid rows.
        // (Note we may need to skip empty rows during this process.)
        let mut rows = Vec::new();
        while let Some(line) = iter.next() {
            if let Some(row) = Board::parse_row(&line) {
                rows.push(row)
            }
            if rows.len() == BOARD_SIZE as usize {
                return Some(Board::parse_board(&rows))
            }
        }
        None    // Couldn't read a complete board?
    }

    // Create a Board object from a set of Rows.
    fn parse_board(rows: &Vec<Row>) -> Board {
        // Insert each grid element into the lookup table.
        let mut grid: HashMap<Callout,RowCol> = HashMap::new();
        for (r,row) in rows.iter().enumerate() {
            for (c,num) in row.iter().enumerate() {
                grid.insert(*num, RowCol {r:r, c:c});
            }
        }
        // Initialize the per-row and per-column counters to zero.
        Board {
            done: false,
            grid: grid,
            grid_marked: vec![false; BOARD_SIZE*BOARD_SIZE],
            cols_marked: vec![0; BOARD_SIZE],
            rows_marked: vec![0; BOARD_SIZE],
        }
    }

    // Parse a single row from a line of text.
    fn parse_row(line: &str) -> Option<Row> {
        let row = common::split_str_as(line, ' ');
        if row.len() == BOARD_SIZE {Some(row)} else {None}
    }

    // Mark a number on a board.  Return true if board just won.
    fn mark(&mut self, x: &Callout) -> bool {
        if self.done {return false}     // Ignore completed boards
        if let Some(pos) = self.grid.get(x) {
            self.grid_marked[pos.idx()] = true;
            self.cols_marked[pos.c] += 1usize;
            self.rows_marked[pos.r] += 1usize;
            self.done = (self.cols_marked[pos.c] >= BOARD_SIZE)
                     || (self.rows_marked[pos.r] >= BOARD_SIZE);
        }
        self.done                       // Did this board just win?
    }

    // Find the sum of all unmarked squares.
    fn sum_unmarked(&self) -> u64 {
        let mut sum = 0u64;
        for (num, pos) in self.grid.iter() {
            if !self.grid_marked[pos.idx()] {
                sum += *num as u64;
            }
        }
        sum
    }
}

// Read an input file.
fn read_input(filename: &str) -> (Vec<Callout>, Vec<Board>) {
    let lines = common::read_lines(filename);
    let mut line_iter = lines.iter();
    let callouts = common::split_str_as(line_iter.next().unwrap(), ',');
    let boards = Board::read_all(&mut line_iter);
    (callouts, boards)
}

// Part 1 solution: Find the earliest winner.
fn solve_part1(filename: &str) -> (u64, u64) {
    let (callouts, mut boards) = read_input(filename);
    for c in callouts.iter() {
        for b in boards.iter_mut() {
            if b.mark(&c) {return (*c as u64, b.sum_unmarked())}
        }
    }
    (0, 0)
}

// Part 1 solution: Find the latest winner.
fn solve_part2(filename: &str) -> (u64, u64) {
    let (callouts, mut boards) = read_input(filename);
    let mut result = (0u64, 0u64);
    for c in callouts.iter() {
        for b in boards.iter_mut() {
            if b.mark(&c) {result = (*c as u64, b.sum_unmarked())}
        }
    }
    result
}

pub fn solve() {
    // Part-1 solution (test + full)
    let test1 = solve_part1("input/test04.txt");
    assert_eq!(test1.0, 24);    // Final callout
    assert_eq!(test1.1, 188);   // Unmarked points
    let part1 = solve_part1("input/input04.txt");
    println!("Part1: {}", part1.0 * part1.1);

    // Part-2 solution (test + full)
    let test2 = solve_part2("input/test04.txt");
    assert_eq!(test2.0, 13);    // Final callout
    assert_eq!(test2.1, 148);   // Unmarked points
    let part2 = solve_part2("input/input04.txt");
    println!("Part2: {}", part2.0 * part2.1);
}
