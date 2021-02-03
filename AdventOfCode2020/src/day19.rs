/// Day 19: https://adventofcode.com/2020/day/19
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
use std::collections::HashSet;
#[path = "common.rs"] mod common;

#[derive(Debug)]
enum Rule {
    Simplex(char),
    Sequence(Vec<usize>),
    EitherOr(Vec<usize>, Vec<usize>),
}

impl Rule {
    fn parse(rule:&String) -> Option<Rule> {
        if rule.contains('"') {
            let s:Vec<char> = rule.trim().chars().collect();
            if let Some(c) = s.get(1) {
                Some(Rule::Simplex(*c))
            } else {None}
        } else if rule.contains('|') {
            let seq:Vec<&str> = rule.split('|').collect();
            if let (Some(a), Some(b)) = (seq.get(0), seq.get(1)) {
                let a = common::parse_vec::<usize>(&String::from(*a), ' ');
                let b = common::parse_vec::<usize>(&String::from(*b), ' ');
                Some(Rule::EitherOr(a, b))
            } else {None}
        } else {
            let a = common::parse_vec::<usize>(&rule, ' ');
            Some(Rule::Sequence(a))
        }
    }
}

/// Parse each line of the form [idx]:[rule].
type RuleSet = HashMap<usize,Rule>;
fn parse_rules(lines:&Vec<String>) -> Option<RuleSet> {
    let mut rules = HashMap::new();
    for line in lines {
        let x:Vec<&str> = line.split(':').collect();
        if let (Some(idx), Some(rule)) = (x.get(0), x.get(1)) {
            let idx  = idx.parse::<usize>();
            let rule = Rule::parse(&String::from(*rule));
            if let (Ok(idx), Some(rule)) = (idx,rule) {
                rules.insert(idx, rule);
            } else {return None;}
        } else {return None;}
    }
    Some(rules)
}

/// Object for checking a string against a RuleSet.
struct RuleChecker<'a> {
    rcvd:   Vec<char>,
    rules:  &'a RuleSet,
    cache:  Vec<HashMap<usize, Vec<usize>>>,
}

impl<'a> RuleChecker<'a> {
    fn create(rcvd: &'a String, rules: &'a RuleSet) -> RuleChecker<'a> {
        let rvec:Vec<char> = rcvd.chars().collect();
        let count:usize = rules.keys().max().unwrap() + 1usize;
        let cache = vec![HashMap::new(); count];
        RuleChecker {rcvd:rvec, rules:rules, cache:cache}
    }

    /// Return true if rule zero matches the entire string.
    fn check(&mut self) -> bool {
        let len = self.rcvd.len();
        let to  = self.check_one(0, 0);
        to.iter().any(|x| *x == len)
    }

    // Helper functions for check_one(), see below.
    fn check_char(&mut self, rule:&char, from:usize) -> Vec<usize> {
        match self.rcvd.get(from) {
            Some(c) if c == rule => vec![from+1],
            _ => vec![]
        }
    }
    fn check_seq(&mut self, rule:&[usize], from:usize) -> HashSet<usize> {
        let mut result:HashSet<usize> = HashSet::new();
        if let Some(next) = rule.first() {
            // Attempt a match on the first sub-item, then recurse.
            let split = self.check_one(*next, from);
            for s in split.iter() {
                let to = self.check_seq(&rule[1..], *s);
                for t in to.iter() {result.insert(*t);}
            }
        } else {
            // End of sequence is always a match.
            result.insert(from);
        }
        result
    }
    fn check_miss(&mut self, rule:&Rule, from:usize) -> Vec<usize> {
        match rule {
            Rule::Simplex(c) =>
                self.check_char(c, from),
            Rule::Sequence(vec) =>
                self.check_seq(&vec, from).iter().cloned().collect(),
            Rule::EitherOr(vec1, vec2) => {
                let a = self.check_seq(&vec1, from);
                let b = self.check_seq(&vec2, from);
                a.union(&b).cloned().collect()
            },
        }
    }

    /// With Nth rule starting at "from", check if a match is possible.
    /// Returns a vector (possibly empty) of all possible "to" matches.
    fn check_one(&mut self, ridx:usize, from:usize) -> Vec<usize> {
        // First check the cache.
        if let Some(r) = self.cache[ridx].get(&from) {
            r.clone()
        } else if let Some(rule) = self.rules.get(&ridx) {
            let result = self.check_miss(rule, from);
            self.cache[ridx].insert(from, result.clone());
            result
        } else {
            vec![]
        }
    }
}

fn count_matches(raw:&Vec<String>, mutate:bool) -> usize {
    // Split rules and received messages.
    let grp   = common::group_strings(&raw);
    let rules = parse_rules(&grp[0]);
    let rxvec = &grp[1];

    // Count the number of valid messages.
    if let Some(mut rules) = rules {
        // Apply mutation to Rules #8 and #11, if requested.
        if mutate {
            rules.insert(8, Rule::EitherOr(vec![42], vec![42,8]));
            rules.insert(11, Rule::EitherOr(vec![42,31], vec![42,11,31]));
        }
        // Test each string.
        let mut count = 0usize;
        for rx in rxvec.iter() {
            let chk = RuleChecker::create(&rx, &rules).check();
            if chk {count += 1usize;}
        }
        count
    } else {0usize}
}

pub fn solve() {
    // Read each input file.
    let example1 = common::read_strings("input/test19a.txt");
    let example2 = common::read_strings("input/test19b.txt");
    let example3 = common::read_strings("input/test19c.txt");
    let input    = common::read_strings("input/input19.txt");

    assert_eq!(2, count_matches(&example1, false));
    assert_eq!(2, count_matches(&example2, false));
    assert_eq!(3, count_matches(&example3, false));
    assert_eq!(12, count_matches(&example3, true));
    println!("Part1: {} matches", count_matches(&input, false));
    println!("Part2: {} matches", count_matches(&input, true));
}
