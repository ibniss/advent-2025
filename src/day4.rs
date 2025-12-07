use std::collections::HashSet;

use crate::grid::Grid;
use crate::solution::Day;

const ROLL: u8 = b'@';
const BLANK: u8 = b'.';

type Pos = (usize, usize);

/// Count accessible rolls (part 1)
fn count_accessible(grid: &Grid<u8>) -> usize {
    grid.iter()
        .filter(|&(x, y, &cell)| {
            cell == ROLL
                && grid
                    .neighbors(x, y)
                    .filter(|&(nx, ny)| grid[(nx, ny)] == ROLL)
                    .count()
                    < 4
        })
        .count()
}

/// Remove as many rolls as possible using candidate tracking (part 2)
fn remove_all_rolls(grid: &mut Grid<u8>) -> usize {
    let mut total_removed = 0;

    // Initialize candidates with only roll positions
    let mut candidates: HashSet<Pos> = grid
        .iter()
        .filter(|&(_, _, &cell)| cell == ROLL)
        .map(|(x, y, _)| (x, y))
        .collect();

    while !candidates.is_empty() {
        let mut next_candidates = HashSet::new();

        for (x, y) in candidates.drain() {
            if grid[(x, y)] != ROLL {
                continue;
            }

            let adjacent: Vec<Pos> = grid
                .neighbors(x, y)
                .filter(|&(nx, ny)| grid[(nx, ny)] == ROLL)
                .collect();

            if adjacent.len() < 4 {
                total_removed += 1;
                grid[(x, y)] = BLANK;
                next_candidates.extend(adjacent);
            }
        }

        candidates = next_candidates;
    }

    total_removed
}

pub struct Solution;

impl Day for Solution {
    fn part1(input: &str) -> crate::solution::Solution {
        let grid = Grid::parse(input);
        count_accessible(&grid).into()
    }

    fn part2(input: &str) -> crate::solution::Solution {
        let mut grid = Grid::parse(input);
        remove_all_rolls(&mut grid).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_input_1() {
        let grid = Grid::parse(TEST_INPUT);
        let rolls = count_accessible(&grid);
        assert_eq!(rolls, 13);
    }

    #[test]
    fn test_input_2() {
        let mut grid = Grid::parse(TEST_INPUT);
        let total_removed = remove_all_rolls(&mut grid);
        assert_eq!(total_removed, 43);
    }
}
