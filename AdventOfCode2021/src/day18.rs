/// Day 17: https://adventofcode.com/2021/day/17
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

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
#[derive(Clone, Debug, Eq, PartialEq)]
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

    fn from_split(x: u64) -> Item {
        Item::Nested(Box::new(Pair::from_split(x)))
    }

    // Explode this item, if applicable. (Returns true if modified.)
    fn explode(&mut self, d:usize) -> bool {
        if let Item::Nested(pair) = self {
            pair.explode(d+1)
        } else {false}
    }

    // Split this item, if applicable. (Return true if modified.)
    fn split(&mut self) -> bool {
        if let Item::Nested(pair) = self {
            pair.split()
        } else {false}
    }

    // Increment a simple value (e.g., as part of "explode")
    fn incr_if_simple(&mut self, n:u64) {
        if let Item::Simple(x) = self {*x += n;}
    }

    // Value of a simple element, leftmost value, or rightmost value.
    fn value(&self) -> u64 {
        if let Item::Simple(n) = self {*n} else {0}
    }

    // Find "magnitude" using leftmost and rightmost numbers.
    fn magnitude(&self) -> u64 {
        match self {
            Item::Simple(n) => *n,
            Item::Nested(p) => p.magnitude(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

    // Explode this pair, if applicable. (Return true if applied.)
    fn explode(&mut self, d:usize) -> bool {
        assert!(d < 4);
        if d == 3 {
            if let Item::Nested(pair) = &self.l {
                eprintln!("explode-left: {:?}", pair); //???
                self.r.incr_if_simple(pair.r.value());
                self.l = Item::Simple(0);
                return true
            }
            if let Item::Nested(pair) = &self.r {
                eprintln!("explode-right: {:?}", pair); //???
                self.l.incr_if_simple(pair.l.value());
                self.r = Item::Simple(0);
                return true
            }
        }
        self.l.explode(d) || self.r.explode(d)
    }

    // Split this pair, if applicable. (Return true if applied.)
    fn split(&mut self) -> bool {
        if let Item::Simple(n) = self.l {
            if n >= 10 {self.l = Item::from_split(n); return true}
        }
        if let Item::Simple(n) = self.r {
            if n >= 10 {self.r = Item::from_split(n); return true}
        }
        self.l.split() || self.r.split()
    }

    // Reduce this top-level expression.
    fn reduce(&self) -> Pair {
        let mut result = self.clone();
        while result.explode(0) || result.split() {}
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

fn sum<'a>(mut iter: impl Iterator<Item=&'a Pair>) -> Pair {
    let mut sum = iter.next().unwrap().clone();
    while let Some(x) = iter.next() {
        sum = sum.add(x);
    }
    sum
}

fn read_file(filename: &str) -> Vec<Pair> {
    let lines = common::read_lines(filename);
    lines.iter().map(|x| Pair::new(x)).collect()
}

pub fn solve() {
    // Test each of the individual examples.
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
    assert_eq!(
        Pair::new("[[[[4,3],4],4],[7,[[8,4],9]]]").add(&Pair::new("[1,1]")),
        Pair::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    assert_eq!(
        Pair::new("[[[[4,3],4],4],[7,[[8,4],9]]]").add(&Pair::new("[1,1]")),
        Pair::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

    // Example summation problems.
    let test1 = vec!(
        Pair::new("[1,1]"), Pair::new("[2,2]"), Pair::new("[3,3]"),
        Pair::new("[4,4]"), Pair::new("[5,5]"), Pair::new("[6,6]"));
    let test2 = read_file("input/test18.txt");
    assert_eq!(sum(test1[0..4].iter()),
        Pair::new("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
    assert_eq!(sum(test1[0..5].iter()),
        Pair::new("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
    assert_eq!(sum(test1[0..6].iter()),
        Pair::new("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    assert_eq!(sum(test2.iter()),
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
    assert_eq!(4140, sum(test2.iter()).magnitude());

    // Solve the real homework problem.
    let data = read_file("input/input18.txt");
    println!("Part1: {}", sum(data.iter()).magnitude());
}
