/// Day 14: https://adventofcode.com/2021/day/14
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::collections::HashMap;
use std::collections::HashSet;

type LetterPair = (char, char);
type PairCount = HashMap<LetterPair, u64>;

// A polymer is a string like "NCNBCHB".
#[derive(Clone, Debug, PartialEq)]
struct Polymer {
    counts: PairCount,
    last:   char,
}

impl Polymer {
    fn new(line: &str) -> Polymer {
        let mut count = PairCount::new();
        let ch: Vec<char> = line.trim().chars().collect();
        for n in 1..ch.len() {
            let pair = (ch[n-1], ch[n]);
            *count.entry(pair).or_insert(0) += 1;
        }
        Polymer {counts:count, last:*ch.last().unwrap()}
    }

    fn empty(prev: &Polymer) -> Polymer {
        let empty = PairCount::new();
        Polymer {counts:empty, last:prev.last}
    }

    fn len(&self) -> u64 {
        self.counts.values().sum::<u64>() + 1
    }

    fn count(&self, ch: &char) -> u64 {
        let mut count = 0u64;
        if *ch == self.last {count += 1;}
        for (key,val) in self.counts.iter() {
            if *ch == key.0 {count += val;}
        }
        count
    }

    fn letters(&self) -> HashSet<char> {
        let mut lt = HashSet::new();
        lt.insert(self.last);
        for key in self.counts.keys() {lt.insert(key.0);}
        lt
    }

    fn score(&self) -> u64 {
        let lt = self.letters();
        let cmin = lt.iter().map(|c| self.count(c)).min().unwrap();
        let cmax = lt.iter().map(|c| self.count(c)).max().unwrap();
        cmax - cmin
    }
}


// A rule maps an input pair into two output pairs.
// "CH -> B" implies each "CH" becomes a "CB" and a "BH".
struct Rule {
    input:  LetterPair,
    out1:   LetterPair,
    out2:   LetterPair,
}

impl Rule {
    fn new(line: &str) -> Rule {
        let ch: Vec<char> = line.trim().chars().collect();
        assert_eq!(ch.len(), 7);
        Rule {                          // Example "CH -> B"
            input:  (ch[0], ch[1]),     //  "CH"
            out1:   (ch[0], ch[6]),     //  "CB"
            out2:   (ch[6], ch[1]),     //  "BH"
        }
    }
}

struct RuleSet {
    init:   Polymer,
    rules:  Vec<Rule>,
}

impl RuleSet {
    fn new(filename: &str) -> RuleSet {
        let lines = common::read_lines(filename);
        assert!(lines.len() >= 3);
        RuleSet {
            init:   Polymer::new(&lines[0]),
            rules:  lines[2..].iter().map(|l| Rule::new(&l)).collect(),
        }
    }

    fn iterate(&self, iters: usize) -> Polymer {
        let mut prev = self.init.clone();
        for _n in 0..iters {
            let mut next = Polymer::empty(&prev);
            for rule in self.rules.iter() {
                if let Some(incr) = prev.counts.get(&rule.input) {
                    *next.counts.entry(rule.out1).or_insert(0) += incr;                    *next.counts.entry(rule.out2).or_insert(0) += incr;
                }
            }
            prev = next;
        }
        return prev
    }
}

pub fn solve() {
    let test = RuleSet::new("input/test14.txt");
    let data = RuleSet::new("input/input14.txt");

    assert_eq!(test.iterate(0), Polymer::new("NNCB"));
    assert_eq!(test.iterate(1), Polymer::new("NCNBCHB"));
    assert_eq!(test.iterate(2), Polymer::new("NBCCNBBBCBHCB"));
    assert_eq!(test.iterate(3), Polymer::new("NBBBCNCCNBBNBNBBCHBHHBCHB"));
    assert_eq!(test.iterate(4), Polymer::new("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"));
    assert_eq!(test.iterate(5).len(), 97);
    let test10 = test.iterate(10);
    assert_eq!(test10.len(), 3073);
    assert_eq!(test10.count(&'B'), 1749);
    assert_eq!(test10.count(&'C'), 298);
    assert_eq!(test10.count(&'H'), 161);
    assert_eq!(test10.count(&'N'), 865);
    assert_eq!(test10.score(), 1588);
    let test40 = test.iterate(40);
    assert_eq!(test40.count(&'B'), 2192039569602);
    assert_eq!(test40.count(&'H'), 3849876073);
    assert_eq!(test40.score(), 2188189693529);

    println!("Part1: {}", data.iterate(10).score());
    println!("Part2: {}", data.iterate(40).score());
}
