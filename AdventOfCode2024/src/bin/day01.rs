/// Advent of Code 2024, Day 1
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashMap;

type Pair = (i64, i64);
type Column = Vec<i64>;
type List = Vec<Pair>;

fn left(input: &List) -> Column {
    return input.iter().map(|x| x.0).collect();
}

fn right(input: &List) -> Column {
    return input.iter().map(|x| x.1).collect();
}

// Parse the raw input strings.
fn parse_lr(line: &str) -> Pair {
    let words: Vec<&str> = line.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect();
    return (words[0].parse().unwrap(),
            words[1].parse().unwrap());
}

fn parse(input: &str) -> List {
    return input.trim().lines()
        .map(|line| parse_lr(line))
        .collect();
}

// Sort the left and right columns.
fn sort(input: &List) -> (Column, Column) {
    let mut ll = left(input);
    let mut rr = right(input);
    ll.sort();
    rr.sort();
    return (ll, rr);
}

// Count unique elements in the given list.
fn count(col: &Column) -> HashMap<i64, i64> {
    let mut result = HashMap::new();
    for x in col {
        let prev = result.get(x).unwrap_or(&0i64);
        result.insert(*x, prev + 1i64);
    }
    return result;
}

// Sort the list, then sum of element-by-element difference.
fn part1(input: &List) -> i64 {
    let (ll, rr) = sort(input);
    let mut total = 0i64;
    for n in 0..ll.len() {
        total += (ll[n] - rr[n]).abs();
    }
    return total;
}

// Count each unique number, then sum(count_left * count_right).
fn part2(input: &List) -> i64 {
    let ll = count(&left(input));
    let rr = count(&right(input));
    let mut total = 0i64;
    for (xl,cl) in ll.iter() {
        let cr = rr.get(xl).unwrap_or(&0i64);
        total += xl * cl * cr;
    }
    return total;
}

const EXAMPLE: &'static str = "\
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 1).unwrap();

    assert_eq!(part1(&parse(EXAMPLE)), 11);
    assert_eq!(part2(&parse(EXAMPLE)), 31);

    let data = parse(&input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
