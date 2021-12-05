/// Day 5: https://adventofcode.com/2021/day/5
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::cmp::max;

// A point (x,y)
struct Point {
    x: i64,
    y: i64,
}

// A "map" is a 2D-array of counters for each X/Y point.
struct MapCount {
    mapsize: usize,
    counts: Vec<usize>,
}

impl MapCount {
    fn new(mapsize: usize) -> MapCount {
        MapCount {mapsize:mapsize, counts: vec![0; mapsize*mapsize]}
    }

    fn mark(&mut self, pt: &Point) {
        let idx = self.mapsize * pt.y as usize + pt.x as usize;
        self.counts[idx] += 1;
    }

    fn count2(&self) -> usize {
        self.counts.iter().filter(|&x| *x >= 2).count()
    }
}

// A line segment from (x1,y1) to (x2,y2)
struct Segment (Point, Point);

impl Segment {
    fn new(line: &str) -> Option<Segment> {
        let xy = common::split_numeric(line);
        if xy.len() == 4 {
            Some(Segment(
                Point {x:xy[0] as i64, y:xy[1] as i64},
                Point {x:xy[2] as i64, y:xy[3] as i64}))
        } else {None}
    }

    fn is_hv(&self) -> bool {
        (self.0.x == self.1.x) || (self.0.y == self.1.y)
    }

    fn len(&self) -> i64 {
        let dx = (self.1.x - self.0.x).abs();
        let dy = (self.1.y - self.0.y).abs();
        max(dx, dy)
    }

    fn mark_on(&self, map: &mut MapCount) {
        let dt = self.len();
        let dx = (self.1.x - self.0.x) / dt;
        let dy = (self.1.y - self.0.y) / dt;
        for t in 0..dt+1 {
            let x = self.0.x + t * dx;
            let y = self.0.y + t * dy;
            map.mark(&Point{x:x, y:y});
        }
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
fn count_overlap(segs: &Vec<Segment>, mapsize: usize) -> usize {
    let mut map = MapCount::new(mapsize);
    for seg in segs {seg.mark_on(&mut map);}
    map.count2()
}

pub fn solve() {
    // Part 1 only considers horizontal and vertical lines.
    let test1 = read_input("input/test05.txt", true);
    assert_eq!(test1.len(), 6);                 // Check H/V filter
    assert_eq!(count_overlap(&test1, 10), 5);   // Check overlap
    let data1 = read_input("input/input05.txt", true);
    println!("Part1: {}", count_overlap(&data1, 1000));

    // Part 2 considers lines of all types.
    let test2 = read_input("input/test05.txt", false);
    assert_eq!(test2.len(), 10);
    assert_eq!(count_overlap(&test2, 10), 12);
    let data2 = read_input("input/input05.txt", false);
    println!("Part1: {}", count_overlap(&data2, 1000));
}
