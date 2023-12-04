/// Advent of Code 2016, Day 4
/// Copyright 2023 by Alex Utter

use aocfetch;

use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Data {
    name: String,
    id: u32,
    hash: String,
}

impl Data {
    // Parse data entry from a line of text.
    fn new(line: &str) -> Self {
        let raw: Vec<&str> = line.trim().split('-').collect();
        let name = raw[0..raw.len()-1].join("-");
        let last = raw.last().unwrap();
        let len = last.len();
        let hash = last[len-6..len-1].to_string();
        let id:u32 = last[0..len-7].parse().unwrap();
        Data {name:name, id:id, hash:hash}
    }

    // Decrypt the label string.
    fn decrypt(&self) -> String {
        const BASE: u32 = 'a' as u32;
        let mut result = String::new();
        for ch in self.name.chars() {
            if ch == '-' {
                result.push(' ');
            } else {
                let idx = (self.id + ch as u32 - BASE) % 26;
                result.push(char::from_u32(BASE + idx).unwrap());
            }
        }
        return result;
    }

    // Does the hash match the expected value?
    fn valid(&self) -> bool {
        // Count occurence of each letter.
        let mut counts: HashMap<char,u64> = HashMap::new();
        for ch in self.name.chars() {
            if ch == '-' {continue;}
            *counts.entry(ch).or_insert(0) += 1;
        }

        // Sort the list of letter counts.
        let mut heap = BinaryHeap::new();
        for (ch, ct) in counts {
            let ch_sort = (char::MAX as u32) - (ch as u32);
            heap.push((ct,ch_sort,ch));
        }
        // Expected hash is the top five characters.
        let (_,_,h1) = heap.pop().unwrap();
        let (_,_,h2) = heap.pop().unwrap();
        let (_,_,h3) = heap.pop().unwrap();
        let (_,_,h4) = heap.pop().unwrap();
        let (_,_,h5) = heap.pop().unwrap();
        let hash = format!("{}{}{}{}{}", h1, h2, h3, h4, h5);
        return hash == self.hash;
    }
}

fn part1(input: &str) -> i64
{
    let mut total = 0i64;
    for line in input.trim().lines() {
        let data = Data::new(line);
        if data.valid() {total += data.id as i64;}
    }
    return total;
}

fn part2(input: &str) -> i64
{
    for line in input.trim().lines() {
        let data = Data::new(line);
        if data.valid() && data.decrypt() == "northpole object storage" {
            return data.id as i64;
        }
    }
    0
}

const TEST: &str = "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 4).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), 1514);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
