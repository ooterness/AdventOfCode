/// Day 8: https://adventofcode.com/2021/day/8
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

// Count the number of set bits.
fn count_bits(x: u8) -> u8 {
    (if x & 1 > 0 {1} else {0}) +
    (if x & 2 > 0 {1} else {0}) +
    (if x & 4 > 0 {1} else {0}) +
    (if x & 8 > 0 {1} else {0}) +
    (if x & 16 > 0 {1} else {0}) +
    (if x & 32 > 0 {1} else {0}) +
    (if x & 64 > 0 {1} else {0}) +
    (if x & 128 > 0 {1} else {0})
}

struct Numeral {
    segs: u8,
    count: u8,
}

impl Numeral {
    fn new(word: &str) -> Numeral {
        let mut segs  = 0u8;
        for ch in word.chars() {
            segs |= match ch {
                'a' => 1u8,
                'b' => 2u8,
                'c' => 4u8,
                'd' => 8u8,
                'e' => 16u8,
                'f' => 32u8,
                'g' => 64u8,
                _   => 0u8,
            };
        };
        Numeral {segs:segs, count:count_bits(segs)}
    }

    // Is this a possible 1, 4, 7, or 8?
    // (i.e., 2, 4, 3, or 7 segments lit.)
    fn part1(&self) -> bool {
        (self.count == 2) || (self.count == 3) || (self.count == 4) || (self.count == 7)
    }
}

struct Display {
    refs: Vec<Numeral>,
    disp: Vec<Numeral>,
}

impl Display {
    fn new(line: &str) -> Display {
        let parts: Vec<&str> = line.split('|').collect();
        assert_eq!(parts.len(), 2);
        let refs = parts[0].trim().split(' ');
        let disp = parts[1].trim().split(' ');
        Display {
            refs: refs.map(Numeral::new).collect(),
            disp: disp.map(Numeral::new).collect(),
        }
    }

    // Count the number of possible 1, 4, 7, or 8.
    // (i.e., 2, 4, 3, or 7 segments lit.)
    fn part1(&self) -> usize {
        self.disp.iter().filter(|x| x.part1()).count()
    }

    // Find the pattern with a given number of segments,
    // after XOR'ing with the designated segment-mask.
    fn find_by_count(&self, m: u8, n: u8) -> u8 {
        self.refs.iter().find(|x| count_bits(x.segs ^ m) == n).unwrap().segs
    }

    // Completely solve a display group.
    fn solve(&self) -> u64 {
        // Identify the digits with unique segment counts.
        let ref1 = self.find_by_count(0, 2);
        let ref4 = self.find_by_count(0, 4);
        let ref7 = self.find_by_count(0, 3);
        let ref8 = self.find_by_count(0, 7);
        // Find additional unique counts after XOR with known patterns.
        let ref3 = self.find_by_count(ref1, 3);
        let ref6 = self.find_by_count(ref7, 5);
        // Use differences to solve individual segments.
        let _sega = ref7 - ref1;
        let segc = ref8 - ref6;
        let segf = ref1 - segc;
        let segd = (ref3 - ref7) & ref4;
        let segb = (ref4 - ref1) - segd;
        let sege = (ref8 - ref3) - segb;
        let _segg = ref3 - (_sega + segc + segd + segf);
        // Construct remaining digits.
        let ref0 = ref8 - segd;
        let ref2 = ref8 - (segb + segf);
        let ref5 = ref8 - (segc + sege);
        let ref9 = ref8 - sege;
        // Match against each digit in the display.
        let mut total = 0u64;
        for digit in self.disp.iter() {
            let n = if digit.segs == ref0 {0}
               else if digit.segs == ref1 {1}
               else if digit.segs == ref2 {2}
               else if digit.segs == ref3 {3}
               else if digit.segs == ref4 {4}
               else if digit.segs == ref5 {5}
               else if digit.segs == ref6 {6}
               else if digit.segs == ref7 {7}
               else if digit.segs == ref8 {8}
               else if digit.segs == ref9 {9} else {0};
            total = 10*total + n;
        }
        total
    }
}

fn part1(x: &Vec<Display>) -> usize {
    x.iter().map(|x| x.part1()).sum()
}

fn part2(x: &Vec<Display>) -> u64 {
    x.iter().map(|x| x.solve()).sum()
}

fn read_and_parse(filename: &str) -> Vec<Display> {
    let lines = common::read_lines(filename);
    lines.iter().map(|x| Display::new(&x)).collect()
}

pub fn solve() {
    let test = read_and_parse("input/test08.txt");
    let data = read_and_parse("input/input08.txt");

    assert_eq!(part1(&test), 26);
    println!("Part1: {}", part1(&data));

    let test2 = Display::new(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
    assert_eq!(test2.solve(), 5353);
    assert_eq!(test[0].solve(), 8394);
    assert_eq!(test[1].solve(), 9781);
    assert_eq!(test[2].solve(), 1197);
    assert_eq!(test[3].solve(), 9361);
    assert_eq!(test[4].solve(), 4873);
    assert_eq!(test[5].solve(), 8418);
    assert_eq!(test[6].solve(), 4548);
    assert_eq!(test[7].solve(), 1625);
    assert_eq!(test[8].solve(), 8717);
    assert_eq!(test[9].solve(), 4315);

    assert_eq!(part2(&test), 61229);
    println!("Part2: {}", part2(&data));
}
