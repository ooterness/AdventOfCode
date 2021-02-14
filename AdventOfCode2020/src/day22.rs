/// Day 22: https://adventofcode.com/2020/day/22
/// Copyright 2021 by Alex Utter

use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
#[path = "common.rs"] mod common;

#[derive(Clone, Hash)]
struct Deck(VecDeque<usize>);

impl Deck {
    /// Read initial deck state from problem description.
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

    /// Copy first N cards for recursive rounds.
    fn recurse(&self, n:usize) -> Deck {
        Deck(self.0.iter().take(n).copied().collect())
    }

    /// Calculate sum-of-product "score" for this deck state.
    fn score(&self) -> u64 {
        let mut score:u64 = 0;
        for (n,c) in self.0.iter().enumerate() {
            let m = self.0.len() - n;
            score += (m as u64) * (*c as u64);
        }
        score
    }
}

#[derive(Clone)]
struct Game {
    deck1: Deck,
    deck2: Deck,
}

#[derive(Debug)]
enum Outcome {
    Player1,
    Player2,
    Continue,
}

type History = HashSet<u64>;

impl Game {
    /// Read initial state of two decks from problem description.
    fn new(lines: &Vec<String>) -> Game {
        // Split deck descriptions by the blank line.
        let groups = common::group_strings(lines);
        Game {
            deck1: Deck::new(&groups[0]),
            deck2: Deck::new(&groups[1]),
        }
    }

    /// Hash of current game state.
    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        1234.hash(&mut hasher);
        self.deck1.hash(&mut hasher);
        self.deck2.hash(&mut hasher);
        hasher.finish()
    }

    /// Has the game ended?
    fn outcome(&self) -> Outcome {
        if self.deck1.0.is_empty()      {Outcome::Player2}
        else if self.deck2.0.is_empty() {Outcome::Player1}
        else                            {Outcome::Continue}
    }

    /// Play a single round using regular rules.
    fn play_one(&mut self) -> Outcome {
        // Compare each player's top card.
        let c1 = self.deck1.0.pop_front().unwrap();
        let c2 = self.deck2.0.pop_front().unwrap();
        if c1 > c2 {
            self.deck1.0.push_back(c1);
            self.deck1.0.push_back(c2);
        } else {
            self.deck2.0.push_back(c2);
            self.deck2.0.push_back(c1);
        }
        self.outcome()
    }

    /// Play to end of game using regular rules.
    fn play_to_end(&mut self) -> Outcome {
        loop {
            match self.play_one() {
                Outcome::Continue   => continue,
                x                   => return x,
            }
        }
    }

    /// Play a single round using recursive rules.
    /// Returns true if
    fn recurse_one(&mut self, history: &mut History) -> Outcome {
        // Infinite recursion check for previous game states.
        // (Manually calculate hash to avoid cloning deck contents.)
        let hash = self.hash();
        if history.contains(&hash) {return Outcome::Player1}
        history.insert(hash);
        // Play the next card:
        let c1 = self.deck1.0.pop_front().unwrap();
        let c2 = self.deck2.0.pop_front().unwrap();
        if (self.deck1.0.len() >= c1) && (self.deck2.0.len() >= c2) {
            // Both players have enough cards to recurse.
            let mut copy = Game {
                deck1: self.deck1.recurse(c1),
                deck2: self.deck2.recurse(c2),
            };
            match copy.recurse_to_end() {
                Outcome::Player1 => {
                    self.deck1.0.push_back(c1);
                    self.deck1.0.push_back(c2);
                }
                Outcome::Player2 => {
                    self.deck2.0.push_back(c2);
                    self.deck2.0.push_back(c1);
                }
                _ => eprintln!("Recursion error."),
            }
        } else if c1 > c2 {
            // Regular comparison, Player 1 wins.
            self.deck1.0.push_back(c1);
            self.deck1.0.push_back(c2);
        } else {
            // Regular comparison, Player 2 wins.
            self.deck2.0.push_back(c2);
            self.deck2.0.push_back(c1);
        }
        self.outcome()
    }

    /// Play to completion using regular rules.
    /// Returns true if player-1 is the winner.
    fn recurse_to_end(&mut self) -> Outcome {
        let mut history:History = History::new();
        loop {
            // Play next round:
            match self.recurse_one(&mut history) {
                Outcome::Continue => continue,
                x                 => return x,
            }
        }
    }

    fn score(&self, outcome:Outcome) -> u64 {
        match outcome {
            Outcome::Player1  => self.deck1.score(),
            Outcome::Player2  => self.deck2.score(),
            Outcome::Continue => 0u64,
        }
    }

    fn part1(&self) -> u64 {
        let mut copy:Game = self.clone();
        let winner = copy.play_to_end();
        copy.score(winner)
    }

    fn part2(&self) -> u64 {
        let mut copy:Game = self.clone();
        let winner = copy.recurse_to_end();
        copy.score(winner)
    }
}

pub fn solve() {
    let example1 = Game::new(&common::read_strings("input/test22a.txt"));
    let example2 = Game::new(&common::read_strings("input/test22b.txt"));
    let input = Game::new(&common::read_strings("input/input22.txt"));

    assert_eq!(example1.part1(), 306);
    assert_eq!(example1.part2(), 291);
    assert_eq!(example2.part2(), 105);

    println!("Part1: {}", input.part1());
    println!("Part2: {}", input.part2());
}
