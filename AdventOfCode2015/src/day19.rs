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
    rinv: HashMap<Chem, Vec<u32>>,      // Inverse rules table
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
            rinv: HashMap::new(),
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
                result.rules.entry(ll[0]).or_insert(Vec::new()).push(rr.clone());
                result.rinv.entry(rr).or_insert(Vec::new()).push(ll[0]);
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

    // Get a list of matching predicates from the inverse rules.
    fn inverse_match(&self, input: &Chem, posn: usize) -> Vec<Chem> {
        let mut result = Vec::new();
        for target in self.rinv.keys() {
            if posn + target.len() <= input.len() {
                let all_match = target.iter().enumerate()
                    .all(|(n,x)| input[posn+n] == *x);
                if all_match {result.push(target.clone());}
            }
        }
        return result;
    }
    
    // Convert chemical string to its vector representation.
    fn str2chem(&self, input: &str) -> Chem {
        self.atoms(input).iter()
            .map(|x| *self.atoms.get(x).unwrap())
            .collect()
    }

    // Convert vector representation to a chemical string.
    // (Useful for debugging, but not used in final version.)
    #[allow(dead_code)]
    fn chem2str(&self, input: &Chem) -> String {
        input.iter()
            .map(|x| self.symbs.get(x).unwrap())
            .cloned().collect()
    }

    // Return the set of all compounds reachable from a given state.
    fn forward(&self, chem: &Chem) -> State {
        let mut result = State::new();
        // For each position in the current chemical...
        for n in 0..chem.len() {
            // Apply each matching rule to replace the designated atom.
            if let Some(rule) = self.rules.get(&chem[n]) {
                for output in rule.iter() {
                    let mut new_chem = Chem::new();
                    new_chem.extend_from_slice(&chem[0..n]);
                    new_chem.extend(output);
                    new_chem.extend_from_slice(&chem[n+1..]);
                    result.insert(new_chem);
                }
            }
        }
        return result;
    }

    // Return the set of all compounds that could produce a given state.
    fn reverse(&self, chem: &Chem) -> State {
        // For each position in the current chemical...
        let mut result = State::new();
        for n in 0..chem.len() {
            // For each matching inverse rule...
            for rule in self.inverse_match(chem, n).iter() {
                // Replace the matching section with each possible precursor.
                for input in self.rinv[rule].iter() {
                    let mut new_chem = Chem::new();
                    new_chem.extend_from_slice(&chem[0..n]);
                    new_chem.push(*input);
                    new_chem.extend_from_slice(&chem[n+rule.len()..]);
                    result.insert(new_chem);
                }
            }
        }
        return result;
    }
}

fn shortest(state: &State) -> Chem {
    let mut best_len = usize::MAX;
    let mut best_chem = Vec::new();
    for chem in state.iter() {
        if chem.len() < best_len {
            best_len = chem.len();
            best_chem = chem.clone();
        }
    }
    return best_chem;
}

fn part1(input: &str) -> usize {
    let (rules, init) = Rules::new(input);
    return rules.forward(&init).len();
}

fn part2(input: &str) -> usize {
    let (rules, target) = Rules::new(input);
    let init = rules.str2chem("e");
    let mut count = 0usize;
    let mut state = State::from([target]);
    // Greedy algorithm: Each iteration only keeps the shortest option.
    // TODO: This may not work for all possible inputs?
    while !state.contains(&init) {
        count += 1;
        state = rules.reverse(&shortest(&state));
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
