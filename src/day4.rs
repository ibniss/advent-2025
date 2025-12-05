use std::collections::HashSet;

use crate::solution::{Solution, SolutionPair};

const ROLL: u8 = b'@';
const BLANK: u8 = b'.';

type Grid = Vec<Vec<u8>>;
type Pos = (usize, usize);

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

/// Get valid neighboring positions for a given (x, y) coordinate
#[inline]
fn neighbors(x: usize, y: usize, width: usize, height: usize) -> impl Iterator<Item = Pos> {
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

/// Count accessible rolls (part 1)
fn count_accessible(grid: &Grid) -> usize {
    let (width, height) = (grid[0].len(), grid.len());

    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell == ROLL {
                    let adjacent = neighbors(x, y, width, height)
                        .filter(|&(nx, ny)| grid[ny][nx] == ROLL)
                        .count();
                    (adjacent < 4).then_some(1)
                } else {
                    None
                }
            })
        })
        .sum()
}

/// Remove as many rolls as possible using candidate tracking (part 2)
fn remove_all_rolls(grid: &mut Grid) -> usize {
    let (width, height) = (grid[0].len(), grid.len());
    let mut total_removed = 0;

    // Initialize candidates with only roll positions
    let mut candidates: HashSet<Pos> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &cell)| cell == ROLL)
                .map(move |(x, _)| (x, y))
        })
        .collect();

    while !candidates.is_empty() {
        let mut next_candidates = HashSet::new();

        for (x, y) in candidates.drain() {
            if grid[y][x] != ROLL {
                continue;
            }

            let adjacent: Vec<Pos> = neighbors(x, y, width, height)
                .filter(|&(nx, ny)| grid[ny][nx] == ROLL)
                .collect();

            if adjacent.len() < 4 {
                total_removed += 1;
                grid[y][x] = BLANK;
                next_candidates.extend(adjacent);
            }
        }

        candidates = next_candidates;
    }

    total_removed
}

pub fn solve(input: &str) -> SolutionPair {
    let parsed = parse_input(input);
    let one_pass_count = count_accessible(&parsed);
    let infinite_pass_size = remove_all_rolls(&mut parsed.clone());
    (
        Solution::from(one_pass_count),
        Solution::from(infinite_pass_size),
    )
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
        let parsed = parse_input(TEST_INPUT);
        let rolls = count_accessible(&parsed);
        assert_eq!(rolls, 13);
    }

    #[test]
    fn test_input_2() {
        let mut parsed = parse_input(TEST_INPUT);
        let total_removed = remove_all_rolls(&mut parsed);
        assert_eq!(total_removed, 43);
    }
}
