/// Advent of Code 2015, Day 14
/// Copyright 2023 by Alex Utter

use aocfetch;

struct Reindeer {
    speed: u64,
    t_fly: u64,
    t_rest: u64,
}

impl Reindeer {
    // Parse Reindeer from written description.
    fn new(line: &str) -> Self {
        let words: Vec<&str> = line.trim().split(' ').collect();
        assert_eq!(words.len(), 15);
        let speed = words[3].parse().unwrap();
        let t_fly = words[6].parse().unwrap();
        let t_rest = words[13].parse().unwrap();
        return Reindeer {speed:speed, t_fly:t_fly, t_rest:t_rest};
    }

    // Calculate cumulative distance at designated time.
    fn distance(&self, time: u64) -> u64 {
        let period = self.t_fly + self.t_rest;
        let t_cyc = self.t_fly * (time / period);
        let t_rem = u64::min(self.t_fly, time % period);
        return self.speed * (t_cyc + t_rem);
    }
}

fn part1(input: &str) -> u64
{
    input.trim().lines()
        .map(Reindeer::new)
        .map(|x| x.distance(2503))
        .max().unwrap()
}

fn part2(input: &str) -> u64
{
    let deer: Vec<Reindeer> = input.trim().lines()
        .map(Reindeer::new).collect();
    let mut scores = vec![0u64; deer.len()];
    for t in 0..2503 {
        // Increment score at each timestep, including ties.
        let dist: Vec<u64> = deer.iter()
            .map(|x| x.distance(t+1)).collect();
        let dmax = *dist.iter().max().unwrap();
        for (n,&d) in dist.iter().enumerate() {
            if d == dmax {scores[n] += 1;}
        }
    }
    return *scores.iter().max().unwrap();
}

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2015, 14).unwrap();

    // Unit tests on provided examples.
    let test1 = Reindeer::new(
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.");
    let test2 = Reindeer::new(
        "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.");
    assert_eq!(test1.distance(11), 140);
    assert_eq!(test2.distance(11), 176);
    assert_eq!(test1.distance(1000), 1120);
    assert_eq!(test2.distance(1000), 1056);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
