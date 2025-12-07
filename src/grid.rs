#![allow(dead_code)]

/// A 2D grid backed by a flat 1D array for better cache locality.
#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    /// Create a new grid with the given dimensions, filled with the default value.
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    /// Create a grid from a 2D vector (row-major order).
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        let height = rows.len();
        let width = rows.first().map_or(0, |row| row.len());
        let data = rows.into_iter().flatten().collect();
        Self {
            data,
            width,
            height,
        }
    }

    /// Create a grid from a 2D vector of columns.
    pub fn from_cols(cols: Vec<Vec<T>>) -> Self {
        let width = cols.len();
        let height = cols.first().map_or(0, |col| col.len());
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for col in &cols {
                data.push(col[y].clone());
            }
        }
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Convert (x, y) to a flat index.
    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Get a reference to the cell at (x, y), or None if out of bounds.
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[self.index(x, y)])
        } else {
            None
        }
    }

    /// Get a mutable reference to the cell at (x, y), or None if out of bounds.
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    /// Set the cell at (x, y). Panics if out of bounds.
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        let idx = self.index(x, y);
        self.data[idx] = value;
    }

    /// Iterate over all cells with their (x, y) coordinates.
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data.iter().enumerate().map(|(i, val)| {
            let x = i % self.width;
            let y = i / self.width;
            (x, y, val)
        })
    }

    /// Iterate over all cells in a given row (left to right).
    /// Panics if the row index is out of bounds.
    pub fn iter_row(&self, y: usize) -> impl Iterator<Item = &T> {
        assert!(
            y < self.height,
            "row index {y} out of bounds (height: {})",
            self.height
        );
        let start = self.index(0, y);
        let end = start + self.width;
        self.data[start..end].iter()
    }

    /// Iterate over all cells in a given column (top to bottom).
    /// Panics if the column index is out of bounds.
    pub fn iter_col(&self, x: usize) -> impl Iterator<Item = &T> + '_ {
        assert!(
            x < self.width,
            "column index {x} out of bounds (width: {})",
            self.width
        );
        (0..self.height).map(move |y| &self.data[self.index(x, y)])
    }

    /// Iterate over all rows, yielding an iterator for each row.
    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.height).map(move |y| self.iter_row(y))
    }

    /// Iterate over all columns, yielding an iterator for each column.
    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T> + '_> + '_ {
        (0..self.width).map(move |x| self.iter_col(x))
    }

    /// Get the 8 neighboring positions (excluding out-of-bounds).
    #[inline]
    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let width = self.width;
        let height = self.height;
        [
            (x.wrapping_sub(1), y.wrapping_sub(1)),
            (x, y.wrapping_sub(1)),
            (x + 1, y.wrapping_sub(1)),
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x.wrapping_sub(1), y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .into_iter()
        .filter(move |(nx, ny)| *nx < width && *ny < height)
    }
}

impl Grid<u8> {
    /// Parse a grid from a string where each character becomes a u8.
    pub fn parse(input: &str) -> Self {
        let vecs: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        Self::from_rows(vecs)
    }
}

impl<T: Clone> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

impl<T: Clone> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let idx = y * self.width + x;
        &mut self.data[idx]
    }
}

/// Display for Grid<u8> that renders bytes as characters
impl std::fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.data[self.index(x, y)] as char)?;
            }
            if y < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

/// Display for Grid<char> that renders characters directly
impl std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.data[self.index(x, y)])?;
            }
            if y < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

/// Generic Debug for any Grid<T> where T: Debug
impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Calculate max width for alignment
        let max_width = self
            .data
            .iter()
            .map(|v| format!("{:?}", v).len())
            .max()
            .unwrap_or(1);

        writeln!(f, "Grid {}x{} {{", self.width, self.height)?;
        for y in 0..self.height {
            write!(f, "  ")?;
            for x in 0..self.width {
                if x > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{:>w$?}", self.data[y * self.width + x], w = max_width)?;
            }
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_basic() {
        let mut grid = Grid::new(3, 3, 0u8);
        grid.set(1, 1, 5);
        assert_eq!(grid[(1, 1)], 5);
        assert_eq!(grid[(0, 0)], 0);
    }

    #[test]
    fn test_grid_parse() {
        let grid = Grid::parse("abc\ndef");
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid[(0, 0)], b'a');
        assert_eq!(grid[(2, 1)], b'f');
    }

    #[test]
    fn test_neighbors() {
        let grid = Grid::<u8>::new(3, 3, 0);

        // Corner - should have 3 neighbors
        let corner: Vec<_> = grid.neighbors(0, 0).collect();
        assert_eq!(corner.len(), 3);

        // Center - should have 8 neighbors
        let center: Vec<_> = grid.neighbors(1, 1).collect();
        assert_eq!(center.len(), 8);
    }

    #[test]
    fn test_iter_row() {
        let grid = Grid::parse("abc\ndef\nghi");

        // First row
        let row0: Vec<_> = grid.iter_row(0).copied().collect();
        assert_eq!(row0, vec![b'a', b'b', b'c']);

        // Second row
        let row1: Vec<_> = grid.iter_row(1).copied().collect();
        assert_eq!(row1, vec![b'd', b'e', b'f']);

        // Third row
        let row2: Vec<_> = grid.iter_row(2).copied().collect();
        assert_eq!(row2, vec![b'g', b'h', b'i']);
    }

    #[test]
    fn test_iter_col() {
        let grid = Grid::parse("abc\ndef\nghi");

        // First column
        let col0: Vec<_> = grid.iter_col(0).copied().collect();
        assert_eq!(col0, vec![b'a', b'd', b'g']);

        // Second column
        let col1: Vec<_> = grid.iter_col(1).copied().collect();
        assert_eq!(col1, vec![b'b', b'e', b'h']);

        // Third column
        let col2: Vec<_> = grid.iter_col(2).copied().collect();
        assert_eq!(col2, vec![b'c', b'f', b'i']);
    }

    #[test]
    fn test_display_u8() {
        let grid = Grid::parse("abc\ndef\nghi");
        let display = format!("{}", grid);
        assert_eq!(display, "abc\ndef\nghi");
    }

    #[test]
    fn test_debug_u8() {
        let grid = Grid::parse("ab\ncd");
        let debug = format!("{:?}", grid);
        // u8 Debug shows numeric values, right-aligned
        assert_eq!(debug, "Grid 2x2 {\n   97  98\n   99 100\n}");
    }

    #[test]
    fn test_debug_numeric() {
        let grid = Grid::from_rows(vec![vec![1, 20], vec![300, 4]]);
        let debug = format!("{:?}", grid);
        // Numbers are right-aligned to max width
        assert_eq!(debug, "Grid 2x2 {\n    1  20\n  300   4\n}");
    }
}
