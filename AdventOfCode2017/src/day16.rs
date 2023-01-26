/// Advent of Code 2017, Day 16
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

fn first_char(input: &str) -> char {
    input.chars().next().unwrap()
}

enum Move {
    Spin(usize),                // Rotate right N places
    Exchange(usize, usize),     // Swap indices
    Partner(char, char),        // Swap values
}

impl Move {
    fn new(input: &str) -> Option<Move> {
        let first = first_char(input);
        let slash: Vec<&str> = input.split_at(first.len_utf8()).1.split('/').collect();
        match first {
            's' => Some(Move::Spin(slash[0].parse().unwrap())),
            'x' => Some(Move::Exchange(slash[0].parse().unwrap(),
                                       slash[1].parse().unwrap())),
            'p' => Some(Move::Partner(first_char(slash[0]),
                                      first_char(slash[1]))),
            _   => None
        }
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.trim().split(',').filter_map(Move::new).collect()
}

#[derive(Clone, PartialEq)]
struct Dance {
    labels: Vec<char>,
}

impl Dance {
    fn new(size: usize) -> Dance {
        let labels: Vec<char> = (0..size as u32)
            .filter_map(|x| char::from_u32('a' as u32 + x))
            .collect();
        Dance { labels }
    }

    fn apply(&self, mv: &Move) -> Dance {
        Dance { labels: match mv {
            Move::Spin(x) => self.spin(*x),
            Move::Exchange(x, y) => self.exchange(*x, *y),
            Move::Partner(x, y) => self.partner(x, y),
        } }
    }

    fn spin(&self, x: usize) -> Vec<char> {
        let len = self.labels.len();
        return (0..len).map(|n| self.labels[(len + n - x) % len]).collect();
    }

    fn exchange(&self, x: usize, y: usize) -> Vec<char> {
        let mut tmp = self.labels.clone();
        (tmp[x], tmp[y]) = (tmp[y], tmp[x]);
        return tmp;
    }

    fn partner(&self, x: &char, y: &char) -> Vec<char> {
        let nx = self.labels.iter().position(|c| c == x).unwrap();
        let ny = self.labels.iter().position(|c| c == y).unwrap();
        return self.exchange(nx, ny);
    }

    fn to_string(&self) -> String {
        self.labels.iter().collect()
    }
}

fn dance(size: usize, input: &str, count: usize) -> String{
    // Parse moves and set initial state.
    let moves = parse_moves(input);
    let init  = Dance::new(size);
    let mut state = init.clone();
    let mut iter  = 0usize;
    // Iterate until we reach the end or loop back on the initial state.
    while iter < count {
        // Execute one iteration of the dance.
        for m in moves.iter() { state = state.apply(m); }
        iter += 1;
        // Stop early if we reach the initial state.
        if state == init { break; }
    }
    // If we stopped early, iter is the repeat interval. Skip another N loops.
    let skip = (count - iter) / iter;
    iter += iter * skip;
    // Iterate until we reach the end.
    while iter < count {
        for m in moves.iter() { state = state.apply(m); }
        iter += 1;
    }
    return state.to_string();
}

fn main() {
    // Fetch problem input from server.
    let input = fetch::get_data(2017, 16).unwrap();

    // Unit tests on provided examples.
    assert_eq!(dance(5, "s1,x3/4,pe/b", 1), "baedc");
    assert_eq!(dance(5, "s1,x3/4,pe/b", 2), "ceadb");

    // Solve for real input.
    println!("Part 1: {}", dance(16, &input, 1));
    println!("Part 2: {}", dance(16, &input, 1000000000));
}
