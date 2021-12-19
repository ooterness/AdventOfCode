/// Day 17: https://adventofcode.com/2021/day/17
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::fmt;

const VERBOSE: bool = false;

// Find the top-level comma for a string of the form "[?,?]".
fn find_comma(x: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (n,ch) in x.chars().enumerate() {
        if depth == 1 && ch == ',' {
            return Some(n); // Found split point
        } else if ch == '[' {
            depth += 1;     // Increase nesting
        } else if ch == ']' && depth == 0 {
            return None;    // Invalid string
        } else if ch == ']' {
            depth -= 1;     // Reduce nesting
        }
    }
    return None;            // Invalid string
}

// An "Item" is either a Simple number of a nested Pair.
#[derive(Clone, Eq, PartialEq)]
enum Item {
    Simple(u64),
    Nested(Box<Pair>),
}

impl Item {
    fn new(x: &str) -> Option<Item> {
        if let Some(n) = x.parse::<u64>().ok() {
            Some(Item::Simple(n))
        } else if let Some(n) = find_comma(x) {
            let l = Item::new(&x[1..n]).unwrap();
            let r = Item::new(&x[n+1..x.len()-1]).unwrap();
            Some(Item::Nested(Box::new(Pair {l:l, r:r})))
        } else {
            None
        }
    }

    // Explode this item, if applicable.
    //  * Return Some((l,r)) if an explosion occurs.
    fn explode(&mut self, d:usize) -> Option<(u64,u64)> {
        if let Item::Nested(pair) = self {
            if d >= 3 {
                let l = pair.l.value();
                let r = pair.r.value();
                *self = Item::Simple(0);
                Some((l,r))                 // Explosion
            } else {pair.explode(d+1)}      // Propagate up
        } else {None}                       // Simple value
    }

    // Split this item, if applicable. (Return true if modified.)
    fn split(&mut self) -> bool {
        let x = self.value();
        if x >= 10 {
            *self = Item::Nested(Box::new(Pair::from_split(x))); true
        } else { match self {
            Item::Simple(_) => false,
            Item::Nested(p) => p.split(),
        } }
    }

    // Increment the leftmost or rightmost simple value.
    // Returns unused residue if propagation should continue.
    fn incr_left(&mut self, n:u64) -> u64 {
        match self {
            Item::Simple(x) => {*x += n; 0},    // Increment applied
            Item::Nested(p) => p.l.incr_left(n),
        }
    }
    fn incr_right(&mut self, n:u64) -> u64 {
        match self {
            Item::Simple(x) => {*x += n; 0},    // Increment applied
            Item::Nested(p) => p.r.incr_right(n),
        }
    }

    // Value of a simple element, leftmost value, or rightmost value.
    fn value(&self) -> u64 {
        if let Item::Simple(x) = self {*x} else {0}
    }

    // Find "magnitude" using leftmost and rightmost numbers.
    fn magnitude(&self) -> u64 {
        match self {
            Item::Simple(x) => *x,
            Item::Nested(p) => p.magnitude(),
        }
    }
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Simple(x) => write!(f, "{:?}", x),
            Item::Nested(p) => write!(f, "[{:?},{:?}]", p.l, p.r),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Pair {
    l: Item,    // Left component
    r: Item,    // Right component
}

impl Pair {
    fn new(line: &str) -> Pair {
        let item = Item::new(line).unwrap();
        if let Item::Nested(pair) = item {
            *pair
        } else {
            panic!("Invalid expression!")
        }
    }

    fn from_split(x: u64) -> Pair {
        let l = Item::Simple((x+0) / 2);    // Round down
        let r = Item::Simple((x+1) / 2);    // Round up
        Pair { l:l, r:r }
    }

    // Explode this pair, if applicable.
    //  * Return Some((l,r)) if an explosion occurs.
    fn explode(&mut self, d:usize) -> Option<(u64,u64)> {
        // Will either nested item explode? Check left first.
        if let Some((l,r)) = self.l.explode(d) {
            assert_eq!(self.r.incr_left(r), 0);
            Some((l,0))     // Continue propagating leftward
        } else if let Some((l,r)) = self.r.explode(d) {
            assert_eq!(self.l.incr_right(l), 0);
            Some((0,r))     // Continue propagating rightward
        } else {
            None
        }
    }

    // Split this pair, if applicable. (Return true if applied.)
    fn split(&mut self) -> bool {
        self.l.split() || self.r.split()
    }

    // Reduce this top-level expression.
    fn reduce(&self) -> Pair {
        if VERBOSE {println!("Original: {:?}", self);}
        let mut result = self.clone();
        while result.explode(0).is_some() || result.split() {
            if VERBOSE {println!("Reduced: {:?}", result);}
        }
        result
    }

    // Add two Pairs and reduce.
    fn add(&self, other: &Pair) -> Pair {
        let l = Item::Nested(Box::new(self.clone()));
        let r = Item::Nested(Box::new(other.clone()));
        Pair{l:l,r:r}.reduce()
    }

    // Find "magnitude" using leftmost and rightmost numbers.
    fn magnitude(&self) -> u64 {
        return 3 * self.l.magnitude() + 2 * self.r.magnitude()
    }
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?},{:?}]", self.l, self.r)
    }
}

fn sum<'a>(mut iter: impl Iterator<Item=&'a Pair>) -> Pair {
    let mut sum = iter.next().unwrap().clone();
    while let Some(x) = iter.next() {
        sum = sum.add(x);
    }
    sum
}

fn largest_mag(data: &Vec<Pair>) -> u64 {
    // Try every pairwise summation.
    // (Note that A+B != B+A, so we must try both options.)
    let mut best = 0u64;
    for a in 0..data.len() {
        for b in 0..data.len() {
            if a == b {continue}
            let next = data[a].add(&data[b]).magnitude();
            best = core::cmp::max(best, next);
        }
    }
    best
}

fn read_file(filename: &str) -> Vec<Pair> {
    let lines = common::read_lines(filename);
    lines.iter().map(|x| Pair::new(x)).collect()
}

pub fn solve() {
    // Test each of the reduction examples.
    assert_eq!(
        Pair::new("[[[[[9,8],1],2],3],4]").reduce(),
        Pair::new("[[[[0,9],2],3],4]"));
    assert_eq!(
        Pair::new("[7,[6,[5,[4,[3,2]]]]]").reduce(),
        Pair::new("[7,[6,[5,[7,0]]]]"));
    assert_eq!(
        Pair::new("[[6,[5,[4,[3,2]]]],1]").reduce(),
        Pair::new("[[6,[5,[7,0]]],3]"));
    assert_eq!(
        Pair::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").reduce(),
        Pair::new("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));

    // Test the simple addition example.
    assert_eq!(
        Pair::new("[[[[4,3],4],4],[7,[[8,4],9]]]").add(&Pair::new("[1,1]")),
        Pair::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

    // Test each list-summation example.
    let test1 = vec!(
        Pair::new("[1,1]"), Pair::new("[2,2]"), Pair::new("[3,3]"),
        Pair::new("[4,4]"), Pair::new("[5,5]"), Pair::new("[6,6]"));
    let test2 = read_file("input/test18a.txt");
    let test3 = read_file("input/test18b.txt");

    assert_eq!(sum(test1[0..4].iter()),
        Pair::new("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
    assert_eq!(sum(test1[0..5].iter()),
        Pair::new("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
    assert_eq!(sum(test1[0..6].iter()),
        Pair::new("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    assert_eq!(sum(test2[0..2].iter()),
        Pair::new("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"));
    assert_eq!(sum(test2[0..3].iter()),
        Pair::new("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"));
    assert_eq!(sum(test2[0..4].iter()),
        Pair::new("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"));
    assert_eq!(sum(test2[0..5].iter()),
        Pair::new("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"));
    assert_eq!(sum(test2[0..6].iter()),
        Pair::new("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"));
    assert_eq!(sum(test2[0..7].iter()),
        Pair::new("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"));
    assert_eq!(sum(test2[0..8].iter()),
        Pair::new("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"));
    assert_eq!(sum(test2[0..9].iter()),
        Pair::new("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"));
    assert_eq!(sum(test2.iter()),
        Pair::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
    assert_eq!(sum(test3.iter()),
        Pair::new("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"));

    // Example magnitude calculations.
    assert_eq!(143,
        Pair::new("[[1,2],[[3,4],5]]").magnitude());
    assert_eq!(1384,
        Pair::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude());
    assert_eq!(445,
        Pair::new("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude());
    assert_eq!(791,
        Pair::new("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude());
    assert_eq!(1137,
        Pair::new("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude());
    assert_eq!(3488,
        Pair::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude());
    assert_eq!(4140, sum(test3.iter()).magnitude());

    // Solve the Part-1 homework problem.
    let data = read_file("input/input18.txt");
    println!("Part1: {}", sum(data.iter()).magnitude());

    // Solve the Part-2 homework problem.
    assert_eq!(3993, largest_mag(&test3));
    println!("Part2: {}", largest_mag(&data));
}
