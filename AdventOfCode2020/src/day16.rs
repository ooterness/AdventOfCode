/// Day 16: https://adventofcode.com/2020/day/16
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
#[path = "common.rs"] mod common;

/// "Range" is a single contiguous block of allowed integers.
#[derive(Clone)]
struct Range(i64, i64);

impl Range {
    /// Create a Range from a string like "13-40"
    fn compile(s: &str) -> Result<Range, String> {
        let ss:Vec<&str> = s.trim().split('-').collect();
        if let Ok(x) = ss[0].parse::<i64>() {
            if let Ok(y) = ss[1].parse::<i64>() {
                return Ok(Range(x,y));
            }
        }
        Err(String::from("Range error") + s)
    }

    /// Test if an integer falls within specified range.
    fn check_value(&self, x:i64) -> bool {
        (self.0 <= x) && (x <= self.1)
    }
}

/// "Field" is a named set of allowed value-ranges.
#[derive(Clone)]
struct Field {
    name: String,
    rules: Vec<Range>,
}
type Fields = HashMap<String,Field>;

impl Field {
    /// Create a Field from a named set of rules like "seat: 13-40 or 45-50"
    fn compile(s: &String) -> Result<Field, String> {
        // Split name from the ruleset using the ":"
        let ss:Vec<&str> = s.split(':').collect();
        if ss.len() < 2 {
            return Err(String::from("Field error") + s);
        }
        // Split individual ranges on the "or" token.
        let name = String::from(ss[0]);
        let mut rules:Vec<Range> = Vec::new();
        for rr in ss[1].split(" or ") {
            let rule = Range::compile(rr)?;
            rules.push(rule);
        }
        Ok(Field {name:name, rules:rules})
    }

    /// Test if an integer falls within any sub-range.
    fn check_value(&self, x:i64) -> bool {
        let mask = self.rules.iter().map(|r| r.check_value(x));
        common::count_true(mask) > 0
    }

    /// Does this field start with the word "departure"?
    fn is_departure(&self) -> bool {
        self.name.starts_with("departure")
    }

    /// Does this rule match against Nth field in every ticket?
    fn check_tickets(&self, n:usize, tickets:&Vec<Ticket>) -> bool {
        tickets.iter().all(|t| self.check_value(t.vals[n]))
    }

    /// Call check_tickets() for each field index, return legal ones.
    fn valid_indices(&self, tickets:&Vec<Ticket>) -> Vec<usize> {
        let mut indices = Vec::new();
        if tickets.len() > 0 {
            for n in 0..tickets[0].vals.len() {
                if self.check_tickets(n, tickets) {indices.push(n);}
            }
        }
        indices
    }
}

/// "Ticket" is a set of fields with unknown labels.
#[derive(Clone)]
struct Ticket {
    vals: Vec<i64>,
}

impl Ticket {
    /// Create a Ticket from a comma-delimited list of values.
    fn compile(line: &String) -> Result<Ticket, String> {
        let mut vals:Vec<i64> = Vec::new();
        for s in line.split(',') {
            if let Ok(n) = s.parse::<i64>() {
                vals.push(n);
            } else {
                return Err(String::from("Ticket error") + line);
            }
        }
        Ok(Ticket{vals:vals})
    }

    /// Return sum of all invalid fields.
    fn invalid_sum(&self, rules:&Fields) -> Option<i64> {
        let mut err = false;
        let mut sum = 0i64;
        for val in self.vals.iter() {
            let mask = rules.values().map(|r| r.check_value(*val));
            let ok   = common::count_true(mask);
            if ok == 0 {        // Any matching range?
                err = true;
                sum += val;
            } 
        }
        if err {Some(sum)} else {None}
    }

    /// Filter for valid tickets.
    fn valid_filter(&self, rules:&Fields) -> Option<Ticket> {
        match self.invalid_sum(rules) {
            Some(_) => None,
            None    => Some(self.clone()),
        }
    }
}

/// "Problem" is a set of fields, your ticket, and nearby tickets.
struct Problem {
    fields: Fields,
    ticket: Ticket,
    nearby: Vec<Ticket>,
}

impl Problem {
    /// Parse a full description into Fields and Tickets.
    fn compile(lines: &Vec<String>) -> Result<Problem, String> {
        let grp = common::group_strings(lines);
        let fields = Problem::compile_fields(&grp[0])?;
        let ticket = Ticket::compile(&grp[1][1])?;
        let nearby = Problem::compile_tickets(&grp[2])?;
        Ok(Problem{fields:fields, ticket:ticket, nearby:nearby})
    }

    // Helper functions for compile:
    fn compile_fields(lines: &Vec<String>) -> Result<Fields, String> {
        let mut fields = HashMap::new();
        for line in lines {     // Parse each input line
            let field = Field::compile(line)?;
            fields.insert(field.name.clone(), field);
        }
        Ok(fields)
    }
    fn compile_tickets(lines: &Vec<String>) -> Result<Vec<Ticket>, String> {
        let mut tickets:Vec<Ticket> = Vec::new();
        for (n,line) in lines.iter().enumerate() {
            if n > 0 {          // Skip the label line
                let ticket = Ticket::compile(line)?;
                tickets.push(ticket);
            }
        }
        Ok(tickets)
    }

    /// Ticket scanning error rate (per problem statement)
    fn scan_errors(&self) -> i64 {
        self.nearby.iter().filter_map(|t| t.invalid_sum(&self.fields)).sum()
    }

    /// Remove invalid scans.
    fn remove_errors(&self) -> Problem {
        let ok = self.nearby.iter().filter_map(|t| t.valid_filter(&self.fields));
        Problem {
            fields: self.fields.clone(),
            ticket: self.ticket.clone(),
            nearby: ok.collect(),
        }
    }

    /// Given name-to-index mapping, find departure-product.
    fn departure(&self, soln:&Permuter) -> Result<i64, String> {
        // For each departure-related field, lookup value in our ticket.
        let mut accum:i64 = 1i64;
        for field in self.fields.values() {
            if field.is_departure() {
                if let Some(idx) = soln.locked.get(&field.name) {
                    accum *= self.ticket.vals[*idx];
                } else {
                    return Err(String::from("Unlocked ") + &field.name);
                }
            }
        }
        Ok(accum)
    }
}

/// "Permuter" narrows down a list of possible name-to-index permutations.
struct Permuter {
    verbose: bool,                          // Print extra diagnostics?
    nitems: usize,                          // Number of items being reordered
    locked: HashMap<String,usize>,          // Index of known fixed values
    unlock: HashMap<String,HashSet<usize>>, // List of unknown possibilities
}

impl Permuter {
    fn create(problem:&Problem, verbose:bool) -> Permuter {
        // Create a new empty Permuter:
        let mut new = Permuter {
            verbose: verbose,
            nitems: problem.ticket.vals.len(),
            locked: HashMap::new(),
            unlock: HashMap::new(),
        };
        // For each named field, set initial possibility space
        // based on which indices match all nearby tickets.
        for (lbl,field) in problem.fields.iter() {
            let valid = field.valid_indices(&problem.nearby);
            if verbose {common::print_list(&lbl, valid.iter().cloned());}
            let set = HashSet::from_iter(valid.iter().cloned());
            new.unlock.insert(lbl.clone(), set);
        }
        // Normalize this possibility space.
        new.normalize(); new
    }

    /// Find number of remaining permuations.
    fn count_permutations(&self) -> u64 {
        let mut perm = 1u64;
        for set in self.unlock.values() {
            perm *= set.len() as u64;
        }
        perm
    }

    /// Find a lockable rule, if possible.
    fn next_lockable(&self) -> Option<(String,usize)> {
        for (lbl,set) in self.unlock.iter() {
            if set.len() == 1 {
                let val:usize = *set.iter().next().unwrap();
                return Some((lbl.clone(), val));
            }
        }
        None
    }

    /// Lock down the specified label-index pair.
    fn lock(&mut self, lbl:String, val:usize) {
        assert!(val < self.nitems);     // Sanity check on index and size
        assert!(self.locked.len() + self.unlock.len() == self.nitems);
        self.unlock.remove(&lbl);       // Remove field from unlocked list
        for set in self.unlock.values_mut() {
            set.remove(&val);           // Remove index from other sets
        }
        if self.verbose {               // Additional diagnostics?
            println!("Locking {} = #{} (rem {} -> {})",
                &lbl, &val, self.unlock.len(), self.count_permutations());
        }
        self.locked.insert(lbl, val);   // Add the newly-locked field
    }

    /// Repeatedly lock down obvious possibilities.
    fn normalize(&mut self) {
        loop {
            if let Some((lbl,val)) = self.next_lockable() {
                self.lock(lbl, val);
            } else {break}
        }
    }
}

/// Run example tests, then solve Part 1 and Part 2.
pub fn solve() {
    let example1:Vec<String> = vec![
        String::from("class: 1-3 or 5-7"),
        String::from("row: 6-11 or 33-44"),
        String::from("seat: 13-40 or 45-50"),
        String::from(""),
        String::from("your ticket:"),
        String::from("7,1,14"),
        String::from(""),
        String::from("nearby tickets:"),
        String::from("7,3,47"),
        String::from("40,4,50"),
        String::from("55,2,20"),
        String::from("38,6,12"),
    ];

    let example2:Vec<String> = vec![
        String::from("class: 0-1 or 4-19"),
        String::from("row: 0-5 or 8-19"),
        String::from("seat: 0-13 or 16-19"),
        String::from(""),
        String::from("your ticket:"),
        String::from("11,12,13"),
        String::from(""),
        String::from("nearby tickets:"),
        String::from("3,9,18"),
        String::from("15,1,5"),
        String::from("5,14,9"),
    ];

    // Parse each example and confirm expected outputs.
    if let Ok(test1) = Problem::compile(&example1) {
        assert_eq!(test1.scan_errors(), 71);
    } else {
        eprintln!("Test1: Parse error!");
    }

    if let Ok(test2) = Problem::compile(&example2) {
        let test2 = test2.remove_errors();
        let soln = Permuter::create(&test2, false);
        assert_eq!(soln.count_permutations(), 1u64);
        assert_eq!(soln.locked.get("row"),   Some(&0usize));
        assert_eq!(soln.locked.get("class"), Some(&1usize));
        assert_eq!(soln.locked.get("seat"),  Some(&2usize));
    } else {
        eprintln!("Test1: Parse error!");
    }

    // Parse and analyze the main input.
    // TODO: Solve general cases where permuter doesn't fully converge.
    let input = common::read_strings("input/input16.txt");
    if let Ok(part1) = Problem::compile(&input) {
        println!("Part1: {} scan errors", part1.scan_errors());
        let part1 = part1.remove_errors();
        let part2 = Permuter::create(&part1, false);
        println!("Part2: Remaining permutations {}", part2.count_permutations());
        match part1.departure(&part2) {
            Ok(val) => println!("Part2: Product = {}", val),
            Err(msg) => eprintln!("Part2: {}", msg),
        }
    } else {
        eprintln!("Part1: Parse error!");
    }
}
