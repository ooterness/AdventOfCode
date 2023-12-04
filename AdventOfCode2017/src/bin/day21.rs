/// Advent of Code 2017, Day 21
/// Copyright 2023 by Alex Utter

use aocfetch;
use std::collections::HashMap;

// Largest cell pattern of interest is 4x4 -> fits in u16.
type Mask = u16;

// Count the number of '1' bits in a given mask.
fn count_bits(mask: &Mask) -> usize {
    (0..16).map(|b| (mask>>b & 1) as usize).sum()
}

// A single 2x2, 3x3, or 4x4 "cell" for table lookup.
// Bit mask reading order (0/1/2/3 in first row, 4/5/6/7 in second...)
// Native representation is a bit-mask in reading order (
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Cell<const N: usize> {
    mask: Mask,
}

impl<const N: usize> Cell<N> {
    fn from_str(input: &str) -> Cell<N> {
        assert_eq!(input.len(), N*N + N - 1);
        let mut mask: Mask = 0;
        for (r,row) in input.split('/').enumerate() {
            for (c,col) in row.chars().enumerate() {
                if col == '#' { mask |= Self::rc(r, c); }
            }
        }
        return Cell {mask};
    }

    fn from_bits(input: &[u16]) -> Cell<N> {
        assert_eq!(input.len(), N*N);
        let mut mask: Mask = 0;
        for (b, v) in input.iter().enumerate() {
            if v > &0 { mask |= 1 << b; }
        }
        return Cell {mask};
    }

    fn bit(&self, r: usize, c: usize) -> Mask {
        self.mask & Self::rc(r, c)
    }

    fn rc(r: usize, c: usize) -> Mask {
        assert!(r < N && c < N);
        (1 as Mask) << (N*r + c)
    }

    fn reshape(&self, newrc: impl Fn(usize,usize) -> Mask) -> Cell<N> {
        let mut mask: Mask = 0;
        for r in 0..N {
            for c in 0..N {
                if self.bit(r,c) > 0 { mask |= newrc(r,c); }
            }
        }
        return Cell {mask};
    }

    fn hflip(&self) -> Cell<N> {
        self.reshape(|r,c: usize| Self::rc(r, N-1-c))
    }

    fn transpose(&self) -> Cell<N> {
        self.reshape(|r,c: usize| Self::rc(c, r))
    }

    fn symmetry(&self) -> [Cell<N>;8] {
        let a = self.hflip();
        let b = a.transpose();
        let c = b.hflip();
        let d = c.transpose();
        let e = d.hflip();
        let f = e.transpose();
        let g = f.hflip();
        assert_eq!(self, &g.transpose());
        return [self.clone(), a, b, c, d, e, f, g];
    }
}

// Rule-based fractal generation operating on counts of each cell type.
type MaskCount = HashMap<Mask, usize>;
type MaskRule = HashMap<Mask, MaskCount>;

struct Rules {
    rule_1a: MaskRule,  // Map 3x3 -> 4x4
    rule_1b: MaskRule,  // Map 4x4 -> 2x2 (x9)
    rule_1c: MaskRule,  // Map 2x2 -> 3x3
    rule_3:  MaskRule,  // Map 3x3 -> 3x3 (x9)
}

impl Rules {
    fn new(input: &str) -> Rules {
        let mut rules = Rules {
            rule_1a: MaskRule::new(),
            rule_1b: MaskRule::new(),
            rule_1c: MaskRule::new(),
            rule_3:  MaskRule::new(),
        };

        // Parse raw tables for 2x2 and 3x3 inputs.
        let mut table2: HashMap<Cell<2>, Cell<3>> = HashMap::new();
        let mut table3: HashMap<Cell<3>, Cell<4>> = HashMap::new();
        for line in input.lines() {
            let lr: Vec<&str> = line.split(" => ").collect();
            assert_eq!(lr.len(), 2);
            if lr[0].len() == 5 {
                let l: Cell<2> = Cell::from_str(lr[0]);
                let r: Cell<3> = Cell::from_str(lr[1]);
                for s in l.symmetry() { table2.insert(s, r); }
            } else {
                let l: Cell<3> = Cell::from_str(lr[0]);
                let r: Cell<4> = Cell::from_str(lr[1]);
                for s in l.symmetry() { table3.insert(s, r); }
            }
        }

        // Rules 1A (3->4) and 1C (2->3) are a direct 1:1 lookup.
        for (before, after) in table2.iter() {
            let tmp = MaskCount::from([(after.mask, 1)]);
            rules.rule_1c.insert(before.mask, tmp);
        }

        for (before, after) in table3.iter() {
            let tmp = MaskCount::from([(after.mask, 1)]);
            rules.rule_1a.insert(before.mask, tmp);
        }

        // Rule 1C (4->2) requires additional parsing.
        for in4 in table3.values() {
            // Table lookup for each input quadrant.
            let nw: Cell<3> = table2[&Cell::from_bits(&[
                in4.bit(0,0), in4.bit(0,1),
                in4.bit(1,0), in4.bit(1,1)])];
            let ne: Cell<3> = table2[&Cell::from_bits(&[
                in4.bit(0,2), in4.bit(0,3),
                in4.bit(1,2), in4.bit(1,3)])];
            let sw: Cell<3> = table2[&Cell::from_bits(&[
                in4.bit(2,0), in4.bit(2,1),
                in4.bit(3,0), in4.bit(3,1)])];
            let se: Cell<3> = table2[&Cell::from_bits(&[
                in4.bit(2,2), in4.bit(2,3),
                in4.bit(3,2), in4.bit(3,3)])];
            // Recombine 3x3 quadrants into nine 2x2 blocks.
            let out_quads: [Cell<2>;9] = [
                Cell::from_bits(&[  // Northwest
                    nw.bit(0,0), nw.bit(0,1),
                    nw.bit(1,0), nw.bit(1,1)]),
                Cell::from_bits(&[  // North
                    nw.bit(0,2), ne.bit(0,0),
                    nw.bit(1,2), ne.bit(1,0)]),
                Cell::from_bits(&[  // Northeast
                    ne.bit(0,1), ne.bit(0,2),
                    ne.bit(1,1), ne.bit(1,2)]),
                Cell::from_bits(&[  // West
                    nw.bit(2,0), nw.bit(2,1),
                    sw.bit(0,0), sw.bit(0,1)]),
                Cell::from_bits(&[  // Center
                    nw.bit(2,2), ne.bit(2,0),
                    sw.bit(0,2), se.bit(0,0)]),
                Cell::from_bits(&[  // East
                    ne.bit(2,1), ne.bit(2,2),
                    se.bit(0,1), se.bit(0,2)]),
                Cell::from_bits(&[  // Southwest
                    sw.bit(1,0), sw.bit(1,1),
                    sw.bit(2,0), sw.bit(2,1)]),
                Cell::from_bits(&[  // South
                    sw.bit(1,2), se.bit(1,0),
                    sw.bit(2,2), se.bit(2,0)]),
                Cell::from_bits(&[  // Southeast
                    se.bit(1,1), se.bit(1,2),
                    se.bit(2,1), se.bit(2,2)]),
            ];
            // Insert each one into the output table.
            let mut result = MaskCount::new();
            for cell in out_quads {
                let old_count = result.get(&cell.mask).unwrap_or(&0);
                result.insert(cell.mask, old_count + 1);
            }
            rules.rule_1b.insert(in4.mask, result);
        }

        // Rule-3 chains together all three of the rules.
        for (input, rule1) in rules.rule_1a.iter() {
            let mut result = MaskCount::new();
            for (mask1, count1) in rule1.iter() {
                if let Some(rule2) = rules.rule_1b.get(mask1) {
                    for (mask2, count2) in rule2.iter() {
                        if let Some(rule3) = rules.rule_1c.get(mask2) {
                            for (mask3, count3) in rule3.iter() {
                                let old_count = result.get(mask3).unwrap_or(&0);
                                result.insert(*mask3, old_count + count1*count2*count3);
                            }
                        }
                    }
                }
            }
            rules.rule_3.insert(*input, result);
        }
        return rules;
    }

    fn apply(rule: &MaskRule, input: &MaskCount) -> MaskCount {
        let mut result = MaskCount::new();
        for (prev, count) in input.iter() {
            for (mask, scale) in rule[prev].iter() {
                let old_count = result.get(mask).unwrap_or(&0);
                result.insert(*mask, old_count + count * scale);
            }
        }
        return result;
    }

}

// Fractal generator operates on counts of each cell type.
struct Fractal {
    step: usize,                        // Current iteration count
    count: HashMap<Mask, usize>,        // Number of cells of each type
}

impl Fractal {
    fn new() -> Fractal {
        // Initial state is a specific 3x3 "glider" pattern.
        let glider: Cell<3> = Cell::from_str(".#./..#/###");
        let count = HashMap::from([(glider.mask, 1usize)]);
        Fractal { step: 0, count: count }
    }

    fn count(&self) -> usize {
        let mut total = 0usize;
        for (mask, count) in self.count.iter() {
            total += count * count_bits(mask);
        }
        return total;
    }

    // Advance one timestep, selecting 1A, 1B, or 1C.
    fn step1(&self, rules: &Rules) -> Fractal {
        let rule = match self.step % 3 {
            0 => &rules.rule_1a,
            1 => &rules.rule_1b,
            _ => &rules.rule_1c,
        };
        return Fractal {
            step: self.step+1,
            count: Rules::apply(rule, &self.count),
        }
    }

    // Perform steps 1A, 1B, and 1C in sequence.
    fn step3(&self, rules: &Rules) -> Fractal {
        assert_eq!(self.step % 3, 0);
        return Fractal {
            step: self.step+3,
            count: Rules::apply(&rules.rule_3, &self.count),
        }
    }
}

fn run(input: &str, steps: usize) -> usize {
    let rules = Rules::new(input);
    let mut state = Fractal::new();
    for _ in 0..(steps/3) { state = state.step3(&rules); }
    for _ in 0..(steps%3) { state = state.step1(&rules); }
    return state.count();
}

fn part1(input: &str) -> usize {
    run(input, 5)
}

fn part2(input: &str) -> usize {
    run(input, 18)
}

const TEST: &str = "\
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

fn main() {
    // Fetch problem input from server.
    let input = aocfetch::get_data(2017, 21).unwrap();

    // Unit tests on provided examples.
    assert_eq!(run(TEST, 0), 5);
    assert_eq!(run(TEST, 1), 4);
    assert_eq!(run(TEST, 2), 12);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
