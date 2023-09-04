/// Advent of Code 2015, Day 19
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashMap;
use std::collections::HashSet;

type Chem = Vec<u32>;                   // A chemical is a list of atoms
type State = HashSet<Chem>;             // Set of all possible chemicals

struct Rules {                          // Rules for manipulating chemicals
    atoms: HashMap<String, u32>,        // Map symbol to index
    symbs: HashMap<u32, String>,        // Map index to symbol
    rules: HashMap<u32, Vec<Chem>>,     // Possible actions by input
}

impl Rules {
    // Given a complete problem input, return ruleset and specified chemical.
    // (Note: Chemical may be an initial state or a desired final state.)
    fn new(input: &str) -> (Rules, Chem) {
        // First pass stores all the atomic symbols.
        let mut result = Rules {
            atoms: HashMap::new(),
            symbs: HashMap::new(),
            rules: HashMap::new(),
        };
        for line in input.trim().lines() {
            for atom in result.atoms(line).into_iter() {
                if !result.atoms.contains_key(&atom) {
                    let new_idx = result.atoms.len() as u32;
                    result.atoms.insert(atom.clone(), new_idx);
                    result.symbs.insert(new_idx, atom.clone());
                }
            }
        }
        // Second pass stores the rules .
        let mut chem = Chem::new();
        for line in input.trim().lines() {
            if line.contains(" => ") {
                let words: Vec<&str> = line.trim().split(' ').collect();
                let ll = result.str2chem(words[0]);
                let rr = result.str2chem(words[2]);
                result.rules.entry(ll[0]).or_insert(Vec::new()).push(rr);
            } else {
                chem = result.str2chem(line);
            }
        }
        return (result, chem);
    }

    // Get a list of atomic labels in a given string.
    // Atomic symbols may be one character ("B", "F") or two ("Al", "Ar").
    fn atoms(&self, input: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut tmp = String::new();
        for ch in input.trim().chars() {
            if ch.is_lowercase() {              // Continue current atom?
                tmp.push(ch);
            } else if ch.is_uppercase() {       // Start a new atom?
                if tmp.len() > 0 {result.push(tmp);}
                tmp = String::from(ch);
            }
        }
        if tmp.len() > 0 {result.push(tmp);}    // Final atom in string
        return result;
    }
    
    // Convert chemical string to its vector representation.
    fn str2chem(&self, input: &str) -> Chem {
        self.atoms(input).iter()
            .map(|x| *self.atoms.get(x).unwrap())
            .collect()
    }

    // Convert vector representation to a chemical string.
    #[allow(dead_code)]
    fn chem2str(&self, input: &Chem) -> String {
        input.iter()
            .map(|x| self.symbs.get(x).unwrap())
            .cloned().collect()
    }

    // Return the set of all compounds reachable from a given state.
    fn iter(&self, state: &State, maxlen: usize) -> State {
        // For each initial state...
        let mut result = State::new();
        for chem in state.iter() {
            // For each position in the current chemical...
            for n in 0..chem.len() {
                // Apply each matching rule to replace the designated atom.
                if let Some(rule) = self.rules.get(&chem[n]) {
                    for output in rule.iter() {
                        let mut new_chem = Chem::new();
                        new_chem.extend_from_slice(&chem[0..n]);
                        new_chem.extend(output);
                        new_chem.extend_from_slice(&chem[n+1..]);
                        if new_chem.len() <= maxlen {
                            result.insert(new_chem);
                        }
                    }
                }
            }
        }
        return result;
    }
}

fn part1(input: &str) -> usize {
    let (rules, init) = Rules::new(input);
    let state = State::from([init]);
    return rules.iter(&state, usize::MAX).len();
}

fn part2(input: &str) -> usize {
    let (rules, target) = Rules::new(input);
    let mut count = 0usize;
    let mut state = State::new();
    state.insert(rules.str2chem("e"));
    while !state.contains(&target) {
        count += 1;
        state = rules.iter(&state, target.len());
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
