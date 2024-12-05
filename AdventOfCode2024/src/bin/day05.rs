/// Advent of Code 2024, Day 5
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

struct Ruleset {
    lt: HashMap<i64, HashSet<i64>>,     // A < B --> lt[a] contains b
}

struct Pages {
    pages: Vec<i64>,
}

type Pageset = Vec<Pages>;

impl Ruleset {
    fn new() -> Self {
        Ruleset {lt: HashMap::new()}
    }

    // Add direct rules, one line at a time.
    fn add(&mut self, line: &str) {
        // New rule A < B.
        let tok: Vec<i64> = line.trim().split('|')
            .map(|x| x.parse().unwrap()).collect();
        let a = tok[0]; let b = tok[1];
        // Direct updates: Add new rules A < B and B > A.
        self.lt.entry(a).or_insert(HashSet::new()).insert(b);
    }

    // Comparison function for use with sort_by(...).
    fn cmp(&self, a: &i64, b: &i64) -> Ordering {
        if let Some(rule) = self.lt.get(a) {
            if rule.contains(b) {return Ordering::Less;}
        }
        if let Some(rule) = self.lt.get(b) {
            if rule.contains(a) {return Ordering::Greater;}
        }
        return Ordering::Equal;
    }

    // Given a set of active pages, make a new ruleset that
    // includes transitive rules (e.g., A < B, B < C --> A < C).
    // Note: Rules are NOT transitive in the general case, only
    // for a particular set of pages. (Inputs may include loops.)
    fn solve(&self, pages: &Pages) -> Self {
        let mut rules = Ruleset::new();
        // Convert the page list to a HashSet for faster search.
        let pset: HashSet<i64> = pages.pages.iter().cloned().collect();
        // For each rule in the input, check if it is active.
        // (Only active rules count as transitive 
        for (&a,bb) in self.lt.iter() {
            if !pset.contains(&a) {continue;}
            for &b in bb.iter() {
                if !pset.contains(&b) {continue;}
                // Insert direct rule A < B.
                rules.lt.entry(a).or_insert(HashSet::new()).insert(b);
                // For each prior rule B < [C], create new transitive rule A < C.
                if let Some(cc) = rules.lt.get(&b).cloned() {
                    for c in cc.into_iter() {
                        rules.lt.entry(a).or_insert(HashSet::new()).insert(c);
                    }
                }
            }
        }
        return rules;
    }
}

impl Pages {
    fn new(line: &str) -> Self {
        let tok: Vec<i64> = line.trim().split(',')
            .map(|x| x.parse().unwrap()).collect();
        return Pages {pages: tok};
    }

    // For an odd-length list of pages, find the middle value.
    fn middle(&self) -> Option<i64> {
        if self.pages.len() == 0 {return None;}
        return Some(self.pages[(self.pages.len() - 1) / 2]);
    }

    // Does this list of pages follow all of the rules?
    fn follows(&self, rules: &Ruleset) -> bool {
        // For each page, check for applicable rule(s).
        for (pidx,pval) in self.pages.iter().enumerate() {
            if let Some(rule) = rules.lt.get(pval) {
                // Do any preceding page numbers violate a rule?
                for n in 0..pidx {
                    if rule.contains(&self.pages[n]) {return false;}
                }
            }
        }
        return true; // All rules OK
    }

    // Reorder this list of pages to follow all rules.
    fn solve(&self, rules: &Ruleset) -> Self {
        // Using only active pages, solve transitive rules.
        let tr = rules.solve(self);
        // Use that comparison function to sort the pages.
        let mut sorted = self.pages.clone();
        sorted.sort_by(|a,b| tr.cmp(a,b));
        return Pages {pages: sorted};
    }
}

fn parse(input: &str) -> (Ruleset, Pageset) {
    let mut blank = false;
    let mut rset = Ruleset::new();
    let mut pset = Pageset::new();
    for line in input.trim().lines() {
        // Parse rules, then blank line, then page lists.
        if line.len() == 0 {
            blank = true;
        } else if blank {
            pset.push(Pages::new(line));
        } else {
            rset.add(line);
        }
    }
    return (rset, pset);
}

fn part1(input: &str) -> i64 {
    let (rules, pages) = parse(input);
    let mut total = 0i64;
    for pset in pages {
        if pset.follows(&rules) {total += pset.middle().unwrap();}
    }
    return total;
}

fn part2(input: &str) -> i64 {
    let (rules, pages) = parse(input);
    let mut total = 0i64;
    for pset in pages {
        if pset.follows(&rules) {continue;}
        let reorder = pset.solve(&rules);
        total += reorder.middle().unwrap();
    }
    return total;
}

const EXAMPLE: &'static str = "\
    47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47";


fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 5).unwrap();

    assert_eq!(part1(EXAMPLE), 143);
    assert_eq!(part2(EXAMPLE), 123);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
