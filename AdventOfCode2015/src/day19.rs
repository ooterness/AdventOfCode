/// Advent of Code 2015, Day 19
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashMap;
use std::collections::HashSet;

type Rules = HashMap<String, Vec<String>>;
type State = HashSet<String>;

fn parse(input: &str) -> (Rules, String) {
    let mut rules = Rules::new();
    let mut state = String::new();
    for line in input.trim().lines() {
        if line.contains(" => ") {
            let words: Vec<&str> = line.trim().split(' ').collect();
            rules.entry(String::from(words[0]))
                .or_insert(Vec::new())
                .push(String::from(words[2]));
        } else if line.trim().len() > 0 {
            state = String::from(line.trim());
        }
    }
    return (rules, state)
}

fn iter(rules: &Rules, state: &State) -> State {
    // Try applying each rule...
    let mut result = State::new();
    for (rule_from, rule_to_vec) in rules.iter() {
        for rule_to in rule_to_vec.iter() {
            // For each matching initial state...
            for chem in state.iter().filter(|st| st.contains(rule_from)) {
                // Apply the rule at each matching position.
                let words: Vec<&str> = chem.split(rule_from).collect();
                for m in 1..words.len() {
                    let mut accum = String::from(words[0]);
                    for n in 1..words.len() {
                        accum += if m == n {rule_to} else {rule_from};
                        accum += words[n];
                    }
                    result.insert(accum);
                }
            }
        }
    }
    return result;
}

fn part1(input: &str) -> usize {
    let (rules, init_str) = parse(input);
    let init = State::from([init_str]);
    return iter(&rules, &init).len();
}

fn part2(input: &str) -> usize {
    let (rules, target) = parse(input);
    let mut count = 0usize;
    let mut state = State::new();
    state.insert(String::from("e"));
    while !state.contains(&target) {
        count += 1;
        state = iter(&rules, &state);
    }
    return count;
}

const TEST: &str = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOH";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 19).unwrap();

    // Unit tests based on the provided examples:
    assert_eq!(part1(TEST), 4);
    assert_eq!(part2(TEST), 3);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
