/// Advent of Code 2025, Day 3
/// Copyright 2025 by Alex Utter

use aocfetch;

type Row = Vec<usize>;
type Grid = Vec<Row>;

fn parse(input: &str) -> Grid {
    let mut grid = Grid::new();
    for line in input.trim().lines() {
        let row = line.trim().chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        grid.push(row);
    }
    return grid;
}

fn scan(row: &Row, digits: usize) -> usize {
    let mut accum = 0usize;
    let mut posn = 0usize;
    for d in 0..digits {
        // Find the largest digit, leaving enough left over.
        let mut best = posn;
        let rem = digits - d;
        for n in (posn)..=(row.len() - rem) {
            if row[n] > row[best] { best = n; }
        }
        // Add that digit to the accumulator, then recurse.
        accum = 10*accum + row[best];
        posn = best + 1;
    }
    return accum;
}

fn part1(grid: &Grid) -> usize {
    grid.iter().map(|row| scan(row, 2)).sum()
}

fn part2(grid: &Grid) -> usize {
    grid.iter().map(|row| scan(row, 12)).sum()
}

const EXAMPLE: &'static str = "\
    987654321111111
    811111111111119
    234234234234278
    818181911112111";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2025, 3).unwrap();

    assert_eq!(part1(&parse(EXAMPLE)), 357);
    assert_eq!(part2(&parse(EXAMPLE)), 3121910778619);

    let time = std::time::Instant::now();
    let data = parse(&input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
    println!("Elapsed time: {:.1?}", time.elapsed());
}
