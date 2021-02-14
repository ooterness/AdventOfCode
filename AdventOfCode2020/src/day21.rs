/// Day 21: https://adventofcode.com/2020/day/21
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
use std::collections::HashSet;
#[path = "common.rs"] mod common;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Ingredient<'a> (&'a str);
type IngredientList<'a> = HashSet<Ingredient<'a>>;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Allergen<'a> (&'a str);
type AllergenList<'a> = HashSet<Allergen<'a>>;

/// Each "food" is a list of ingredients and allergens.
struct Food<'a> {
    ingredients: IngredientList<'a>,
    allergens: AllergenList<'a>,
}

impl<'a> Food<'a> {
    /// Attempt to parse food from description, e.g.,
    /// "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"
    fn new(line: &'a str) -> Option<Food<'a>> {
        let line = line.strip_suffix(')').unwrap();
        if let Some((str1,str2)) = common::split2(line, " (contains ") {
            let mut food = Food {
                ingredients: HashSet::new(),
                allergens: HashSet::new(),
            };
            for s in str1.split(' ')  {
                food.ingredients.insert(Ingredient(s));
            }
            for s in str2.split(", ") {
                food.allergens.insert(Allergen(s));
            }
            Some(food)
        } else {None}
    }
}

/// A problem-set is a list of Food descriptors.
struct Problem<'a> {
    foods: Vec<Food<'a>>,
    ingredients: IngredientList<'a>,
    allergens: AllergenList<'a>,
    solution: HashMap<Allergen<'a>, Ingredient<'a>>,
}

impl<'a> Problem<'a> {
    /// Parse a list of Food descriptors, one per line.
    fn parse(lines: &'a Vec<String>) -> Problem<'a> {
        // Parse the problem statement.
        let foods = lines.iter().filter_map(|l| Food::new(l)).collect();
        let mut prob = Problem {
            foods:          foods,
            ingredients:    IngredientList::new(),
            allergens:      AllergenList::new(),
            solution:       HashMap::new(),
        };
        // Map each allergen to an ingredient name.
        prob.solve();
        prob
    }

    /// Map each allergen to an ingredient.
    fn solve(&mut self) {
        // Find list of all unique identifiers.
        for food in self.foods.iter() {
            for i in food.ingredients.iter()
                {self.ingredients.insert(*i);}
            for a in food.allergens.iter()
                {self.allergens.insert(*a);}
        }
        // Iteratively solve for allergen-to-ingredient mapping.
        while self.solution.len() < self.allergens.len() {
            for a in self.allergens.iter() {
                if let Some(i) = self.solve_one(a) {
                    self.solution.insert(*a, i);
                }
            }
        }
    }

    /// Attempt to match an allergen to a single ingredient.
    fn solve_one(&self, a: &Allergen<'a>) -> Option<Ingredient<'a>> {
        // Skip any allergen that's already solved.
        if self.solution.contains_key(a) {return None;}
        // Screen for all possible matching ingredients.
        let mut possible = self.ingredients.clone();
        for food in self.foods.iter() {
            if food.allergens.contains(a) {
                possible = possible.intersection(&food.ingredients).copied().collect();
            }
        }
        // Subtract any previous unique solutions.
        for i in self.solution.values() {
            possible.remove(i);
        }
        // Is the remainder unique?
        if possible.len() == 1 {
            possible.into_iter().nth(0)
        } else {None}
    }

    /// Find ingredients that are potentially unsafe.
    fn unsafe_ingredients(&self) -> IngredientList<'a> {
        let mut danger = IngredientList::new();
        for i in self.solution.values() {danger.insert(*i);}
        danger
    }

    /// Find ingredients that cannot be allergens.
    fn safe_ingredients(&self) -> IngredientList<'a> {
        // Subtract dangerous items from the overall list.
        let danger = self.unsafe_ingredients();
        self.ingredients.difference(&danger).copied().collect()
    }

    /// Part-1 solution: Count instances of safe ingredients.
    fn part1(&self) -> usize {
        let mut count = 0usize;
        for safe in self.safe_ingredients() {
            count += common::count_true(self.foods.iter()
                .map(|f| f.ingredients.contains(&safe)));
        }
        count
    }

    /// Part-2 solution: Concatenated list of unsafe ingredients.
    fn part2(&self) -> String {
        // Get a sorted list of allergens.
        let mut allergens:Vec<&Allergen> =
            self.allergens.iter().collect();
        allergens.sort();
        // Create the corresponding ingredient list.
        let mut list = String::new();
        for (n,a) in allergens.iter().enumerate() {
            if n > 0 {list += ",";}
            if let Some(i) = self.solution.get(a) {
                list += i.0;
            }
        }
        list
    }
}

pub fn solve() {
    let str_example = vec![
        String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
        String::from("trh fvjkl sbzzf mxmxvkd (contains dairy)"),
        String::from("sqjhc fvjkl (contains soy)"),
        String::from("sqjhc mxmxvkd sbzzf (contains fish)"),
    ];
    let str_input = common::read_strings("input/input21.txt");

    let example = Problem::parse(&str_example);
    let input = Problem::parse(&str_input);

    assert_eq!(example.part1(), 5usize);
    assert_eq!(example.part2(), String::from("mxmxvkd,sqjhc,fvjkl"));
    println!("Part1: {}", input.part1());
    println!("Part2: {}", input.part2());
}
