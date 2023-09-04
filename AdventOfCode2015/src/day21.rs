/// Advent of Code 2015, Day 21
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Item {
    fn new(cost:usize, damage:usize, armor:usize) -> Self {
        Item { cost:cost, damage:damage, armor:armor }
    }
}

fn read_stat(line: &str) -> usize {
    let words: Vec<&str> = line.trim().split(':').collect();
    return words[words.len()-1].trim().parse().unwrap();
}

#[derive(Clone, Copy)]
struct Fighter {
    hp: usize,
    damage: usize,
    armor: usize,
    spent: usize,
}

impl Fighter {
    fn new(hp:usize, damage:usize, armor:usize) -> Self {
        Fighter { hp:hp, damage:damage, armor:armor, spent:0 }
    }

    fn boss(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().lines().collect();
        let hp      = read_stat(lines[0]);
        let damage  = read_stat(lines[1]);
        let armor   = read_stat(lines[2]);
        Fighter { hp:hp, damage:damage, armor:armor, spent:0 }
    }

    fn player(hp: usize) -> Self {
        Fighter { hp:hp, damage:0, armor:0, spent:0 }
    }

    fn add_item(&mut self, item: &Item) {
        self.spent  += item.cost;
        self.damage += item.damage;
        self.armor  += item.armor;
    }

    fn hits_to_kill(&self, other: &Fighter) -> usize {
        let dmg = if self.damage > other.armor
            {self.damage - other.armor} else {1usize};
        return (other.hp + dmg - 1) / dmg;
    }

    fn fight(&self, other: &Fighter) -> bool {
        let hits1 = self.hits_to_kill(other);
        let hits2 = other.hits_to_kill(self);
        return hits1 <= hits2;
    }
}

struct ItemShop {
    player: Fighter,
    idx_wpn: usize,
    idx_arm: usize,
    idx_r1: usize,
    idx_r2: usize,
}

impl ItemShop {
    fn new(player: Fighter) -> Self {
        ItemShop { player:player, idx_wpn:0, idx_arm:0, idx_r1:0, idx_r2:0 }
    }

    fn weapon(&self) -> Item {
        match self.idx_wpn {
            0 => Item::new(8, 4, 0),    // Dagger
            1 => Item::new(10, 5, 0),   // Shortsword
            2 => Item::new(25, 6, 0),   // Warhammer
            3 => Item::new(40, 7, 0),   // Longsword
            _ => Item::new(74, 8, 0),   // Greataxe
        }
    }

    fn armor(&self) -> Item {
        match self.idx_arm {
            0 => Item::new(0, 0, 0),    // None
            1 => Item::new(13, 0, 1),   // Leather
            2 => Item::new(31, 0, 2),   // Chainmail
            3 => Item::new(53, 0, 3),   // Splintmail
            4 => Item::new(75, 0, 4),   // Bandedmail
            _ => Item::new(102, 0, 5),  // Platemail
        }
    }

    fn ring(&self, idx: usize) -> Item {
        match idx {
            0 => Item::new(0, 0, 0),    // None
            1 => Item::new(25, 1, 0),   // Damage +1
            2 => Item::new(50, 2, 0),   // Damage +2
            3 => Item::new(100, 3, 0),  // Damage +3
            4 => Item::new(20, 0, 1),   // Defense +1
            5 => Item::new(40, 0, 2),   // Defense +2
            _ => Item::new(80, 0, 3),   // Defense +3
        }
    }
}

impl Iterator for ItemShop {
    type Item = Fighter;

    fn next(&mut self) -> Option<Fighter> {
        // Are we done?
        if self.idx_wpn > 4 {return None;}

        // Add the current item set.
        let mut player = self.player.clone();
        player.add_item(&self.weapon());
        player.add_item(&self.armor());
        player.add_item(&self.ring(self.idx_r1));
        player.add_item(&self.ring(self.idx_r2));

        // Increment state for next time around
        self.idx_r2 += 1;
        if self.idx_r2 == self.idx_r1 {self.idx_r2 += 1;}
        if self.idx_r2 > 6 {
            self.idx_r2 = 0;
            self.idx_r1 += 1;
            if self.idx_r1 > 6 {
                self.idx_r1 = 0;
                self.idx_arm += 1;
                if self.idx_arm > 5 {
                    self.idx_arm = 0;
                    self.idx_wpn += 1;
                }
            }
        }
        return Some(player);
    }
}

// Find minimum budget required to win.
fn part1(input: &str) -> usize {
    let mut budget = usize::MAX;
    let boss = Fighter::boss(input);
    for player in ItemShop::new(Fighter::player(100)) {
        if player.spent < budget && player.fight(&boss) {
            budget = player.spent;
        }
    }
    return budget;
}

// Find maximum expenditure that still leads to a loss.
fn part2(input: &str) -> usize {
    let mut budget = 0;
    let boss = Fighter::boss(input);
    for player in ItemShop::new(Fighter::player(100)) {
        if player.spent > budget && !player.fight(&boss) {
            budget = player.spent;
        }
    }
    return budget;
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 21).unwrap();

    // Unit tests based on the provided examples:
    let test_play = Fighter::new(8, 5, 5);
    let test_boss = Fighter::new(12, 7, 2);
    assert!(test_play.fight(&test_boss));

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
