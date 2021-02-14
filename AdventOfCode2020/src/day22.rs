/// Day 22: https://adventofcode.com/2020/day/22
/// Copyright 2021 by Alex Utter

use std::collections::VecDeque;
#[path = "common.rs"] mod common;

struct Deck(VecDeque<usize>);

impl Deck {
    fn new(lines: &Vec<String>) -> Deck {
        // Skip player name and read one integer per line.
        let mut deck = Deck(VecDeque::new());
        for (n,line) in lines.iter().enumerate() {
            if n > 0 {
                if let Ok(x) = line.parse() {
                    deck.0.push_back(x);
                }
            }
        }
        deck
    }

    fn score(&self) -> u64 {
        let mut score:u64 = 0;
        for (n,c) in self.0.iter().enumerate() {
            let m = self.0.len() - n;
            score += (m as u64) * (*c as u64);
        }
        score
    }
}

struct Game {
    deck1: Deck,
    deck2: Deck,
}

impl Game {
    fn new(lines: &Vec<String>) -> Game {
        // Split deck descriptions by the blank line.
        let groups = common::group_strings(lines);
        Game {
            deck1: Deck::new(&groups[0]),
            deck2: Deck::new(&groups[1]),
        }
    }

    fn play_one(&mut self) {
        let c1 = self.deck1.0.pop_front().unwrap();
        let c2 = self.deck2.0.pop_front().unwrap();
        if c1 > c2 {
            self.deck1.0.push_back(c1);
            self.deck1.0.push_back(c2);
        } else {
            self.deck2.0.push_back(c2);
            self.deck2.0.push_back(c1);
        }
    }

    fn play_to_end(&mut self) {
        while (self.deck1.0.len() > 0) && (self.deck2.0.len() > 0) {
            self.play_one();
        }
    }

    fn score(&self) -> u64 {
        self.deck1.score() + self.deck2.score()
    }
}

pub fn solve() {
    let mut example = Game::new(&common::read_strings("input/test22.txt"));
    let mut input = Game::new(&common::read_strings("input/input22.txt"));

    example.play_to_end();
    assert_eq!(example.score(), 306);

    input.play_to_end();
    println!("Part1: {}", input.score());
}
