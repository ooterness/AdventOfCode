/// Advent of Code 2017, Day 2
/// Copyright 2023 by Alex Utter

extern crate aocfetch;

type Row = Vec<i64>;
type Matrix = Vec<Row>;

fn read_row(line: &str) -> Row
{
    return line.split('\t')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn read_matrix(input: &str) -> Matrix
{
    return input.lines()
        .map(read_row)
        .collect()
}

fn part1(input: &Matrix) -> i64
{
    let mut sum = 0;
    for row in input.iter() {
        let x0 = *row.iter().min().unwrap_or(&0);
        let x1 = *row.iter().max().unwrap_or(&0);
        sum += x1 - x0;
    }
    return sum
}

fn part2(input: &Matrix) -> i64
{
    let mut sum = 0;
    for row in input.iter() {
        for a in row.iter() {
            for b in row.iter() {
                if a % b == 0 { sum += a / b; }
            }
        }
        sum -= row.len() as i64 // Don't count self-match
    }
    return sum 
}

fn main() {
    // Fetch input from server.
    let test1 = read_matrix("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8\n");
    let test2 = read_matrix("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5\n");
    let input = read_matrix(&aocfetch::get_data(2017, 2).unwrap());

    // Unit tests on provided examples
    assert_eq!(part1(&test1), 18);
    assert_eq!(part2(&test2), 9);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
