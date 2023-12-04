/// Advent of Code 2015, Day 15
/// Copyright 2023 by Alex Utter

use aocfetch;

struct Metric {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Metric {
    fn zero() -> Self {
        Metric {
            capacity:   0,
            durability: 0,
            flavor:     0,
            texture:    0,
            calories:   0,
        }
    }

    fn new(line: &str) -> Self {
        let words: Vec<&str> = line.trim().split([' ', ',']).collect();
        assert_eq!(words.len(), 15);
        Metric {
            capacity:   words[2].parse().unwrap(),
            durability: words[5].parse().unwrap(),
            flavor:     words[8].parse().unwrap(),
            texture:    words[11].parse().unwrap(),
            calories:   words[14].parse().unwrap(),
        }
    }

    fn add(&self, x: &Metric, y: i64) -> Self {
        Metric {
            capacity:   self.capacity   + y * x.capacity,
            durability: self.durability + y * x.durability,
            flavor:     self.flavor     + y * x.flavor,
            texture:    self.texture    + y * x.texture,
            calories:   self.calories   + y * x.calories,
        }
    }

    fn score(&self, cal: &Option<i64>) -> i64 {
        // Pre-screening for calorie requirements?
        if let Some(c) = cal {
            if *c != self.calories {return 0;}
        }
        // Otherwise return the product of the other metrics.
        i64::max(0, self.capacity) *
        i64::max(0, self.durability) *
        i64::max(0, self.flavor) *
        i64::max(0, self.texture)
    }
}

struct Recipe {
    ingredients: Vec<Metric>,
}

impl Recipe {
    fn new(input: &str) -> Self {
        let ingredients = input.trim().lines().map(Metric::new).collect();
        Recipe { ingredients:ingredients }
    }

    // Recursive brute-force search.
    // (Max four ingredients is only 101^3 iterations.)
    fn eval(&self, idx: usize, rem: i64, base: &Metric, cal: &Option<i64>) -> i64 {
        let next = &self.ingredients[idx];
        if idx+1 == self.ingredients.len() {
            // Quantity for the last ingredient is locked.
            let metric = base.add(next, rem);
            // Check the calorie count if one is provided.
            return metric.score(cal);
        } else {
            // Recursively try all possible quantities for this ingredient.
            let mut best_score = i64::MIN;
            for qty in 0..=rem {
                let metric = base.add(next, qty);
                best_score = i64::max(best_score,
                    self.eval(idx+1, rem-qty, &metric, cal));
            }
            return best_score;
        }
    }

    fn best_score(&self, cal: &Option<i64>) -> i64 {
        self.eval(0, 100, &Metric::zero(), cal)
    }
}

fn part1(input: &str) -> i64
{
    Recipe::new(input).best_score(&None)
}

fn part2(input: &str) -> i64
{
    Recipe::new(input).best_score(&Some(500))
}

const TEST: &str = "\
    Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2015, 15).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1(TEST), 62842880);
    assert_eq!(part2(TEST), 57600000);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
