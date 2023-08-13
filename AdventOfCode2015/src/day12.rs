/// Advent of Code 2015, Day 12
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

fn part1(input: &str) -> i64
{
    // Character-by-character parsing.
    let mut total = 0i64;
    let mut accum = 0i64;
    let mut sign  = 1i64;
    for ch in input.chars() {
        if let Some(d) = ch.to_digit(10) {
            accum = 10*accum + (d as i64) * sign;
        } else if ch == '-' {
            total += accum;
            accum = 0;
            sign = -1;
        } else {
            total += accum;
            accum = 0;
            sign = 1;
        }
    }
    return total + accum;
}

fn part2(input: &str) -> i64
{
    // Character-by-character parsing, now with a stack
    // to nullify objects with the "red" property.
    let mut stack: Vec<(i64, i64, i64)> = Vec::new();
    let mut accum3 = 0i64;  // Parent object
    let mut accum2 = 0i64;  // Current object
    let mut accum1 = 0i64;  // Current number
    let mut sign = 1i64;
    let mut keep = 1i64;
    let mut prev = [' ';5];
    for ch in input.chars() {
        // Watch for the "red" keyword.
        if prev[0] == ':' &&
           prev[1] == '"' &&
           prev[2] == 'r' &&
           prev[3] == 'e' &&
           prev[4] == 'd' &&
           ch      == '"' {keep = 0;}
        prev = [prev[1], prev[2], prev[3], prev[4], ch];
        // Running total within current object.
        if let Some(d) = ch.to_digit(10) {
            accum1 = 10*accum1 + (d as i64) * sign;
        } else if ch == '-' {
            accum2 += accum1;
            accum1 = 0;
            sign = -1;
        } else {
            accum2 += accum1;
            accum1 = 0;
            sign = 1;
        }
        // Nested-object parsing.
        if ch == '{' {
            stack.push((accum3, accum2, keep));
            accum3 = accum2;
            accum2 = 0;
            keep = 1;
        } else if ch == '}' {
            let temp = accum2 * keep;
            (accum3, accum2, keep) = stack.pop().unwrap();
            accum2 += temp;
        }
    }
    return accum3 + accum2 * keep;
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 12).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1("[1,2,3]"), 6);
    assert_eq!(part1("{\"a\":2,\"b\":4}"), 6);
    assert_eq!(part1("[[[3]]]"), 3);
    assert_eq!(part1("{\"a\":{\"b\":4},\"c\":-1}"), 3);
    assert_eq!(part1("{\"a\":[-1,1]}"), 0);
    assert_eq!(part1("[-1,{\"a\":1}]"), 0);
    assert_eq!(part1("[]"), 0);
    assert_eq!(part1("{}"), 0);
    assert_eq!(part2("[1,2,3]"), 6);
    assert_eq!(part2("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
    assert_eq!(part2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
    assert_eq!(part2("[1,\"red\",5]"), 6);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
