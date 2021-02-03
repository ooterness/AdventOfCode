/// Day 2: https://adventofcode.com/2020/day/2
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

pub fn solve() {
    // Simple example from problem statement.
    let test_i = vec![
        String::from("1-3 a: abcde"),
        String::from("1-3 b: cdefg"),
        String::from("2-9 c: ccccccccc"),
    ];
    let test1 = count_valid_passwords1(&test_i);
    println!("Test 1: {} valid passwords.", test1);

    // Part 1: Check each password from input file.
    let input = common::read_strings("input/input02.txt");
    let part1 = count_valid_passwords1(&input);
    println!("Part 1: {} valid passwords.", part1);

    // Part 2: Same with the new rules.
    let test2 = count_valid_passwords2(&test_i);
    println!("Test 2: {} valid passwords.", test2);
    let part2 = count_valid_passwords2(&input);
    println!("Part 2: {} valid passwords.", part2);
}

/// Count valid passwords using Part-1 rule
fn count_valid_passwords1(list: &Vec<String>) -> usize {
    let valid = list.iter().map(|x| password_is_valid1(x));
    common::count_true(valid)
}

/// Count valid passwords using Part-2 rule
fn count_valid_passwords2(list: &Vec<String>) -> usize {
    let valid = list.iter().map(|x| password_is_valid2(x));
    common::count_true(valid)
}

/// Test a single Part-1 password rule
fn password_is_valid1(line: &String) -> bool {
    if let Some(rule) = rule_parse(line) {
        let cnum = count_chars(rule.2, rule.3);
        return (rule.0 <= cnum) && (cnum <= rule.1)
    } else {
        return false
    }
}

/// Test a single Part-2 password rule
fn password_is_valid2(line: &String) -> bool {
    if let Some(rule) = rule_parse(line) {
        let char1 = rule.3.chars().nth((rule.0-1) as usize).unwrap();
        let char2 = rule.3.chars().nth((rule.1-1) as usize).unwrap();
        return (char1 == rule.2 && char2 != rule.2)
            || (char1 != rule.2 && char2 == rule.2)
    } else {
        return false
    }
}

/// Parser for password-rule strings
struct Rule<'a>(usize, usize, char, &'a str);
fn rule_parse(line: &String) -> Option<Rule> {
    // Typical line: "1-3 b: cdefg"
    // Tokenized:     0 1 2  44444
    let vec: Vec<&str> = line.split(&[' ','-',':'][..]).collect();
    if vec.len() < 5 {
        return None     // Invalid input string (not enough parts)
    } else if let Some(cref) = vec[2].chars().nth(0) {
        let cmin:usize = vec[0].parse().unwrap();
        let cmax:usize = vec[1].parse().unwrap();
        return Some(Rule(cmin, cmax, cref, vec[4]))
    } else {
        return None     // Invalid input string (no reference)
    }
}

/// Count instances of a specific char in a string.
fn count_chars(r:char, s:&str) -> usize {
    common::count_true(s.chars().map(|c| c==r))
}
