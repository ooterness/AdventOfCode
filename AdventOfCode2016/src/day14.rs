/// Advent of Code 2016, Day 14
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use md5;

#[derive(Clone)]
struct Hash {
    value: [u8;16],     // MD5 hash of salt + index
    count: [u8;16],     // Max consecutive 0-F in hash
    tripl: Option<u8>,  // First triplet, if one exists
}

struct HashFinder {
    key: String,        // Key/salt for creating each Hash
    stt: usize,         // Hash stretching parameter
    idx: usize,         // Next index to be searched
    buf: Vec<Hash>,     // Circular buffer of Hash objects
}

fn to_digits(val: &[u8;16]) -> [u8;32] {
    let mut digits = [0u8;32];
    for n in 0..16 {
        digits[2*n+0] = (val[n] >> 4) & 0xF;
        digits[2*n+1] = (val[n] >> 0) & 0xF;
    }
    return digits;
}

fn to_hexstr(val: &[u8;16]) -> [u8;32] {
    let mut hexstr = to_digits(val);
    for n in 0..32 {
        let x = hexstr[n];
        hexstr[n] = if x < 10 {x+0x30} else {x+0x57};
    }
    return hexstr;
}

impl Hash {
    fn new(key: &str, index: usize, stretch: usize) -> Self {
        // Calculate the initial MD5 hash.
        let salt = format!("{}{}", key, index);
        let mut hash = Hash {
            value: *md5::compute(salt.as_bytes()),
            count: [0;16],
            tripl: None,
        };
        // Hash stretching, if applicable...
        for _ in 0..stretch {
            hash.value = *md5::compute(to_hexstr(&hash.value));
        }
        // Count consecutive hexadecimal digits.
        let digits = to_digits(&hash.value);
        let mut prev  = 0u8;
        let mut count = 0u8;
        for d in digits {
            let n = d as usize;
            count = if d == prev {count+1} else {1};
            if count >= 3 && hash.tripl == None {hash.tripl = Some(d);}
            if count >= hash.count[n] {hash.count[n] = count;}
            prev = d;
        }
        return hash;
    }

    fn has3(&self) -> Option<u8> {
        self.tripl
    }

    fn has5(&self, digit: u8) -> bool {
        self.count[digit as usize] >= 5
    }
}

impl HashFinder {
    fn new(key: &str, search: usize, stretch: usize) -> Self {
        HashFinder {
            key: key.to_string(),
            idx: 0usize,
            stt: stretch,
            buf: (0..search+1).map(|n| Hash::new(key, n, stretch)).collect(),
        }
    }

    fn test(&self) -> bool {
        let m = self.idx % self.buf.len();
        if let Some(d) = self.buf[m].has3() {
            for offset in 1..self.buf.len() {
                let n = (self.idx + offset) % self.buf.len();
                if self.buf[n].has5(d) {return true;}
            }
        }
        return false;
    }

    fn incr(&mut self) {
        let n = self.idx % self.buf.len();
        self.buf[n] = Hash::new(&self.key, self.idx + self.buf.len(), self.stt);
        self.idx += 1;
    }

    fn next(&mut self) -> usize {
        while !self.test() {self.incr();}
        self.incr();    // Get ready for next search...
        return self.idx - 1;
    }
}

fn part1(input: &str) -> usize {
    let mut finder = HashFinder::new(input.trim(), 1000, 0);
    let mut index = 0usize;
    for _ in 0..64 {index = finder.next();}
    return index;
}

fn part2(input: &str) -> usize {
    let mut finder = HashFinder::new(input.trim(), 1000, 2016);
    let mut index = 0usize;
    for _ in 0..64 {index = finder.next();}
    return index;
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 14).unwrap();

    // Unit tests on provided examples
    let mut test1 = HashFinder::new("abc", 1000, 0);
    let mut test2 = HashFinder::new("abc", 1000, 2016);
    assert_eq!(Hash::new("abc", 18, 0).has3(), Some(8));
    assert_eq!(Hash::new("abc", 5, 2016).has3(), Some(2));
    assert_eq!(test1.next(), 39);
    assert_eq!(test1.next(), 92);
    assert_eq!(test2.next(), 10);
    assert_eq!(part1("abc"), 22728);
    assert_eq!(part2("abc"), 22551);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
