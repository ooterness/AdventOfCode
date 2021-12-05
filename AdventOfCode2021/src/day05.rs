/// Day 5: https://adventofcode.com/2021/day/5
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use core::cmp::max;
use core::cmp::min;

// A point (x,y)
struct Point {
    x: i64,
    y: i64,
}

// A line segment from (x1,y1) to (x2,y2)
struct Segment (Point, Point);

impl Segment {
    fn new(line: &str) -> Option<Segment> {
        let xy = common::split_numeric(line);
        if xy.len() == 4 {
            if xy[0] <= xy[2] {
                Some(Segment(
                    Point {x:xy[0] as i64, y:xy[1] as i64},
                    Point {x:xy[2] as i64, y:xy[3] as i64}))
            } else {
                Some(Segment(
                    Point {x:xy[2] as i64, y:xy[3] as i64},
                    Point {x:xy[0] as i64, y:xy[1] as i64}))
            }
        } else {None}
    }

    fn is_hv(&self) -> bool {
        (self.0.x == self.1.x) || (self.0.y == self.1.y)
    }

    fn contains(&self, p: &Point) -> bool {
        if (self.0.x <= p.x) && (p.x <= self.1.x) {
            let y1 = min(self.0.y, self.1.y);
            let y2 = max(self.0.y, self.1.y);
            (y1 <= p.y) && (p.y <= y2)
        } else {false}
    }
}

// Read input file and return a vector of line segments.
// If "HV" is set, return only horizontal and vertical lines.
fn read_input(filename: &str, hv: bool) -> Vec<Segment> {
    let lines = common::read_lines(filename);
    let segs = lines.iter().filter_map(|x| Segment::new(x));
    if hv {
        let keep_if_hv = |x:Segment| if x.is_hv() {Some(x)} else {None};
        segs.filter_map(keep_if_hv).collect()   // Only horiz/vert
    } else {
        segs.collect()                          // All segments
    }
}

// Count points with at least two overlapping segments.
fn count_overlap(segs: &Vec<Segment>, mapsize: i64) -> usize {
    let mut count_ovr = 0usize;
    for x in 0..mapsize {
        for y in 0..mapsize {
            let point = Point {x:x, y:y};
            let mut count_seg = 0usize;
            for seg in segs {
                if seg.contains(&point) {count_seg += 1;}
            }
            if count_seg >= 2 {count_ovr += 1;}
        }
    }
    return count_ovr
}

pub fn solve() {
    // Part 1 only considers horizontal and vertical lines.
    let test1 = read_input("input/test05.txt", true);
    assert_eq!(test1.len(), 6);                 // Check H/V filter
    assert_eq!(count_overlap(&test1, 10), 5);   // Check overlap
    let data1 = read_input("input/input05.txt", true);
    println!("Part1: {}", count_overlap(&data1, 1000));
}
