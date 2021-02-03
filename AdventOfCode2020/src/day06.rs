/// Day 6: https://adventofcode.com/2020/day/6
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

/// Find all unique answers from a group of answer strings.
fn find_unique(vec: &Vec<String>) -> String {
    let mut grp: Vec<char> = vec.concat().chars().collect();
    grp.sort();                 // Sort concatenated responses
    grp.dedup();                // Remove consecutive duplicates
    grp.into_iter().collect()   // Convert vector to string
}

/// As "find_unique" but from a delimited list of groups.
fn group_unique(raw: &Vec<String>) -> Vec<String> {
    let grp = common::group_strings(raw);
    grp.iter().map(find_unique).collect()
}

/// Find all unanimous answers from a group of answer strings.
fn find_unam(vec: &Vec<String>) -> String {
    let mut out:String = String::new();
    for cc in 'a'..='z' {
        // For each possible answer "a-z", test which answer
        // strings in the group contain that character.
        let mask = vec.iter().map(|x| x.contains(cc));
        let count = common::count_true(mask) as usize;
        // If everyone has that answer, add it to output.
        if count == vec.len() {out.push(cc);}
    }
    out
}

/// As "find_unam" but from a delimited list of groups.
fn group_unam(raw: &Vec<String>) -> Vec<String> {
    let grp = common::group_strings(raw);
    grp.iter().map(find_unam).collect()
}

/// Find sum-of-counts for a list of strings.
fn sum_of_counts(vec: &Vec<String>) -> usize {
    vec.iter().map(|x| x.len()).sum()
}

/// Print a list of strings.
fn fmt_strvec(vec: &Vec<String>) -> String {
    let mut str = String::new();
    for (n,s) in vec.iter().enumerate() {
        if n == 0 {str.push_str("[");}
             else {str.push_str(",");}
        str.push_str(&s);
    }
    str.push_str("]");
    return str
}

/// Solve Part-1 and Part-2 of the problems statement.
pub fn solve() {
    // Parse the example input.
    let test_str = vec![
        String::from("abc"), String::from(""),
        String::from("a"), String::from("b"), String::from("c"), String::from(""),
        String::from("ab"), String::from("ac"), String::from(""),
        String::from("a"), String::from("a"), String::from("a"), String::from("a"), String::from(""),
        String::from("b")];
    let test1 = group_unique(&test_str);
    let test2 = group_unam(&test_str);
    println!("Test 1: Count {} {}", sum_of_counts(&test1), fmt_strvec(&test1));
    println!("Test 2: Count {} {}", sum_of_counts(&test2), fmt_strvec(&test2));

    // Read the main input.
    let input = common::read_strings("input/input06.txt");
    let part1 = group_unique(&input);
    println!("Part 1: Count {}", sum_of_counts(&part1));
    let part2 = group_unam(&input);
    println!("Part 2: Count {}", sum_of_counts(&part2));
}
