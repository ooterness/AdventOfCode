/// Advent of Code 2015, Day 22
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

// Subtract hit points, to a minimum of zero (knocked out).
fn damage(hp: &mut usize, dmg: usize) {
    if *hp > dmg {*hp -= dmg;} else {*hp = 0;}
}

// Read number from text input, e.g., "Hit Points: 51".
fn read_stat(line: &str) -> usize {
    let words: Vec<&str> = line.trim().split(':').collect();
    return words[words.len()-1].trim().parse().unwrap();
}

#[derive(Clone)]
struct State {
    boss_hp: usize,     // Remaining boss HP
    boss_dmg: usize,    // Boss physical damage
    play_hp: usize,     // Remaining player HP
    play_mana: usize,   // Remaining player mana
    mana_spent: usize,  // Total mana spent
    timer_shield: u8,   // Remaining shield effect
    timer_poison: u8,   // Remaining poison effect
    timer_recharge: u8, // Remaining recharge effect
}

impl State {
    fn from(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().lines().collect();
        let boss_hp = read_stat(lines[0]);
        let boss_dmg = read_stat(lines[1]);
        State::new(boss_hp, boss_dmg, 50, 500)
    }

    fn new(boss_hp:usize, boss_dmg:usize, play_hp:usize, play_mana:usize) -> Self {
        State {
            boss_hp: boss_hp,
            boss_dmg: boss_dmg,
            play_hp: play_hp,
            play_mana: play_mana,
            mana_spent: 0,
            timer_shield: 0,
            timer_poison: 0,
            timer_recharge: 0,
        }
    }

    fn win(&self) -> bool {
        self.boss_hp == 0
    }

    // In-place modifications.
    fn damage_boss(&mut self, dmg: usize) {
        damage(&mut self.boss_hp, dmg);
    }

    fn timer_effects(&mut self) {
        if self.timer_shield > 0 {
            self.timer_shield -= 1;
        }
        if self.timer_poison > 0 {
            self.damage_boss(3);
            self.timer_poison -= 1;
        }
        if self.timer_recharge > 0 {
            self.play_mana += 101;
            self.timer_recharge -= 1;
        }
    }

    // Return non-KO state after damaging the player.
    fn damage_player(&self, dmg: usize) -> Option<Self> {
        let mut next = self.clone();
        if self.boss_hp == 0 {              // No attack if boss is KO'd
        } else if self.timer_shield == 0 {  // Player takes full damage
            damage(&mut next.play_hp, dmg);     
        } else if dmg > 7 {                 // Damage reduced by armor
            damage(&mut next.play_hp, dmg-7);   
        } else {                            // Minimum physical damage
            damage(&mut next.play_hp, 1);       
        }
        if next.play_hp > 0 {Some(next)} else {None}
    }

    // Helper function used when attempting to cast various spells.
    fn spend_mana(&self, cost: usize, timer: u8) -> Option<Self> {
        if timer == 0 && self.play_mana >= cost {
            let mut tmp = self.clone();
            tmp.play_mana -= cost;
            tmp.mana_spent += cost;
            return Some(tmp);
        } else {
            return None;
        }
    }

    // Spells are all speculative; return new state if able to cast.
    fn cast_missile(&self) -> Option<Self> {
        if let Some(mut next) = self.spend_mana(53, 0) {
            next.damage_boss(4); Some(next)
        } else {None}
    }

    fn cast_drain(&self) -> Option<Self> {
        if let Some(mut next) = self.spend_mana(73, 0) {
            next.damage_boss(2); next.play_hp += 2; Some(next)
        } else {None}
    }

    fn cast_shield(&self) -> Option<Self> {
        if let Some(mut next) = self.spend_mana(113, self.timer_shield) {
            next.timer_shield = 6; Some(next)
        } else {None}
    }

    fn cast_poison(&self) -> Option<Self> {
        if let Some(mut next) = self.spend_mana(173, self.timer_poison) {
            next.timer_poison = 6; Some(next)
        } else {None}
    }

    fn cast_recharge(&self) -> Option<Self> {
        if let Some(mut next) = self.spend_mana(229, self.timer_recharge) {
            next.timer_recharge = 5; Some(next)
        } else {None}
    }

    // Return all possible states after the next player + boss turns.
    fn next_turn(&self, hard_mode: bool) -> Vec<Self> {
        // Start of player turn, apply timer effects.
        // (Play ends immediately if boss is killed by timer effects.)
        let mut next1 = self.clone();
        if hard_mode {next1.play_hp -= 1;}
        if next1.play_hp == 0 {return vec![];}
        next1.timer_effects();
        if next1.win() {return vec![next1];}

        // Try casting each possible spell.
        let mut next2: Vec<State> = vec![
            next1.cast_missile(),
            next1.cast_drain(),
            next1.cast_shield(),
            next1.cast_poison(),
            next1.cast_recharge(),
        ].into_iter().flatten().collect();

        // Start of boss turn, apply timer effects.
        for st in next2.iter_mut() {st.timer_effects();}

        // Boss turn, damage the player.
        // Return states where the player survives.
        return next2.iter()
            .filter_map(|st| st.damage_player(self.boss_dmg))
            .collect()
    }

    // Find minimum mana expenditure required to win.
    fn mana_to_win(&self, hard_mode: bool) -> usize {
        // Breadth first search of all possible game states...
        let mut min_mana = usize::MAX;
        let mut states: Vec<State> = vec![self.clone()];
        while states.len() > 0 {
            // From each current state, try all possible moves...
            let mut next_states = Vec::new();
            for state in states.iter() {
                for next in state.next_turn(hard_mode).into_iter() {
                    // Abort search if we've already spent too much mana.
                    // Otherwise, update win state or keep searching.
                    if next.mana_spent >= min_mana {continue;}
                    if next.win() {
                        min_mana = next.mana_spent;
                    } else {
                        next_states.push(next);
                    }
                }
            }
            states = next_states;
        }
        return min_mana;
    }
    
}

fn part1(input: &str) -> usize {
    State::from(input).mana_to_win(false)
}

// Find maximum expenditure that still leads to a loss.
fn part2(input: &str) -> usize {
    State::from(input).mana_to_win(true)
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 22).unwrap();

    // Unit tests based on the provided examples:
    let test1 = State::new(13, 8, 10, 250);
    let test2 = State::new(14, 8, 10, 250);
    assert!(test1.mana_to_win(false) <= 226);
    assert!(test2.mana_to_win(false) <= 641);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
