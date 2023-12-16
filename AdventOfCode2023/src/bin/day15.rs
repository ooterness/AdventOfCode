/// Advent of Code 2023, Day 15
/// Copyright 2023 by Alex Utter

use aocfetch;

fn hash(input: &str) -> u8 {
    let mut total = 0u8;
    for ch in input.trim().chars() {
        total = ((total as u32 + ch as u32) * 17 % 256) as u8;
    }
    return total;
}

type Lens = (String, usize);

struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn new() -> Self {
        Box { lenses: Vec::new() }
    }

    fn find(&self, lbl: &str) -> Option<usize> {
        for (n,lens) in self.lenses.iter().enumerate() {
            if lens.0 == lbl {return Some(n);}
        }
        return None;
    }

    fn focus(&self, b: usize) -> usize {
        self.lenses.iter().enumerate().map(|(l,lens)| (b+1)*(l+1)*lens.1).sum()
    }
}

struct HashMap {
    boxes: Vec<Box>,
}

impl HashMap {
    fn new(input: &str) -> Self {
        let mut tmp = HashMap {
            boxes: (0..256).map(|_| Box::new()).collect(),
        };
        for step in input.trim().split(',') {tmp.apply(step);}
        return tmp;
    }

    fn apply(&mut self, input: &str) {
        let tok: Vec<&str> = input.trim().split(&['-','=']).collect();
        let lbl = tok[0];
        let baux = &mut self.boxes[hash(lbl) as usize];
        assert_eq!(tok.len(), 2);
        if let Ok(pwr) = tok[1].parse::<usize>() {
            // Assign operation (=). Does label match an existing lens?
            if let Some(lens) = baux.find(lbl) {
                baux.lenses[lens].1 = pwr;
            } else {
                baux.lenses.push((lbl.to_string(), pwr));
            }
        } else {
            // Remove operation (-) purges designated lens if found.
            if let Some(lens) = baux.find(lbl) {
                baux.lenses.remove(lens);
            }
        }
    }

    fn focus(&self) -> usize {
        self.boxes.iter().enumerate().map(|(b,baux)| baux.focus(b)).sum()
    }
}

fn part1(input: &str) -> usize {
    input.trim().split(',').map(|s| hash(s) as usize).sum()
}

fn part2(input: &str) -> usize {
    HashMap::new(input).focus()
}

const EXAMPLE: &'static str =
    "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 15).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(EXAMPLE), 1320);
    assert_eq!(part2(EXAMPLE), 145);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
