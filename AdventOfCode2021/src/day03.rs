/// Day 3: https://adventofcode.com/2021/day/3
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

const DEBUG_VERBOSE: bool = false;

// Convert 0/1 binary string to integer.
fn binstring(x: &str) -> u64 {
    u64::from_str_radix(x, 2).unwrap_or(0u64)
}

// Submarine diagnostic report is a matrix of ones and zeros.
struct Diagnostic {
    numbits: usize,
    numrows: usize,
    rows: Vec<u64>,
}

impl Diagnostic {
    fn new(filename: &str) -> Diagnostic {
        // Read input file.
        let raw = common::read_lines(filename);
        let numrows = raw.len();
        assert!(numrows > 0);
        // Parse each row as an integer.
        let numbits = raw[0].len();
        let rows = raw.iter().map(|x| binstring(&x)).collect();
        Diagnostic {numbits:numbits, numrows:numrows, rows:rows}
    }

    // Count the number of rows that have a '1' in the given bit.
    fn count_ones(&self, mask: u64) -> u64 {
        let mut count = 0u64;
        for r in 0..self.numrows {
            if (self.rows[r] & mask) > 0 {count += 1};
        }
        return count
    }

    // Calculate most common bit in each position.
    fn gamma(&self) -> u64 {
        // Calculate gamma term for each bit...
        let mut gamma = 0u64;
        let min_count = self.numrows as u64 / 2;
        for n in 0..self.numbits {
            // Count the number of rows where the current bit is '1'.
            let mask = 1u64 << n;
            if self.count_ones(mask) >= min_count {gamma += mask;}
        }
        gamma
    }

    // Calculate least common bit in each position.
    fn epsilon(&self) -> u64 {
        // Epsilon is just the complement of gamma.
        let mask = (1u64 << self.numbits) - 1;
        mask - self.gamma()
    }

    fn power(&self) -> u64 {
        self.gamma() * self.epsilon()
    }

    // Construct a new diagnostic report, keeping only the rows where the
    // masked bit matches the reference. (Then move to the next bit.)
    fn filter(&self, mask: u64, bref: u64) -> Diagnostic {
        let newrows : Vec<u64> = self.rows.iter()
            .filter_map(|x| if (x & mask) == bref {Some(x)} else {None})
            .map(|x| x.clone())
            .collect();
        Diagnostic {numbits: self.numbits - 1, numrows: newrows.len(), rows: newrows}
    }

    // Recursively find the oxygen generator rating (most popular filter)
    fn oxygen(&self) -> u64 {
        if DEBUG_VERBOSE {common::print_list("oxygen", self.rows.iter());}
        // Last remaining row is the result.
        if self.numrows == 1 {return self.rows[0];}
        // Otherwise, filter on the next bit.
        let mask = 1u64 << (self.numbits - 1);
        let half = (self.numrows as u64 + 1) / 2;
        let bref = if self.count_ones(mask) >= half {mask} else {0u64};
        self.filter(mask, bref).oxygen()
    }

    // Recursively find the CO2 scrubber rating (least popular filter)
    fn carbon(&self) -> u64 {
        if DEBUG_VERBOSE {common::print_list("carbon", self.rows.iter());}
        // Last remaining row is the result.
        if self.numrows == 1 {return self.rows[0];}
        // Otherwise, filter on the next bit.
        let mask = 1u64 << (self.numbits - 1);
        let half = (self.numrows as u64 + 1) / 2;
        let bref = if self.count_ones(mask) < half {mask} else {0u64};
        self.filter(mask, bref).carbon()
    }

    fn life(&self) -> u64 {
        self.oxygen() * self.carbon()
    }
}

pub fn solve() {
    let test = Diagnostic::new("input/test03.txt");
    let input = Diagnostic::new("input/input03.txt");

    assert_eq!(test.gamma(), 22);
    assert_eq!(test.epsilon(), 9);
    assert_eq!(test.power(), 198);

    println!("Part1: {}", input.power());

    assert_eq!(test.oxygen(), 23);
    assert_eq!(test.carbon(), 10);
    assert_eq!(test.life(), 230);

    println!("Part2: {}", input.life());
}
