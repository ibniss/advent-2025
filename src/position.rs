#![allow(dead_code)]

/// A 2D position with named fields.
/// Uses (x, y) convention where x is column and y is row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    /// Create a new position from x (column) and y (row).
    #[inline]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Create a position from (col, row) - same as new() but more explicit.
    #[inline]
    pub const fn from_col_row(col: usize, row: usize) -> Self {
        Self { x: col, y: row }
    }

    /// Create a position from (row, col) - for when you're thinking row-first.
    #[inline]
    pub const fn from_row_col(row: usize, col: usize) -> Self {
        Self { x: col, y: row }
    }

    /// Get column (x coordinate).
    #[inline]
    pub const fn col(&self) -> usize {
        self.x
    }

    /// Get row (y coordinate).
    #[inline]
    pub const fn row(&self) -> usize {
        self.y
    }

    /// Convert to (x, y) tuple.
    #[inline]
    pub const fn as_xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Convert to (row, col) tuple.
    #[inline]
    pub const fn as_row_col(&self) -> (usize, usize) {
        (self.y, self.x)
    }

    /// Move up (decrease y), returning None if at y=0.
    #[inline]
    pub fn up(&self) -> Option<Self> {
        self.y.checked_sub(1).map(|y| Self { x: self.x, y })
    }

    /// Move down (increase y).
    #[inline]
    pub const fn down(&self) -> Self {
        Self { x: self.x, y: self.y + 1 }
    }

    /// Move left (decrease x), returning None if at x=0.
    #[inline]
    pub fn left(&self) -> Option<Self> {
        self.x.checked_sub(1).map(|x| Self { x, y: self.y })
    }

    /// Move right (increase x).
    #[inline]
    pub const fn right(&self) -> Self {
        Self { x: self.x + 1, y: self.y }
    }

    /// Manhattan distance to another position.
    #[inline]
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<(usize, usize)> for Position {
    /// Convert from (x, y) tuple.
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl From<Position> for (usize, usize) {
    /// Convert to (x, y) tuple.
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pos = Position::new(3, 5);
        assert_eq!(pos.x, 3);
        assert_eq!(pos.y, 5);
        assert_eq!(pos.col(), 3);
        assert_eq!(pos.row(), 5);
    }

    #[test]
    fn test_from_row_col() {
        // When thinking "row 2, col 3"
        let pos = Position::from_row_col(2, 3);
        assert_eq!(pos.row(), 2);
        assert_eq!(pos.col(), 3);
        assert_eq!(pos.x, 3);
        assert_eq!(pos.y, 2);
    }

    #[test]
    fn test_movement() {
        let pos = Position::new(5, 5);

        assert_eq!(pos.up(), Some(Position::new(5, 4)));
        assert_eq!(pos.down(), Position::new(5, 6));
        assert_eq!(pos.left(), Some(Position::new(4, 5)));
        assert_eq!(pos.right(), Position::new(6, 5));

        // Edge cases
        let origin = Position::new(0, 0);
        assert_eq!(origin.up(), None);
        assert_eq!(origin.left(), None);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = Position::new(0, 0);
        let b = Position::new(3, 4);
        assert_eq!(a.manhattan_distance(&b), 7);
    }

    #[test]
    fn test_conversions() {
        let pos = Position::new(3, 5);
        assert_eq!(pos.as_xy(), (3, 5));
        assert_eq!(pos.as_row_col(), (5, 3));

        let from_tuple: Position = (3, 5).into();
        assert_eq!(from_tuple, pos);
    }
}
