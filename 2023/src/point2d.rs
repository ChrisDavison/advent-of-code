#![allow(dead_code)]
use itertools::iproduct;

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Point2D {
    pub row: isize,
    pub col: isize,
}

impl Point2D {
    pub fn distance(&self, other: &Point2D) -> (isize, isize) {
        (
            self.col.abs_diff(other.col) as isize,
            self.row.abs_diff(other.row) as isize,
        )
    }

    pub fn adjacent(&self, other: &Point2D) -> bool {
        self.within_range(other, 1, 1)
    }

    pub fn within_range(&self, other: &Point2D, xdiff: isize, ydiff: isize) -> bool {
        let d = self.distance(other);
        d.0 <= xdiff && d.1 <= ydiff
    }

    pub fn surrounding(&self) -> Vec<Point2D> {
        let mut out = Vec::new();

        for rowdelta in [-1, 0, 1] {
            for coldelta in [-1, 0, 1] {
                if rowdelta == 0 && coldelta == 0 {
                    continue;
                }
                let newrow = self.row + rowdelta;
                let newcol = self.col + coldelta;
                if newrow < 0 || newcol < 0 {
                    continue;
                }
                out.push(Point2D {
                    row: newrow,
                    col: newcol,
                });
            }
        }
        out
    }

    pub fn surrounding_wrapped(&self, xlim: usize, ylim: usize) -> Vec<Point2D> {
        let mut out = Vec::new();

        for rowdelta in [-1, 0, 1] {
            for coldelta in [-1, 0, 1] {
                if rowdelta == 0 && coldelta == 0 {
                    continue;
                }
                out.push(Point2D {
                    row: (self.row + rowdelta) % ylim as isize,
                    col: (self.col + coldelta) % xlim as isize,
                });
            }
        }
        out
    }

    pub fn surrounding_clipped(&self, xlim: usize, ylim: usize) -> Vec<Point2D> {
        let mut out = Vec::new();

        for rowdelta in [-1, 0, 1] {
            for coldelta in [-1, 0, 1] {
                let newrow = self.row + rowdelta;
                let newcol = self.col + coldelta;
                if rowdelta == 0 && coldelta == 0 {
                    continue;
                }
                if newrow > ylim as isize || newcol > xlim as isize {
                    continue;
                }
                if newrow < 0 || newcol < 0 {
                    continue;
                }

                out.push(Point2D {
                    row: newrow,
                    col: newcol,
                });
            }
        }
        out
    }
}
