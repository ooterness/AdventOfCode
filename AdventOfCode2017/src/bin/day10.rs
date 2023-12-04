/// Advent of Code 2017, Day 10
/// Copyright 2023 by Alex Utter

use aocfetch;
#[path = "knot.rs"] mod knot;

fn part1(size: usize, input: &str) -> i64 {
    knot::part1(size, input)
}

fn part2(input: &str) -> String {
    knot::hash_hex(input)
}

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 10).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(5, "3,4,1,5"), 12);
    assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");

    // Solve for real input.
    println!("Part 1: {}", part1(256, &input));
    println!("Part 2: {}", part2(input.trim()));
}
