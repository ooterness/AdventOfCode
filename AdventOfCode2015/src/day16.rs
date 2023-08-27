/// Advent of Code 2015, Day 16
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashMap;
use std::collections::HashSet;

struct Aunt {
    label: String,
    info: HashMap<String, u64>,
    mask_gt: HashSet<String>,
    mask_lt: HashSet<String>,
}

impl Aunt {
    fn new(input: &str) -> Self {
        // Split up the input string...
        let words: Vec<&str> = input.trim().split([':', ',']).collect();
        let label = String::from(words[0].trim());
        let count = (words.len() - 1) / 2;
        // Push each label + quantity pair into the "info" map.
        let mut info = HashMap::new();
        for n in 0..count {
            let typ = String::from(words[2*n + 1].trim());
            let qty = words[2*n + 2].trim().parse::<u64>().unwrap();
            info.insert(typ, qty);
        }
        return Aunt {
            label: label, info: info,
            mask_gt: HashSet::new(),
            mask_lt: HashSet::new()
        }
    }

    fn allow_gt(&mut self, typ: &str) {
        self.mask_gt.insert(String::from(typ));
    }

    fn allow_lt(&mut self, typ: &str) {
        self.mask_lt.insert(String::from(typ));
    }

    fn check(&self, typ: &str, qty: u64) -> bool {
        let ref_qty = *self.info.get(typ).unwrap();
        if self.mask_gt.contains(typ) {
            return qty > ref_qty;
        } else if self.mask_lt.contains(typ) {
            return qty < ref_qty;
        } else {
            return qty == ref_qty;
        }
    }

    fn is_match(&self, foo: &Aunt) -> bool {
        // Each item in self.info must be an match to the one in foo.info.
        // (But not vice-versa, since items in self.info may be missing.)
        self.info.iter().all(|(typ,qty)| foo.check(typ, *qty))
    }
}

const SCAN_RESULT: &str =
    "Reference 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

fn part1(input: &str) -> String {
    let mfcsam = Aunt::new(SCAN_RESULT);
    for aunt in input.lines().map(Aunt::new) {
        if aunt.is_match(&mfcsam) {return aunt.label.clone();}
    }
    return String::from("No match");
}

fn part2(input: &str) -> String {
    let mut mfcsam = Aunt::new(SCAN_RESULT);
    mfcsam.allow_gt("cats");
    mfcsam.allow_gt("trees");
    mfcsam.allow_lt("pomeranians");
    mfcsam.allow_lt("goldfish");
    for aunt in input.lines().map(Aunt::new) {
        if aunt.is_match(&mfcsam) {return aunt.label.clone();}
    }
    return String::from("No match");
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 16).unwrap();

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
