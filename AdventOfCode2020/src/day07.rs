/// Day 7: https://adventofcode.com/2020/day/7
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
#[path = "common.rs"] mod common;

struct Rule(HashMap <String, usize>);   // Color -> #Bags that color
struct Rules(HashMap <String, Rule>);   // Color -> Child rule(s)

fn make_color(x:&str, y:&str) -> String {
    String::from(x) + y
}

impl Rule {
    fn new() -> Rule {
        Rule(HashMap::new())
    }

    /// Read a single-line descriptor and parse the Rule.
    fn read(line:&String) -> Option<(String,Rule)> {
        // Split line by spaces:
        // e.g., "light red bags contain 1 bright white bag, 2 muted yellow bags."
        //        00000 111 2222 3333333 4 555555 66666 7777 8 99999 000000 11111
        let words:Vec<&str> = line.split(' ').collect();
        // Sanity check for invalid lines:
        if words.len() < 7 {return None;}           // Too short to parse
        // Parse each child...
        let nchild = (words.len() - 3) / 4;         // Number of children
        let bcolor = make_color(words[0], words[1]);
        let mut children = Rule::new();
        for n in 0..nchild {
            let nc = words[4*n+4].parse::<usize>(); // #Bags for this child
            let cc = make_color(words[4*n+5], words[4*n+6]);
            if let Ok(nc) = nc {
                children.0.insert(cc, nc);          // Add to child-list
            } else {
                return None;                        // Failed to parse qty
            }
        }
        Some((bcolor, children))
    }

    fn count(&self, clr:&String) -> usize {
        match self.0.get(clr) {
            None => 0,      // No such child
            Some(x) => *x,  // Return qty
        }
    }
}

impl Rules {
    fn from(raw:&Vec<String>) -> Rules {
        let mut rules = Rules(HashMap::new());
        for line in raw {
            if let Some(pair) = Rule::read(line) {
                rules.0.insert(pair.0, pair.1);
            }
        }
        return rules
    }

    fn can_contain(&self, outer:&String, inner:&String) -> bool {
        // Lookup rule by color-name.
        if let Some(rule) = self.0.get(outer) {
            // Does this bag-type contain target directly?
            if rule.count(inner) > 0 {return true;}
            // Otherwise, recurse on all sub-types.
            for color in rule.0.keys() {
                if self.can_contain(color, inner) {return true;}
            }
            false   // No match in any descendant.
        } else {
            false   // No such color defined.
        }
    }

    fn can_contain_gold(&self) -> usize {
        let gold = String::from("shinygold");
        let test = |x| self.can_contain(x, &gold);
        common::count_true(self.0.keys().map(test))
    }

    fn count_child(&self, outer:&String) -> u64 {
        if let Some(rule) = self.0.get(outer) {
            let mut total:u64 = 1u64;   // Start with outer bag
            for inner in rule.0.keys() {
                total += rule.count(inner) as u64
                       * self.count_child(inner)
            }
            total   // Total nested bags
        } else {
            1u64    // Outer bag only, no children
        }
    }

    fn count_child_gold(&self) -> u64 {
        self.count_child(&String::from("shinygold")) - 1u64
    }
}


/// Solve Part-1 and Part-2 of the problem statement.
pub fn solve() {
    // Parse the example input.
    let test1 = Rules::from(&vec![
        String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
        String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
        String::from("bright white bags contain 1 shiny gold bag."),
        String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
        String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
        String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
        String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
        String::from("faded blue bags contain no other bags."),
        String::from("dotted black bags contain no other bags."),
    ]);
    let test2 = Rules::from(&vec![
        String::from("shiny gold bags contain 2 dark red bags."),
        String::from("dark red bags contain 2 dark orange bags."),
        String::from("dark orange bags contain 2 dark yellow bags."),
        String::from("dark yellow bags contain 2 dark green bags."),
        String::from("dark green bags contain 2 dark blue bags."),
        String::from("dark blue bags contain 2 dark violet bags."),
        String::from("dark violet bags contain no other bags."),
    ]);
    println!("Test 1: {} colors can contain gold.", test1.can_contain_gold());
    println!("Test 2: {}/32 nested bags.", test1.count_child_gold());
    println!("Test 3: {}/126 nested bags.", test2.count_child_gold());

    // Read and analyze the main input.
    let input_str = common::read_strings("input/input07.txt");
    let input = Rules::from(&input_str);
    println!("Part 1: {} colors can contain gold.", input.can_contain_gold());
    println!("Part 2: {} nested bags.", input.count_child_gold());
}
