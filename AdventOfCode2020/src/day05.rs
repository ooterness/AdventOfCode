/// Day 5: https://adventofcode.com/2020/day/5
/// Copyright 2021 by Alex Utter

use std::convert::TryFrom;
#[path = "common.rs"] mod common;

struct Seat(u8, u8);

impl Seat {
    fn char2idx(x:char, val:u8) -> u8 {
        if (x == 'B') || (x == 'R') {val} else {0u8}
    }

    fn id(&self) -> u64 {
        (self.0 as u64 * 8u64) + (self.1 as u64)
    }
}

impl TryFrom<&String> for Seat {
    type Error = ();

    fn try_from(value:&String) -> Result<Self, Self::Error> {
        let cc:Vec<char> = value.chars().collect();
        if cc.len() < 10 {return Err(());}
        let r:u8 = Seat::char2idx(cc[0], 64)
                 + Seat::char2idx(cc[1], 32)
                 + Seat::char2idx(cc[2], 16)
                 + Seat::char2idx(cc[3], 8)
                 + Seat::char2idx(cc[4], 4)
                 + Seat::char2idx(cc[5], 2)
                 + Seat::char2idx(cc[6], 1);
        let c:u8 = Seat::char2idx(cc[7], 4)
                 + Seat::char2idx(cc[8], 2)
                 + Seat::char2idx(cc[9], 1);
        Ok(Seat(r,c))
    }
}

pub fn solve() {
    // Test cases from the problem statement.
    let test1a = Seat::try_from(&String::from("BFFFBBFRRR")).unwrap();
    let test1b = Seat::try_from(&String::from("FFFBBBFRRR")).unwrap();
    let test1c = Seat::try_from(&String::from("BBFFBBFRLL")).unwrap();
    assert_eq!(test1a.id(), 567);
    assert_eq!(test1b.id(), 119);
    assert_eq!(test1c.id(), 820);

    // Load input and keep valid passes.
    let input:Vec<String> = common::read_strings("input/input05.txt");
    let passes:Vec<Seat> = input.iter()
        .map(|x| Seat::try_from(x))
        .filter_map(|x| x.ok())
        .collect();
    println!("Loaded {} boarding passes.", passes.len());

    // What's the highest numbered boarding pass?
    let mut idvec:Vec<u64> = passes.iter().map(|x| x.id()).collect();
    if let Some(max_id) = idvec.iter().max() {
        println!("Part 1: Highest ID {}", max_id);
    }

    // Find the missing pass location, which is the only
    // one where the ID vector increments by +2.
    idvec.sort();
    let mut prev = idvec[0];
    for id in idvec.iter() {
        if *id == prev+2 {
            println!("Part 2: My seat ID is {}.", prev+1);
        }
        prev = *id;
    }
}
