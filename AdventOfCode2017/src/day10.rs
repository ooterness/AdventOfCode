/// Advent of Code 2017, Day 10
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

struct Loop {
    vals: Vec<i64>,
    posn: usize,
    skip: usize,
}

impl Loop {
    fn new(size: usize) -> Loop {
        let mut vals = Vec::new();
        for n in 0..size {vals.push(n as i64);}
        return Loop { vals:vals, posn:0, skip:0 }
    }

    fn twist(&mut self, len: usize) {
        // Make each individual swap...
        for n in 0..(len/2) {
            let p1 = (self.posn + n) % self.vals.len();
            let p2 = (self.posn + len-1 - n) % self.vals.len();
            (self.vals[p1], self.vals[p2]) = (self.vals[p2], self.vals[p1]);
        }
        // Update current cursor position.
        self.posn = (self.posn + self.skip + len) % self.vals.len();
        self.skip += 1;
    }

    fn score(&self) -> i64 {
        self.vals[0] * self.vals[1]
    }

    fn xor(&self, size: usize) -> Vec<i64> {
        assert_eq!(self.vals.len(), size * size);
        let mut svec: Vec<i64> = Vec::new();
        for m in 0..size {
            let mut tmp = 0i64;
            for n in 0..size { tmp ^= self.vals[size*m+n]; }
            svec.push(tmp);
        }
        return svec;
    }

    fn hash(&self) -> String {
        let mut hstr = String::new();
        for x in self.xor(16).iter() {
            hstr.push_str(&format!("{:02x}", x));
        }
        return hstr;
    }
}

// Convert psuedo-ASCII sequence for Part 2 rules.
fn convert(input: &str) -> Vec<usize> {
    let mut seq: Vec<usize> = input.chars().map(|x| x as usize).collect();
    for n in vec![17, 31, 73, 47, 23] {seq.push(n);}
    return seq
}

fn part1(size: usize, input: &str) -> i64 {
    let mut beads = Loop::new(size);
    for step in input.split(',') {
        if let Ok(n) = step.parse::<usize>() {
            beads.twist(n);
        }
    }
    return beads.score();
}

fn part2(input: &str) -> String {
    let seq = convert(input);
    let mut beads = Loop::new(256);
    for _ in 0..64 {
        for n in seq.iter() {
            beads.twist(*n);
        }
    }
    return beads.hash();
}

fn main() {
    // Fetch problem input from server.
    let input = fetch::get_data(2017, 10).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(5, "3,4,1,5"), 12);
    assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");

    // Solve for real input.
    println!("Part 1: {}", part1(256, &input));
    println!("Part 2: {}", part2(input.trim()));
}
