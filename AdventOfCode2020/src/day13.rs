/// Day 13: https://adventofcode.com/2020/day/13
/// Copyright 2021 by Alex Utter

extern crate num_integer;
use num_integer::lcm;
#[path = "common.rs"] mod common;

struct Bus {
    pos: u64,
    id:  u64,
}

impl Bus {
    fn parse(list:&str) -> Vec<Bus> {
        let mut bus:Vec<Bus> = Vec::new();
        for (n,id) in list.split(',').enumerate() {
            if let Ok(id) = id.parse::<u64>() {
                bus.push(Bus {pos:n as u64, id:id});
            }
        }
        bus
    }

    fn next_stop(&self, t0:u64) -> u64 {
        // What trip number falls on or after t0?
        let n = (t0 + self.id - 1) / self.id;
        self.id * n     // Next departure time
    }
}

fn part1(t0:&str, bus_str:&str) -> u64 {
    // Parse the input strings, ignoring "x" entries.
    let t0:u64 = t0.parse().unwrap_or(0);
    let bus_vec = Bus::parse(bus_str);

    // Find the next bus to arrive on or after t0.
    let mut best_id = u64::MAX;
    let mut best_tt = u64::MAX;
    for bus in bus_vec.iter() {
        let tt = bus.next_stop(t0);
        if tt < best_tt {
            best_id = bus.id;
            best_tt = tt;
        }
    }

    // Part-1 result is the product of ID and wait-time.
    best_id * (best_tt - t0)
}

fn part2(bus_str:&str) -> Option<u64> {
    // Parse the list of Bus-IDs by position.
    let bus_vec = Bus::parse(bus_str);

    // Solution for first bus is trivial (t=0, k=id).
    // Add each subsequent bus by:
    //  * Calculating k' = LCM(k, new-id)
    //  * Incrementing t0 by k until:
    //      * We find a match --> Repeat with next bus
    //      * t > k' --> No solution (abort)
    let mut t = 0u64;
    let mut k = 1u64;
    for bus in bus_vec.iter() {
        let kp = lcm::<u64>(k, bus.id);
        while (t < kp) && ((t+bus.pos) % bus.id != 0) {
            t += k;
        }
        if t >= kp {return None;}
        k = kp;
    }
    Some(t)
}

pub fn solve() {
    let example = vec![
        String::from("939"),
        String::from("7,13,x,x,59,x,31,19")];
    let input = common::read_strings("input/input13.txt");

    assert_eq!(295,                 part1(&example[0], &example[1]));
    assert_eq!(Some(3417),          part2(&"17,x,13,19"));
    assert_eq!(Some(754018),        part2(&"67,7,59,61"));
    assert_eq!(Some(779210),        part2(&"67,x,7,59,61"));
    assert_eq!(Some(1261476),       part2(&"67,7,x,59,61"));
    assert_eq!(Some(1202161486),    part2(&"1789,37,47,1889"));
    assert_eq!(Some(1068781),       part2(&example[1]));
    println!("Part1: {}", part1(&input[0], &input[1]));
    println!("Part2: {}", part2(&input[1]).unwrap_or(0));
}
