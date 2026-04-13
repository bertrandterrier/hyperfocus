use std::cmp::{ PartialEq, Eq, Ord, PartialOrd, Ordering };

/// One-dimensional object on two dimensional axis.
#[derive(Clone, Copy)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
    pub fn col(&self) -> i32 { self.0 }
    pub fn row(&self) -> i32 { self.1 }
    pub fn unpack(&self) -> (i32, i32) {
        (self.0, self.1)
    }
}

/// Surface as size from `Point(COLUMN, ROW)` to `Point(COLUMN, ROW)`.
pub struct PtSize(pub Point, pub Point);

impl PtSize {
    pub fn new(start: Point, stop: Point) -> Self {
        Self(start, stop)
    }
    pub fn unpack(&self) -> (Point, Point) {
        (self.0, self.1)
    }
}

/// Iterator for `PtSize`
pub struct PtSzIter {
    stop: Point,
    curr_row: i32,
    curr_col: i32,
}

impl PtSzIter {
    pub fn new(size: PtSize) -> Self {
        let (start, stop) = size.unpack();
        let (curr_col, curr_row) = start.unpack();
        Self { stop, curr_col, curr_row }
    }
}


impl Iterator for PtSzIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row >= self.stop.row() {
            None
        } else if self.curr_col >= self.stop.col() {
            self.curr_col = 0;
            self.curr_row += 1;
            self.next()
        } else {
            Some(Point(self.curr_col, self.curr_row))
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match (self.0.cmp(&rhs.0), self.1.cmp(&rhs.1)) {
            (Ordering::Equal, o) | (o, Ordering::Equal) => Some(o),
            (Ordering::Greater, Ordering::Less) | (Ordering::Less, Ordering::Greater) => None,
            (o, _) => Some(o),
        }
    }
}
