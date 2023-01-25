/// Advent of Code 2017, Knot hash (Day 10, Day 14)
/// Copyright 2023 by Alex Utter

struct Knot {
    vals: Vec<u8>,
    posn: usize,
    skip: usize,
}

impl Knot {
    fn new(size: usize) -> Knot {
        let mut vals = Vec::new();
        for n in 0..size {vals.push(n as u8);}
        return Knot { vals:vals, posn:0, skip:0 }
    }

    fn hash(input: &str) -> Knot {
        let seq = convert(input);
        let mut beads = Knot::new(256);
        for _ in 0..64 {
            for n in seq.iter() {
                beads.twist(*n as usize);
            }
        }
        return beads;
    }

    // Apply an individual "pinch and twist" operation.
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

    // Calculate N-way XOR-reduction of an N-byte block of data.
    fn xor(&self, size: usize) -> Vec<u8> {
        assert_eq!(self.vals.len(), size * size);
        let mut svec: Vec<u8> = Vec::new();
        for m in 0..size {
            let mut tmp = 0u8;
            for n in 0..size { tmp ^= self.vals[size*m+n]; }
            svec.push(tmp);
        }
        return svec;
    }

    // Convert current hash state to a byte-vector.
    fn hash_raw(&self) -> Vec<u8> {
        self.xor(16)
    }

    // Convert current hash state to a hexadecimal string.
    fn hash_hex(&self) -> String {
        let mut hstr = String::new();
        for x in self.hash_raw().iter() {
            hstr.push_str(&format!("{:02x}", x));
        }
        return hstr;
    }
}

// Convert psuedo-ASCII sequence and add fixed suffix.
fn convert(input: &str) -> Vec<u8> {
    let mut seq: Vec<u8> = input.chars().map(|x| x as u8).collect();
    for n in vec![17, 31, 73, 47, 23] {seq.push(n);}
    return seq
}

// Special case using part-1 rules.
pub fn part1(size: usize, input: &str) -> i64 {
    let mut beads = Knot::new(size);
    for step in input.split(',') {
        if let Ok(n) = step.parse::<usize>() {
            beads.twist(n);
        }
    }
    return (beads.vals[0] as i64) * (beads.vals[1] as i64);
}

// Calculate Knot-hash as a byte-vector.
#[allow(dead_code)]
pub fn hash_raw(input: &str) -> Vec<u8> {
    Knot::hash(input).hash_raw()
}

// Calculate Knot-hash as a hexadecimal string.
#[allow(dead_code)]
pub fn hash_hex(input: &str) -> String {
    Knot::hash(input).hash_hex()
}
