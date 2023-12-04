/// Advent of Code 2016, Day 18
/// Copyright 2023 by Alex Utter

use aocfetch;

struct Row {
    tiles: Vec<bool>,
}

impl Row {
    fn new(input: &str) -> Self {
        let tiles = input.trim().chars()
            .map(|ch| ch == '^')
            .collect();
        Row {tiles: tiles}
    }

    fn count_trap(&self) -> usize {
        self.tiles.iter().filter(|&x| *x).count()
    }

    fn count_safe(&self) -> usize {
        self.tiles.len() - self.count_trap()
    }

    fn count_near(&self, n: usize) -> bool {
        let a: bool = n > 0 && *self.tiles.get(n-1).unwrap_or(&false);
        let b: bool = *self.tiles.get(n).unwrap_or(&false);
        let c: bool = *self.tiles.get(n+1).unwrap_or(&false);
        return (a&&b&&!c) || (!a&&b&&c) || (a&&!b&&!c) || (!a&&!b&&c);
    }

    fn next_row(&self) -> Self {
        let tiles = (0..self.tiles.len())
            .map(|n| self.count_near(n))
            .collect();
        Row {tiles: tiles}
    }
}

fn count_safe(input: &str, rows: usize) -> usize {
    let mut row = Row::new(input);
    let mut safe = row.count_safe();
    for _ in 1..rows {
        row = row.next_row();
        safe += row.count_safe();
    }
    return safe;
}

fn part1(input: &str) -> usize {
    count_safe(input, 40)
}

fn part2(input: &str) -> usize {
    count_safe(input, 400000)
}

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 18).unwrap();

    // Unit tests on provided examples
    assert_eq!(count_safe("..^^.", 3), 6);
    assert_eq!(count_safe(".^^.^.^^^^", 10), 38);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
