/// Advent of Code 2015, Day 24
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

struct Packages {
    list: Vec<usize>,
    prod: usize,
    sum: usize,
}

impl Packages {
    // Create an empty list of packages.
    fn new() -> Self {
        Packages { list: Vec::new(), prod: 1usize, sum: 0usize }
    }

    // Parse package list from string, one weight per line.
    fn from(input: &str) -> Self {
        let mut tmp = Packages::new();
        for line in input.trim().lines() {
            tmp.add(line.trim().parse().unwrap());
        }
        return tmp;
    }

    // Add a package to the list.
    fn add(&mut self, weight: usize) {
        // Prevent overflow for long lists.
        if weight <= usize::MAX / self.prod {
            self.prod *= weight;
        } else {
            self.prod = usize::MAX;
        }
        self.list.push(weight);
        self.sum += weight;
    }

    // Length of this list.
    fn len(&self) -> usize {
        self.list.len()
    }

    // Return a Splitter object for the designated target weight.
    fn split(&self, target: usize) -> Splitter {
        Splitter { index:0, source:&self.list, target:target }
    }
}

struct Splitter<'a> {
    index: usize,
    source: &'a Vec<usize>,
    target: usize,
}

impl<'a> Splitter<'a> {
    // Split self into included and excluded half.
    fn split(&self) -> (Packages, Packages) {
        let mut inc = Packages::new();
        let mut exc = Packages::new();
        for (n,w) in self.source.iter().enumerate() {
            if self.index & (1usize<<n) > 0 {
                inc.add(*w);
            } else {
                exc.add(*w);
            }
        }
        return (inc, exc);
    }

    // Find total weight for the current index.
    fn weight(&self) -> usize {
        self.source.iter().enumerate()
            .map(|(n,w)| if self.index & (1usize<<n) > 0 {*w} else {0})
            .sum()
    }
}

impl<'a> Iterator for Splitter<'a> {
    // Each item is a pair of included and excluded packages.
    type Item = (Packages, Packages);

    // Yield the next matching item.
    fn next(&mut self) -> Option<Self::Item> {
        let max_index = 1usize << self.source.len();
        while self.index <= max_index {
            if self.weight() == self.target {
                let result = self.split();
                self.index += 1;
                return Some(result);
            } else {
                self.index += 1;
            }
        }
        return None;
    }
}

fn part1(input: &str) -> usize {
    let packages = Packages::from(input);
    let target = packages.sum / 3;
    // For each valid subset of packages...
    let mut best = (usize::MAX, usize::MAX);
    for (inc, exc) in packages.split(target) {
        // Confirm the 2nd and 3rd bins can be divided evenly...
        if let Some(_) = exc.split(target).next() {
            let next = (inc.len(), inc.prod);
            if next < best {best = next;}
        } 
    }
    return best.1
}

fn part2(input: &str) -> usize {
    let _packages = Packages::from(input);
    0
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 24).unwrap();

    // Unit tests based on the provided examples:
    let test = "1\n2\n3\n4\n5\n7\n8\n9\n10\n11";
    assert_eq!(part1(test), 99);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
