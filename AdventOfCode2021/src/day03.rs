/// Day 3: https://adventofcode.com/2021/day/3
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

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

    fn count_ones(&self, mask: u64) -> u64 {
        let mut count = 0u64;
        for r in 0..self.numrows {
            if (self.rows[r] & mask) > 0 {count += 1};
        }
        return count
    }

    fn gamma(&self) -> u64 {
        // Calculate gamma term for each bit...
        let mut gamma = 0u64;
        let min_count = self.numrows as u64 / 2;
        for n in 0..self.numbits {
            // Count the number of rows with the current bit set.
            let mask = 1u64 << n;
            if self.count_ones(mask) >= min_count {gamma += mask;}
        }
        gamma
    }

    fn epsilon(&self) -> u64 {
        // Epsilon is just the complement of gamma.
        let mask = (1u64 << self.numbits) - 1;
        mask - self.gamma()
    }

    fn power(&self) -> u64 {
        self.gamma() * self.epsilon()
    }
}

pub fn solve() {
    let test = Diagnostic::new("input/test03.txt");
    assert_eq!(test.gamma(), 22);
    assert_eq!(test.epsilon(), 9);
    assert_eq!(test.power(), 198);

    let input = Diagnostic::new("input/input03.txt");
    println!("Part1: {}", input.power());
}
