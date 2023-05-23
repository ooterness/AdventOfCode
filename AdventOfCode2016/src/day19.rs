/// Advent of Code 2016, Day 19
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

#[derive(Clone)]
struct Elf {
    count: usize,   // Number of held presents
    next: usize,    // Index of elf to their left
}

impl Elf {
    fn new(next: usize) -> Self {
        Elf {count:1, next:next}
    }

    fn take(&mut self, victim: &Elf) -> usize{
        self.count  += victim.count;
        self.next    = victim.next;
        return victim.next;
    }
}

fn part1(input: &str) -> usize {
    let size: usize = input.trim().parse().unwrap();
    let mut circle: Vec<Elf> = (0..size)
        .map(|n| Elf::new((n+1) % size))
        .collect();
    let mut next = 0usize;
    loop {
        // Does the current elf hold all the presents?
        if circle[next].count >= size {return next + 1;}
        // Otherwise, take presents from the next elf.
        let idx = circle[next].next;
        let tmp = circle[idx].clone();
        next = circle[next].take(&tmp);
    }
}

fn part2(_input: &str) -> usize {
    0 //???
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 19).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1("5"), 3);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
