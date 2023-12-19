/// Advent of Code 2023, Day 19
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;

enum Test {
    Any(),
    Gt(char, usize),
    Lt(char, usize),
}

struct Rule {
    dst: String,
    typ: Test,
}

struct RuleSet {
    label: String,
    rules: Vec<Rule>,
    other: String,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

const VMIN: usize = 1;
const VMAX: usize = 4000;
const VSIZE: usize = 1 + VMAX - VMIN;
type Mask = [bool;VSIZE];

fn mask_and(x: &Mask, y: &Mask) -> Mask {
    core::array::from_fn(|n| x[n] && y[n])
}
fn mask_not(x: &Mask, y: &Mask) -> Mask {
    core::array::from_fn(|n| x[n] && !y[n])
}

#[derive(Clone)]
struct Range {
    x: Mask,
    m: Mask,
    a: Mask,
    s: Mask,
}

struct Work {
    flows: HashMap<String, RuleSet>,
    parts: Vec<Part>,
}

impl Rule {
    fn new(input: &str) -> Self {
        let tok: Vec<&str> = input.trim().split(&['<', '>', ':']).collect();
        if input.contains('>') {
            let ch = tok[0].chars().nth(0).unwrap();
            let val: usize = tok[1].parse().unwrap();
            return Rule { dst:tok[2].to_string(), typ:Test::Gt(ch, val) };
        } else if input.contains('<') {
            let ch = tok[0].chars().nth(0).unwrap();
            let val: usize = tok[1].parse().unwrap();
            return Rule { dst:tok[2].to_string(), typ:Test::Lt(ch, val) };
        } else {
            return Rule { dst:input.to_string(), typ:Test::Any() };
        }
    }

    fn check(&self, part:&Part) -> bool {
        match self.typ {
            Test::Any() => true,
            Test::Gt(ch, val) => part.get(ch) > val,
            Test::Lt(ch, val) => part.get(ch) < val,
        }
    }

    fn split(&self, range:&Range) -> (Range, Range) {
        match self.typ {
            Test::Any() =>
                (Range::none(), range.clone()),
            Test::Gt(ch, val) => {
                let mask: Mask = core::array::from_fn(|n| n+VMIN > val);
                (range.mask0(ch, &mask), range.mask1(ch, &mask))}
            Test::Lt(ch, val) => {
                let mask: Mask = core::array::from_fn(|n| n+VMIN < val);
                (range.mask0(ch, &mask), range.mask1(ch, &mask))}
        }
    }
}

impl RuleSet {
    fn new(input: &str) -> Self {
        let tok: Vec<&str> = input.trim().split(&['{', ',', '}']).collect();
        let label = tok[0].to_string();
        let rules = tok[1..=tok.len()-2].iter().map(|s| Rule::new(*s)).collect();
        let other = tok[tok.len()-1].to_string();
        return RuleSet { label:label, rules:rules, other:other };
    }

    fn check<'a>(&'a self, part: &Part) -> &'a str {
        for rule in self.rules.iter() {
            if rule.check(part) {return &rule.dst;}
        }
        return &self.other;
    }

    fn split<'a>(&'a self, range: Range) -> Vec<(&'a str, Range)> {
        let mut out = Vec::new();
        let mut rem = range;
        for rule in self.rules.iter() {
            let (no,yes) = rule.split(&rem);
            rem = no; out.push((&rule.dst as &str, yes));
        }
        out.push((&self.other as &str, rem));
        return out.into_iter().filter(|r| r.1.combos() > 0).collect();
    }
}

impl Part {
    fn from(input: &str) -> Self {
        let tok: Vec<&str> = input.trim().split(&['{', '=', ',', '}']).collect();
        Part {x: tok[2].parse().unwrap(),
              m: tok[4].parse().unwrap(),
              a: tok[6].parse().unwrap(),
              s: tok[8].parse().unwrap()}
    }

    fn get(&self, ch: char) -> usize {
        match ch {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _   => panic!("No such metric."),
        }
    }

    fn score(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl Range {
    fn all() -> Self {
        Range { x:[true;VSIZE], m:[true;VSIZE], a:[true;VSIZE], s:[true;VSIZE] }
    }

    fn none() -> Self {
        Range { x:[false;VSIZE], m:[false;VSIZE], a:[false;VSIZE], s:[false;VSIZE] }
    }

    fn mask1(&self, ch:char, mask:&Mask) -> Self {
        Range { x: if ch=='x' {mask_and(&self.x, mask)} else {self.x.clone()},
                m: if ch=='m' {mask_and(&self.m, mask)} else {self.m.clone()},
                a: if ch=='a' {mask_and(&self.a, mask)} else {self.a.clone()},
                s: if ch=='s' {mask_and(&self.s, mask)} else {self.s.clone()} }
    }
    fn mask0(&self, ch:char, mask:&Mask) -> Self {
        Range { x: if ch=='x' {mask_not(&self.x, mask)} else {self.x.clone()},
                m: if ch=='m' {mask_not(&self.m, mask)} else {self.m.clone()},
                a: if ch=='a' {mask_not(&self.a, mask)} else {self.a.clone()},
                s: if ch=='s' {mask_not(&self.s, mask)} else {self.s.clone()} }
    }

    fn combos(&self) -> usize {
        let x: usize = self.x.iter().map(|v| if *v {1} else {0}).sum();
        let m: usize = self.m.iter().map(|v| if *v {1} else {0}).sum();
        let a: usize = self.a.iter().map(|v| if *v {1} else {0}).sum();
        let s: usize = self.s.iter().map(|v| if *v {1} else {0}).sum();
        return x * m * a * s;
    }
}

impl Work {
    fn new(input: &str) -> Self {
        let mut work = Work { flows: HashMap::new(), parts: Vec::new() };
        let mut upper = true;
        for line in input.trim().lines() {
            if line.trim().len() == 0 {
                upper = false;  // Everything after this is a "part"
            } else if upper {
                let flow = RuleSet::new(line);
                work.flows.insert(flow.label.clone(), flow);
            } else {
                work.parts.push(Part::from(line));
            }
        }
        return work;
    }

    fn accept(&self, part: &Part) -> usize {
        let mut lbl: &str = "in";
        for _ in 0..self.flows.len() {
            lbl = self.flows[lbl].check(part);
            if lbl == "A" {return part.score();}
            if lbl == "R" {return 0;}
        }
        panic!("Infinite loop?");
    }

    fn score(&self) -> usize {
        self.parts.iter().map(|p| self.accept(p)).sum()
    }

    fn combos(&self) -> usize {
        let mut queue = vec![("in", Range::all())];
        let mut total = 0usize;
        while let Some((lbl, range)) = queue.pop() {
            if lbl == "A" {
                total += range.combos();
            } else if lbl != "R" {
                queue.extend(self.flows[lbl].split(range));
            }
        }
        return total;
    }
}

fn part1(input: &str) -> usize {
    Work::new(input).score()
}

fn part2(input: &str) -> usize {
    Work::new(input).combos()
}

const EXAMPLE: &'static str = "\
    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}\n
    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 19).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 19114);
    assert_eq!(part2(EXAMPLE), 167409079868000);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
