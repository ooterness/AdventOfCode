/// Common grid and matrix library functions for my Advent of Code solutions
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

/// Read an MxN single-digit grid as a vector-of-vectors.
#[allow(dead_code)]
pub fn read_grid<T: From<u8>>(filename: &str) -> Grid<T> {
    let lines = common::read_lines(filename);
    let rows = lines.iter()     // For each line...
        .map(|x| x.trim())      // Trim whitespace
        .map(|x| x.chars()      // Parse each digit
            .map(|d| d.to_digit(10).unwrap() as u8)
            .map(|n| T::from(n))
            .collect());
    Grid::<T>::new(rows.collect())
}

/// Size of an RxC grid.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct GridSize {
    pub r: usize,
    pub c: usize,
}

/// A "grid" is a matrix of values (rows then columns).
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
    pub size: GridSize,
}

#[allow(dead_code)]
impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Grid<T> {
        // Measure size using the first row.
        let rows = data.len();
        let cols = if rows > 0 {data[0].len()} else {0};
        // Confirm every row has the same length.
        for row in data.iter() {assert_eq!(row.len(), cols);}
        Grid {data:data, size: GridSize{r:rows,c:cols} }
    }

    pub fn get(&self, rc: &RowCol) -> Option<&T> {
        if (0 <= rc.r) && ((rc.r as usize) < self.size.r) &&
           (0 <= rc.c) && ((rc.c as usize) < self.size.c) {
            Some(&self.data[rc.r as usize][rc.c as usize])
        } else {None}
    }

    pub fn set(&mut self, rc: &RowCol, val: T) {
        if (0 <= rc.r) && ((rc.r as usize) < self.size.r) &&
           (0 <= rc.c) && ((rc.c as usize) < self.size.c) {
            self.data[rc.r as usize][rc.c as usize] = val;
        }
    }

    pub fn gets(&self, rc: &GridSize) -> Option<&T> {
        if (rc.r < self.size.r) && (rc.c < self.size.c) {
            Some(&self.data[rc.r][rc.c])
        } else {None}
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator { size:self.size.clone(), next: GridSize{r:0,c:0} }
    }
}

/// Iterator over every row/column coordinate in a Grid.
pub struct GridIterator {
    size: GridSize,
    next: GridSize,
}

impl<'a> Iterator for GridIterator {
    type Item = RowCol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.r < self.size.r && self.next.c < self.size.c {
            let result = Some( RowCol {
                r:self.next.r as i32,
                c:self.next.c as i32,
            } );
            if self.next.c + 1 < self.size.c {
                self.next.c += 1;   // Move to next column
            } else {
                self.next.c  = 0;   // Wrap to next row
                self.next.r += 1;
            }
            result
        } else {None}
    }
}

/// Abstract row/column position (often within a Grid).
#[allow(dead_code)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct RowCol {
    pub r: i32,
    pub c: i32,
}

#[allow(dead_code)]
impl RowCol {
    pub fn new(r: i32, c: usize) -> RowCol {
        RowCol {r:r as i32, c:c as i32}
    }

    // Move one space in a given direction
    pub fn nw(&self) -> RowCol { self.nw_by(1) }
    pub fn nn(&self) -> RowCol { self.nn_by(1) }
    pub fn ne(&self) -> RowCol { self.ne_by(1) }
    pub fn ee(&self) -> RowCol { self.ee_by(1) }
    pub fn se(&self) -> RowCol { self.se_by(1) }
    pub fn ss(&self) -> RowCol { self.ss_by(1) }
    pub fn sw(&self) -> RowCol { self.sw_by(1) }
    pub fn ww(&self) -> RowCol { self.ww_by(1) }

    // Move N spaces in a given direction
    pub fn nw_by(&self, d:i32) -> RowCol { RowCol {r:self.r-d, c:self.c-d} }
    pub fn nn_by(&self, d:i32) -> RowCol { RowCol {r:self.r-d, c:self.c  } }
    pub fn ne_by(&self, d:i32) -> RowCol { RowCol {r:self.r-d, c:self.c+d} }
    pub fn ee_by(&self, d:i32) -> RowCol { RowCol {r:self.r,   c:self.c+d} }
    pub fn se_by(&self, d:i32) -> RowCol { RowCol {r:self.r+d, c:self.c+d} }
    pub fn ss_by(&self, d:i32) -> RowCol { RowCol {r:self.r+d, c:self.c  } }
    pub fn sw_by(&self, d:i32) -> RowCol { RowCol {r:self.r+d, c:self.c-d} }
    pub fn ww_by(&self, d:i32) -> RowCol { RowCol {r:self.r,   c:self.c-d} }
}
