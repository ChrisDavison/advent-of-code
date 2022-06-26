use std::fmt;

/// Stride over slices of 2d array
///
/// e.g. with array
///
/// ```markdown
/// [[1, 2, 3]
///  [4, 5, 6]
///  [7, 8, 9]]
/// ```
///
///  and stride of width 2, height 2, an iteration would provide
///
///  ```markdown
///  [[1, 2]
///   [4, 5]]
///
///  [[2, 3]
///   [5, 6]] etc...
///  ```
#[derive(Clone, PartialEq)]
pub struct Strides<'a, T> {
    pub data: &'a Vec<Vec<T>>,
    pub row: usize,
    pub col: usize,
    pub row_lim: usize,
    pub col_lim: usize,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code, clippy::ptr_arg)]
impl<'a, T> Strides<'a, T>
where
    T: Clone,
{
    pub fn new(data: &'a Vec<Vec<T>>, width: usize, height: usize) -> Strides<T> {
        Strides {
            data,
            width,
            height,
            row_lim: data.len() - height,
            col_lim: data[0].len() - width + 1,
            row: 0,
            col: 0,
        }
    }
}

impl<'a, T> std::iter::Iterator for Strides<'a, T>
where
    T: Clone,
{
    type Item = Vec<Vec<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.row > self.row_lim {
            return None;
        }
        let mut out = Vec::new();
        for row_offset in 0..self.height {
            let actual_row = self.row + row_offset;
            let (y1, y2) = (self.col, self.col + self.width);
            out.push(self.data[actual_row][y1..y2].to_vec());
        }
        self.col = (self.col + 1) % self.col_lim;
        if self.col == 0 {
            self.row += 1;
        }
        Some(out)
    }
}

impl<'a, T> std::fmt::Display for Strides<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid_width = self.data[0].len();
        let grid_height = self.data.len();
        write!(
            f,
            "Stride: {}x{} tiles over {}x{} grid",
            self.width, self.height, grid_width, grid_height
        )
    }
}

impl<'a, T> std::fmt::Debug for Strides<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let grid_width = self.data[0].len();
        let grid_height = self.data.len();
        let t = core::any::type_name::<T>();
        write!(
            f,
            "Stride<{}> {{ {}x{} over {}x{} }}",
            t, self.width, self.height, grid_width, grid_height
        )
    }
}
