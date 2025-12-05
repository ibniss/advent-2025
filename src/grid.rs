#![allow(dead_code)]

/// A 2D grid backed by a flat 1D array for better cache locality.
#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn from_vecs(vecs: Vec<Vec<T>>) -> Self {
        let height = vecs.len();
        let width = vecs.first().map_or(0, |row| row.len());
        let data = vecs.into_iter().flatten().collect();
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
        Self::from_vecs(vecs)
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
}
