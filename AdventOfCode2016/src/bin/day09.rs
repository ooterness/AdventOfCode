/// Advent of Code 2016, Day 9
/// Copyright 2023 by Alex Utter

use aocfetch;

// Read a marker of the form "(MxN)".
// Returns tuple (L, M, N), where L is the length of the marker itself.
fn read_marker(input: &[char]) -> Option<(usize, usize, usize)> {
    let mut m = 0usize;
    let mut n = 0usize;
    for (p, &ch) in input.iter().enumerate() {
        if ch == ')' {          // End of marker
            return Some((p+1, m, n));
        } else if ch == 'x' {   // Start parsing second number
            m = n;
            n = 0;
        } else if let Some(d) = ch.to_digit(10) {
            n = 10*n + d as usize;
        }
    }
    return None;                // Error (missing end-of-marker)
}

// Decompression with or without recursion.
fn analyze(input: &[char], recurse: bool) -> usize {
    let mut pos = 0usize;   // Current read position
    let mut tot = 0usize;   // Running total length
    while pos < input.len() {
        if input[pos] == '(' {
            let (l,m,n) = read_marker(&input[pos..]).unwrap();
            pos += l;       // Consume marker itself.
            if recurse {    // Expand recursively?
                let seg = &input[pos..pos+m];
                tot += analyze(seg, true) * n;
            } else {        // Expand one layer only
                tot += m * n;
            }
            pos += m;       // Consume input segment.
        } else {
            pos += 1;
            tot += 1;
        }
    }
    return tot;
}

fn part1(input: &str) -> usize {
    let tmp: Vec<char> = input.trim().chars().collect();
    analyze(&tmp, false)
}

fn part2(input: &str) -> usize {
    let tmp: Vec<char> = input.trim().chars().collect();
    analyze(&tmp, true)
}

const TEST1: &str = "ADVENT";
const TEST2: &str = "A(1x5)BC";
const TEST3: &str = "(3x3)XYZ";
const TEST4: &str = "A(2x2)BCD(2x2)EFG";
const TEST5: &str = "(6x1)(1x3)A";
const TEST6: &str = "X(8x2)(3x3)ABCY";
const TEST7: &str = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
const TEST8: &str = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 9).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST1), 6);
    assert_eq!(part1(TEST2), 7);
    assert_eq!(part1(TEST3), 9);
    assert_eq!(part1(TEST4), 11);
    assert_eq!(part1(TEST5), 6);
    assert_eq!(part1(TEST6), 18);
    assert_eq!(part2(TEST3), 9);
    assert_eq!(part2(TEST6), 20);
    assert_eq!(part2(TEST7), 241920);
    assert_eq!(part2(TEST8), 445);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
