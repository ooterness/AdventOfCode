/// Advent of Code 2017, Day 7
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashMap;

struct Program {
    name: String,
    weight: i64,
    children: Vec<String>,
    parent: Option<String>,
    total: i64,
}

impl Program {
    fn new(line: &str) -> Program {
        // Split on the arrow: "fwft (72) -> ktlj, cntj, xhth"
        let split: Vec<&str> = line.split(" -> ").collect();
        // Left half = Name + weight.
        let left: Vec<&str> = split[0].split(" (").collect();
        let name = left[0].to_string();
        let wtlen = left[1].len() - 1;
        let weight = left[1][0..wtlen].parse().unwrap();
        // Right half = Held programs, if any.
        let mut children = vec![];
        if split.len() > 1 {
            for lbl in split[1].split(", ") {
                children.push(lbl.to_string());
            }
        }
        // Leave parent empty for now.
        let parent = None;
        return Program {name, weight, children, parent, total:weight};
    }
}

struct Stack {
    pnames: Vec<String>,
    programs: HashMap<String, Program>,
}

impl Stack {
    fn new(input: &str) -> Stack {
        // Parse the individual descriptions.
        let mut programs = HashMap::new();
        for line in input.lines() {
            let prog = Program::new(line);
            programs.insert(prog.name.clone(), prog);
        }
        // Make a copy of all the program names.
        let pnames: Vec<String> = programs.keys()
            .map(|x| x.clone()).collect();
        // Link parents for each node.
        for parent in pnames.iter() {
            let children = programs.get(parent).unwrap().children.clone();
            for child in children.iter() {
                programs.get_mut(child).unwrap()
                    .parent = Some(parent.clone());
            }
        }
        // Propagate total weights of each subtree.
        for leaf in pnames.iter() {
            let incr = programs[leaf].weight;
            let mut next: String = leaf.clone();
            while let Some(parent) = programs[&next].parent.clone() {
                programs.get_mut(&parent).unwrap().total += incr;
                next = parent;
            }
        }
        return Stack {pnames, programs};
    }

    // Find the name of the root node.
    fn root(&self) -> Option<String> {
        // Find the node with no parent.
        for prog in self.programs.values() {
            if let None = prog.parent { return Some(prog.name.clone()); }
        }
        return None;
    }

    // Is the designated node balanced?
    fn balanced(&self, label: &str) -> bool {
        let child = &self.programs[label].children;
        if child.len() == 0 {
            return true;    // No children -> Always balanced.
        } else {
            let wt_ref = self.programs[&child[0]].total;
            return child.iter().all(
                |x| self.programs[x].total == wt_ref);
        }
    }

    // Find the corrected weight of the unbalanced node.
    fn correction(&self) -> i64 {
        // Find a node where each child is balanced individually,
        // but the children do not have equal weights.
        for leaf in self.pnames.iter() {
            if self.balanced(leaf) {continue;}
            let child = &self.programs[leaf].children;
            if child.iter().all(|x| self.balanced(x)) {
                // Use the first three items to find a consensus.
                assert!(child.len() >= 3);
                let wt_vec: Vec<i64> = child.iter()
                    .map(|x| self.programs[x].total)
                    .collect();
                let wt_ref = if wt_vec[0] == wt_vec[1]
                    {wt_vec[0]} else {wt_vec[2]};
                // Find the mismatched node, then solve for the new
                // node weight to achieve the desired total weight.
                for prog in child.iter().map(|x| &self.programs[x]) {
                    if prog.total != wt_ref {
                        return prog.weight + wt_ref - prog.total;
                    }
                }
            }
        }
        return 0;
    }
}

const TEST: &str = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

fn main() {
    // Fetch problem input from server.
    let test: Stack = Stack::new(&TEST);
    let input: Stack = Stack::new(&fetch::get_data(2017, 7).unwrap());

    // Unit tests on provided example.
    assert_eq!(test.root().unwrap(), "tknk");
    assert_eq!(test.correction(), 60);

    // Solve for real input.
    println!("Part 1: {}", &input.root().unwrap());
    println!("Part 2: {}", &input.correction());
}
