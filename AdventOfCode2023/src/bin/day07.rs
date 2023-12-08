/// Advent of Code 2023, Day 7
/// Copyright 2023 by Alex Utter

use aocfetch;
use core::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [usize;5],
    typ: usize,
    bid: usize,
}

impl Hand {
    fn new(line: &str, part1: bool) -> Self {
        let mut hand = Hand {cards:[0;5], typ:0, bid:0};
        let mut count = [0usize;15];
        for n in 0..5 {
            hand.cards[n] = match line.chars().nth(n).unwrap() {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => if part1 {11} else {0},
                'T' => 10,
                ch  => ch.to_digit(10).unwrap() as usize,
            };
            count[hand.cards[n]] += 1;
        }
        let max = count.iter().skip(1).max().unwrap();
        let two = count.iter().skip(1).filter(|x| **x == 2).count();
        let wild = hand.cards.iter().filter(|x| **x == 0).count();
        hand.bid = line[6..].parse().unwrap();
        hand.typ = match (max + wild, wild, two) {
            (5,_,_) => 6, // Five of a kind
            (4,_,_) => 5, // Four of a kind
            (3,1,2) => 4, // Full house w/ wild
            (3,0,1) => 4, // Full house no wild
            (3,_,_) => 3, // Three of a kind
            (2,_,2) => 2, // Two pair
            (2,_,_) => 1, // One pair
            (_,_,_) => 0, // High card
        };
        return hand;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut result = self.typ.cmp(&other.typ);
        for n in 0..5 {result = result.then(self.cards[n].cmp(&other.cards[n]));}
        return result;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.trim().lines()
        .map(|s| Hand::new(s.trim(), true)).collect();
    hands.sort();
    return hands.iter().enumerate().map(|(r,h)| (r+1) * h.bid).sum()
}

fn part2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.trim().lines()
        .map(|s| Hand::new(s.trim(), false)).collect();
    hands.sort();
    return hands.iter().enumerate().map(|(r,h)| (r+1) * h.bid).sum()
}

const EXAMPLE: &'static str = "\
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 7).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 6440);
    assert_eq!(part2(EXAMPLE), 5905);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
