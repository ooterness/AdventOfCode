/// Advent of Code 2023, Day 2
/// Copyright 2023 by Alex Utter

extern crate aocfetch as fetch;
use core::cmp::max;

// A triplet of red, green, and blue marbles.
struct Rgb {
    r: usize,
    g: usize,
    b: usize,
}

impl Rgb {
    // Parse comma-delimited description, e.g., "3 green, 4 blue, 1 red".
    fn parse(line: &str) -> Rgb {
        let mut rgb = Rgb {r:0, g:0, b:0};
        for item in line.trim().split(',') {
            let tok: Vec<&str> = item.trim().split(' ').collect();
            if tok.len() != 2 {continue;}
            match (tok[0].parse::<usize>(), tok[1]) {
                (Ok(n), "red")      => {rgb.r = n;},
                (Ok(n), "green")    => {rgb.g = n;},
                (Ok(n), "blue")     => {rgb.b = n;},
                (_, _)              => {},
            }
        }
        return rgb;
    }

    // Increase count to meet or exceed the example.
    fn expand(&mut self, limit: &Rgb) {
        self.r = max(self.r, limit.r);
        self.g = max(self.g, limit.g);
        self.b = max(self.b, limit.b);
    }

    // Power rating of a given configuration.
    fn power(&self) -> usize {
        self.r * self.g * self.b
    }

    // Does this triplet fit within the designated upper bound?
    fn within(&self, limit: &Rgb) -> bool {
        self.r <= limit.r && self.g <= limit.g && self.b <= limit.b
    }
}

// Helper function to parse a game number. (e.g., "Game 12" -> 12)
fn game_index(label: &str) -> usize {
    let tok: Vec<&str> = label.split(' ').collect();
    return tok[1].parse().unwrap();
}

// Parse a one-line description into a game-number and a list of RGB triplets.
// e.g., "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".
fn parse_line(line: &str) -> (usize, Vec<Rgb>) {
    let tok: Vec<&str> = line.trim().split(':').collect();
    let idx = game_index(tok[0]);
    let rgb = tok[1].split(';').map(Rgb::parse).collect();
    return (idx, rgb);
}

fn part1(input: &str) -> usize {
    const LIMIT: Rgb = Rgb {r:12, g:13, b:14};
    let mut total = 0usize;
    for line in input.trim().lines() {
        let (idx, rgb) = parse_line(line);
        if rgb.iter().all(|x| x.within(&LIMIT)) {
            total += idx;
        }
    }
    return total;
}

fn part2(input: &str) -> usize {
    let mut total = 0usize;
    for line in input.trim().lines() {
        let mut lower = Rgb {r:0, g:0, b:0};
        let (_, rgb) = parse_line(line);
        for x in rgb.iter() {lower.expand(x);}
        total += lower.power();
    }
    return total;
}

const EXAMPLE: &'static str = "\
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2023, 2).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 8);
    assert_eq!(part2(EXAMPLE), 2286);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
