/// Day 1: https://adventofcode.com/2020/day/1
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

pub fn solve() {
    // Simple example from problem statement.
    let test_i = [1721, 979, 366, 299, 675, 1456].to_vec();
    let test_o = find_pair(2020, &test_i);
    print_product2("Test", test_o);

    // Part 1 and Part 2 solutions:
    let input = common::read_integers("input/input01.txt");

    let soln1 = find_pair(2020, &input);
    print_product2("Part1", soln1);

    let soln2 = find_triplet(2020, &input);
    print_product3("Part1", soln2);
}

fn print_product2(lbl: &str, xy: (i64, i64)) {
    println!("{}: {} * {} = {}",
        lbl, xy.0, xy.1, xy.0 * xy.1)
}

fn print_product3(lbl: &str, xyz: (i64, i64, i64)) {
    println!("{}: {} * {} * {} = {}",
        lbl, xyz.0, xyz.1, xyz.2, xyz.0 * xyz.1 * xyz.2)
}

// Find a pair of numbers that add up to "target".
fn find_pair(target:i64, list:&Vec<i64>) -> (i64, i64) {
    for a in 0..list.len()-1 {
        for b in a+1..list.len() {
            if list[a] + list[b] == target {
                return (list[a], list[b]);
            }
        }
    }
    return (0,0)
}

// Find a triplet of numbers that add up to "target".
fn find_triplet(target:i64, list:&Vec<i64>) -> (i64, i64, i64) {
    for a in 0..list.len()-2 {
        for b in a+1..list.len()-1 {
            for c in b+1..list.len() {
                if list[a] + list[b] + list[c] == target {
                    return (list[a], list[b], list[c]);
                }
            }
        }
    }
    return (0,0,0)
}
