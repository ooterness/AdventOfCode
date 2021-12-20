/// Day 20: https://adventofcode.com/2021/day/20
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
#[path = "grid.rs"] mod grid;

// Image-enhancement filter is a 512-element lookup table.
type Filter = Vec<bool>;

// An image contains:
//  * The value of each pixel beyond the edge (true/false)
//  * A grid containing each normal in-bounds pixel.
type Image = (bool, grid::Grid<bool>);

// Read a line of "...##.#" style text.
fn read_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

// Read a single line as a Filter objec.t
fn read_filter(line: &str) -> Filter {
    let filter = read_line(line);
    assert_eq!(filter.len(), 512);
    filter
}

// Read several lines as a binary text image.
fn read_image(lines: &[String]) -> Image {
    let mat = lines.iter().map(|l| read_line(l));
    (false, grid::Grid::new(mat.collect()))
}

// Count the number of lit pixels.
fn count(img: &Image) -> usize {
    assert!(!img.0);    // Infinite lit pixels?
    img.1.iter()        // Count non-infinite section
        .filter_map(|rc| img.1.get(&rc))
        .map(|b| if *b {1usize} else {0usize})
        .sum()
}

// Apply filter to image.
fn enhance(img: &Image, filt: &Filter) -> Image {
    // Create an empty output image.
    let out_size = grid::GridSize {r:img.1.size.r+2, c:img.1.size.c+2};
    let mut out = grid::Grid::empty(&false, &out_size);
    // Calculate the beyond-the-edge output value.
    let border = if img.0 {filt[511]} else {filt[0]};
    // Sliding window filter over each output pixel.
    for rc in out.iter() {
        let ctr = rc.nw();          // Center in input frame.
        let nbr = [ctr.se(), ctr.ss(), ctr.sw(),
                   ctr.ee(), ctr.cc(), ctr.ww(),
                   ctr.ne(), ctr.nn(), ctr.nw()];
        let idx:usize = nbr.iter()  // Neighbors in LSB-first order
            .map(|rc| img.1.get(rc).unwrap_or(&img.0))
            .enumerate()            // Convert bits to integer...
            .map(|(n,p)| if *p {1usize << n} else {0})
            .sum();                 // Index for enhancement filter
        out.set(&rc, filt[idx]);    // Filter-table lookup
    }
    (border, out)
}

pub fn solve() {
    let test = common::read_lines("input/test20.txt");
    let data = common::read_lines("input/input20.txt");

    // Part 1 tests
    let testf = read_filter(&test[0]);
    let test0 = read_image(&test[2..]);
    let test1 = enhance(&test0, &testf);
    let test2 = enhance(&test1, &testf);
    assert_eq!(count(&test0), 10);
    assert_eq!(count(&test1), 24);
    assert_eq!(count(&test2), 35);

    // Part 1 data
    let dataf = read_filter(&data[0]);
    let data0 = read_image(&data[2..]);
    let data1 = enhance(&data0, &dataf);
    let data2 = enhance(&data1, &dataf);
    println!("Part1: {}", count(&data2));

    // Part 2 tests
    let mut test50 = test2.clone();
    for _ in 2..50 {test50 = enhance(&test50, &testf);}
    assert_eq!(count(&test50), 3351);

    // Part 2 data
    let mut data50 = data2.clone();
    for _ in 2..50 {data50 = enhance(&data50, &dataf);}
    println!("Part2: {}", count(&data50));
}
