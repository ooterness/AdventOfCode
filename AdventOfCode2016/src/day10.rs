/// Advent of Code 2016, Day 10
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Target {
    Bin(usize),                     // Target = Bin #N
    Bot(usize),                     // Target = Bot #N
}

type List = Vec<usize>;             // A list of token values
struct Transfer(usize, Target);     // Token in transit (value, dest)

struct Bot {
    label: usize,                   // Index for this robot
    rule: (Target, Target),         // Give lo/hi to designated index
    held: Option<usize>,            // Currently held chip, if any
}

struct Sim {
    bins: HashMap<usize, List>,     // Output bins by index
    bots: HashMap<usize, Bot>,      // Robots by index
    queue: Vec<Transfer>,           // Items in transit
    part1: Option<usize>,           // Which bot compares 17 & 61?
}

impl Target {
    fn parse(typ: &str, dst: &str) -> Self {
        let idx: usize = dst.parse().unwrap();
        return if typ == "bot" {Target::Bot(idx)} else {Target::Bin(idx)};
    }
}

impl Transfer {
    fn parse(line: &str) -> Self {
        let tokens: Vec<&str> = line.trim().split(' ').collect();
        let value: usize    = tokens[1].parse().unwrap();
        let target: usize   = tokens[5].parse().unwrap();
        return Transfer(value, Target::Bot(target));
    }
}

impl Bot {
    fn new(line: &str) -> Self {
        let tokens: Vec<&str> = line.trim().split(' ').collect();
        let label: usize    = tokens[1].parse().unwrap();
        let tgt_lo: Target  = Target::parse(&tokens[5], &tokens[6]);
        let tgt_hi: Target  = Target::parse(&tokens[10], &tokens[11]);
        return Bot { label: label, rule: (tgt_lo, tgt_hi), held: None };
    }

}

impl Sim {
    fn new(rules: &str) -> Self {
        let mut sim = Sim {
            bins: HashMap::new(),
            bots: HashMap::new(),
            queue: Vec::new(),
            part1: None,
        };
        for rule in rules.trim().lines() {
            // Sort rule type using the first letter...
            let ch = rule.chars().nth(0).unwrap();
            if ch == 'v' {  // "value 5 goes to bot 2"
                let tok = Transfer::parse(rule);
                sim.queue.push(tok);
            } else {        // "bot 2 gives low to bot 1 and high to bot 0"
                let bot = Bot::new(rule);
                sim.bots.insert(bot.label, bot);
            }
        }
        return sim;
    }

    fn give_bin(&mut self, bin: usize, token: usize) {
        let bin = self.bins.entry(bin).or_insert(Vec::new());
        bin.push(token);
    }

    fn give_bot(&mut self, bot: usize, token: usize) {
        let mut obj = self.bots.get_mut(&bot).unwrap();
        if let Some(tok_bot) = obj.held {
            // Pass along the new and old tokens.
            let tok_lo = min(token, tok_bot);
            let tok_hi = max(token, tok_bot);
            self.queue.push(Transfer(tok_lo, obj.rule.0));
            self.queue.push(Transfer(tok_hi, obj.rule.1));
            obj.held = None;
            // Was this the special Part-1 transfer?
            if tok_lo == 17 && tok_hi == 61 {
                self.part1 = Some(bot);
            }
        } else {
            // Nothing held -> Bot gets the new token.
            obj.held = Some(token);
        }
    }

    fn step(&mut self) -> bool {
        if let Some(Transfer(tok,tgt)) = self.queue.pop() {
            match tgt {
                Target::Bin(bin) => self.give_bin(bin, tok),
                Target::Bot(bot) => self.give_bot(bot, tok),
            }   
            return true;    // State changed.
        } else {
            return false;   // No actions remaining.
        }
    }
}

fn part1(rules: &str) -> usize {
    // Run simulation until a robot compares the designated tokens.
    let mut sim = Sim::new(rules);
    while sim.step() {
        if let Some(x) = sim.part1 {return x;}
    }
    return 0;
}

fn part2(rules: &str) -> usize {
    // Run simulation to completion.
    let mut sim = Sim::new(rules);
    while sim.step() {}
    // Result is the product of three token values.
    let tok0 = sim.bins.get_mut(&0).unwrap().pop().unwrap();
    let tok1 = sim.bins.get_mut(&1).unwrap().pop().unwrap();
    let tok2 = sim.bins.get_mut(&2).unwrap().pop().unwrap();
    return tok0 * tok1 * tok2;
}

const TEST: &str = "\
value 61 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 17 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 10).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), 0);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
