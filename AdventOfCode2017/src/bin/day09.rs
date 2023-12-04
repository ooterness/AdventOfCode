/// Advent of Code 2017, Day 9
/// Copyright 2023 by Alex Utter

use aocfetch;

// Per problem description, a Thing is either a Garbage string <...>
// or a Group {...} containing a comma-delimited list of Things.
enum Thing {
    Garbage(String),
    Group(Things),
}

type Things = Vec<Box<Thing>>;

fn parse_thing(chars: &Vec<char>, start: usize) -> Option<(usize, Thing)> {
    if chars[start] == '{' {
        // Start of a group -> nested list of Things.
        let mut sub: Things = Things::new();
        let mut idx: usize = start + 1;
        while chars[idx] != '}' {
            let (end, tmp) = parse_thing(chars, idx).unwrap();
            idx = end; if chars[idx] == ',' {idx += 1;}
            sub.push(Box::new(tmp));
        }
        return Some((idx+1, Thing::Group(sub)));
    } else if chars[start] == '<' {
        // Start of a garbage string -> read up to matching '>'.
        let mut str: String = String::new();
        let mut idx: usize = start + 1;
        while chars[idx] != '>' {
            if chars[idx] == '!' {idx += 2;}
            else {str.push(chars[idx]); idx += 1;}
        }
        return Some((idx+1, Thing::Garbage(str)));
    } else {
        println!("Parse error at index {}", start);
        return None;
    }
}

fn parse(input: &str) -> Thing {
    let chars = input.trim().chars().clone().collect();
    let (pmax, root) = parse_thing(&chars, 0).unwrap();
    assert_eq!(pmax, chars.len());
    return root;
}

fn nesting(grp: &Thing, lvl: i64) -> i64 {
    match grp {
        Thing::Garbage(_) => 0,
        Thing::Group(v) => lvl + v.iter().map(|x| nesting(x, lvl+1)).sum::<i64>(),
    }
}

fn garbage(grp: &Thing) -> i64 {
    match grp {
        Thing::Garbage(s) => s.len() as i64,
        Thing::Group(v) => v.iter().map(|x| garbage(x)).sum::<i64>(),
    }
}

fn part1(grp: &Thing) -> i64 { nesting(grp, 1) }

fn part2(grp: &Thing) -> i64 { garbage(grp) }

fn main() {
    // Fetch problem input from server.
    let input = parse(&aocfetch::get_data(2017, 9).unwrap());

    // Unit tests on provided examples.
    assert_eq!(part1(&parse("{}")), 1);
    assert_eq!(part1(&parse("{{{}}}")), 6);
    assert_eq!(part1(&parse("{{},{}}")), 5);
    assert_eq!(part1(&parse("{{{},{},{{}}}}")), 16);
    assert_eq!(part1(&parse("{<a>,<a>,<a>,<a>}")), 1);
    assert_eq!(part1(&parse("{{<ab>},{<ab>},{<ab>},{<ab>}}")), 9);
    assert_eq!(part1(&parse("{{<!!>},{<!!>},{<!!>},{<!!>}}")), 9);
    assert_eq!(part1(&parse("{{<a!>},{<a!>},{<a!>},{<ab>}}")), 3);
    assert_eq!(part2(&parse("<>")), 0);
    assert_eq!(part2(&parse("<random characters>")), 17);
    assert_eq!(part2(&parse("<<<<>")), 3);
    assert_eq!(part2(&parse("<{!>}>")), 2);
    assert_eq!(part2(&parse("<!!>")), 0);
    assert_eq!(part2(&parse("<!!!>>")), 0);
    assert_eq!(part2(&parse("<{o\"i!a,<{i<a>")), 10);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
