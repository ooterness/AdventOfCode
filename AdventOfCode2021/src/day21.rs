/// Day 21: https://adventofcode.com/2021/day/21
/// Copyright 2021 by Alex Utter

use std::cmp::max;
use std::cmp::min;

const VERBOSE: bool = false;

// Given starting position, play game with deterministic dice.
fn part1(start: &(u64,u64)) -> u64 {
    // Set initial game state.
    let mut die = 0u64;
    let mut pos = (start.0-1, start.1-1);
    let mut pts = (0u64, 0u64);
    // Keep playing until one player reaches target score.
    while pts.0 < 1000 && pts.1 < 1000 {
        // Roll the die three times.
        let incr = (3*die+1) + (3*die+2) + (3*die+3);
        // Move pieces and update points.
        if die % 2 == 0 {
            pos.0 = (pos.0 + incr) % 10;
            pts.0 += pos.0 + 1;
        } else {
            pos.1 = (pos.1 + incr) % 10;
            pts.1 += pos.1 + 1;
        }
        // Increment state of deterministic die.
        die += 1;
    }
    3 * die * min(pts.0, pts.1)
}

// Indexable game state
struct GameState {
    pos: (usize,usize),
    pts: (usize,usize),
}

impl GameState {
    fn new(x: &(usize,usize)) -> GameState {
        GameState { pos:(x.0-1, x.1-1), pts:(0,0) }
    }

    fn from_idx(x: &usize) -> GameState {
        GameState {
            pos: ((x/1) % 10, (x/10) % 10),
            pts: ((x/100) % 21, (x/2100)),
        }
    }

    fn to_idx(&self) -> usize {
        self.pos.0 + 10*self.pos.1 + 100*self.pts.0 + 2100*self.pts.1
    }

    fn incr(&self, nxt0:bool, n:usize) -> GameState {
        if nxt0 {
            let pos = (self.pos.0 + n) % 10;
            GameState { pos:(pos, self.pos.1), pts:(self.pts.0+pos+1, self.pts.1) }
        } else {
            let pos = (self.pos.1 + n) % 10;
            GameState { pos:(self.pos.0, pos), pts:(self.pts.0, self.pts.1+pos+1) }
        }
    }

    fn won(&self) -> bool {
        self.pts.0 > 20 || self.pts.1 > 20
    }
}

// Given starting position, play game with quantum dice.
fn part2(start: &(usize,usize)) -> u64 {
    // Count the number of games in each state.
    let max_states:usize = 10*10*21*21;
    let mut count = vec![0u64; max_states];
    let mut wins = (0u64,0u64);
    let mut turn = 0usize;
    // Set the initial state.
    count[GameState::new(start).to_idx()] = 1;
    // Simulate until all games are completed.
    while count.iter().sum::<u64>() > 0 {
        let mut next = vec![0u64; max_states];
        // From each possible initial state...
        for n in 0..max_states {
            // Skip empty states.
            if count[n] == 0 {continue;}
            // Determine new state after each possible die roll.
            let nxt0 = (turn % 2) == 0;
            let st0 = GameState::from_idx(&n);
            let st3 = st0.incr(nxt0, 3);
            let st4 = st0.incr(nxt0, 4);
            let st5 = st0.incr(nxt0, 5);
            let st6 = st0.incr(nxt0, 6);
            let st7 = st0.incr(nxt0, 7);
            let st8 = st0.incr(nxt0, 8);
            let st9 = st0.incr(nxt0, 9);
            // Who wins if this is the last round?
            let wn = if nxt0 {&mut wins.0} else {&mut wins.1};
            // Increment state by number of three-die permutations.
            if st3.won() {*wn += count[n]*1} else {next[st3.to_idx()] += count[n]*1};
            if st4.won() {*wn += count[n]*3} else {next[st4.to_idx()] += count[n]*3};
            if st5.won() {*wn += count[n]*6} else {next[st5.to_idx()] += count[n]*6};
            if st6.won() {*wn += count[n]*7} else {next[st6.to_idx()] += count[n]*7};
            if st7.won() {*wn += count[n]*6} else {next[st7.to_idx()] += count[n]*6};
            if st8.won() {*wn += count[n]*3} else {next[st8.to_idx()] += count[n]*3};
            if st9.won() {*wn += count[n]*1} else {next[st9.to_idx()] += count[n]*1};
        }
        count = next;
        turn += 1;
        if VERBOSE {println!("Turn {}: {} states", turn, count.iter().sum::<u64>())};
    }
    max(wins.0, wins.1)
}

pub fn solve() {
    assert_eq!(part1(&(4,8)), 739785);
    println!("Part1: {}", part1(&(7,4)));

    assert_eq!(part2(&(4,8)), 444356092776315);
    println!("Part2: {}", part2(&(7,4)));
}
