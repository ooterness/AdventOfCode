/// Advent of Code 2016, Day 7
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashSet;

fn is_tls(line: &str) -> bool
{
    let mut bracket = false;        // Inside a bracket sequence?
    let mut abba_good = false;      // ABBA sequence outside bracket?
    let mut abba_bad  = false;      // ABBA sequence inside bracket?
    let mut ch1 = 'X';              // Previous three characters
    let mut ch2 = 'X';
    let mut ch3 = 'X';
    for ch4 in line.trim().chars() {
        if ch4 == '[' {             // Entering bracket?
            bracket = true;
        } else if ch4 == ']' {      // Leaving bracket?
            bracket = false;
        } else if ch1 == ch4 && ch2 == ch3 && ch1 != ch2 {
            // Found an ABBA sequence -> Set the appropriate flag.
            if bracket {abba_bad = true} else {abba_good = true};
        }
        ch1 = ch2; ch2 = ch3; ch3 = ch4;
    }
    return abba_good && !abba_bad;
}

fn is_ssl(line: &str) -> bool
{
    type CharSet = HashSet<(char, char)>;
    let mut seq0 = CharSet::new();  // ABA sequences (outside bracket)
    let mut seq1 = CharSet::new();  // BAB sequences (inside bracket)
    let mut bracket = false;        // Inside a bracket sequence?
    let mut ch1 = 'X';              // Previous two characters
    let mut ch2 = 'X';
    for ch3 in line.trim().chars() {
        if ch3 == '[' {             // Entering bracket?
            bracket = true;
        } else if ch3 == ']' {      // Leaving bracket?
            bracket = false;
        } else if ch1 == ch3 && ch1 != ch2 {
            // Found an ABA or BAB sequence.
            // Have we already seen its counterpart?
            if bracket {
                let bab = (ch2, ch1);
                if seq0.contains(&bab) {return true;}
                seq1.insert(bab);
            } else {
                let aba = (ch1, ch2);
                if seq1.contains(&aba) {return true;}
                seq0.insert(aba);
            }
        }
        ch1 = ch2; ch2 = ch3;
    }
    return false;
}

fn part1(input: &str) -> usize
{
    input.trim().lines().filter(|&x| is_tls(x)).count()
}

fn part2(input: &str) -> usize
{
    input.trim().lines().filter(|&x| is_ssl(x)).count()
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 7).unwrap();

    // Unit tests on provided examples
    assert_eq!(is_tls("abba[mnop]qrst"), true);
    assert_eq!(is_tls("abcd[bddb]xyyx"), false);
    assert_eq!(is_tls("aaaa[qwer]tyui"), false);
    assert_eq!(is_tls("ioxxoj[asdfgh]zxcvbn"), true);
    assert_eq!(is_ssl("aba[bab]xyz"), true);
    assert_eq!(is_ssl("xyx[xyx]xyx"), false);
    assert_eq!(is_ssl("aaa[kek]eke"), true);
    assert_eq!(is_ssl("zazbz[bzb]cdb"), true);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
