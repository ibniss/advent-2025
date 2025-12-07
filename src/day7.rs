use std::collections::{HashMap, HashSet};

use crate::{grid::Grid, position::Position, solution::Day};

const START_LOCATION: char = 'S';
const SPLITTER: char = '^';
const EMPTY: char = '.';
const BEAM: char = '|';

/// Visualize the grid with beam positions marked.
/// Beams are shown as '*', overlaid on the original grid.
#[allow(dead_code)]
fn visualize_beams(grid: &Grid<char>, beams: &HashSet<Position>) -> String {
    let mut result = String::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let pos = Position::new(x, y);
            if beams.contains(&pos) {
                // Show beam, but preserve splitters/start if beam is on them
                let cell = grid[pos];
                if cell == SPLITTER || cell == START_LOCATION {
                    result.push(cell);
                } else {
                    result.push(BEAM);
                }
            } else {
                result.push(grid[pos]);
            }
        }
        result.push('\n');
    }
    result
}

fn parse_input(input: &str) -> Grid<char> {
    Grid::from_rows(input.lines().map(|line| line.chars().collect()).collect())
}

/// State for beam simulation: (current beam positions, split count)
type BeamState = (HashSet<Position>, usize);

fn count_beams(grid: &Grid<char>) -> usize {
    // start position - find 'S' in the first row
    let init_position: Position = grid
        .iter_row(0)
        .enumerate()
        .find_map(|(col, c)| {
            if *c == START_LOCATION {
                Some(Position::new(col, 0))
            } else {
                None
            }
        })
        .expect("No start location found");

    let init_state: BeamState = (HashSet::from([init_position]), 0);

    grid.iter_rows()
        .enumerate()
        .skip(1)
        // fold over each row, tracking (current beams, split count)
        .fold(init_state, |(current, splits), (row_idx, row_iter)| {
            // visualize current state
            // eprintln!("Row {} (splits so far: {}):\n{}", row_idx, splits, visualize_beams(grid, &current));

            // find splitters in this row
            let splitters: HashSet<Position> = row_iter
                .enumerate()
                .filter_map(|(col_idx, &c)| {
                    if c == SPLITTER {
                        Some(Position::new(col_idx, row_idx))
                    } else {
                        None
                    }
                })
                .collect();

            // compute next beam positions and count splits
            let (next, new_splits): (HashSet<Position>, usize) =
                current
                    .iter()
                    .fold((HashSet::new(), 0), |(mut next, split_count), pos| {
                        let next_pos = pos.down();
                        // if it hits a splitter, split into two beams adjacent to it
                        if splitters.contains(&next_pos) {
                            next.insert(next_pos.left().expect("splitter at left edge"));
                            next.insert(next_pos.right());
                            (next, split_count + 1)
                        } else {
                            next.insert(next_pos);
                            (next, split_count)
                        }
                    });

            (next, splits + new_splits)
        })
        .1 // return split count
}

/// Count timelines using position
fn count_timelines(grid: &Grid<char>) -> usize {
    // start position - find 'S' in the first row
    let init_position: Position = grid
        .iter_row(0)
        .enumerate()
        .find_map(|(col, c)| {
            if *c == START_LOCATION {
                Some(Position::new(col, 0))
            } else {
                None
            }
        })
        .expect("No start location found");

    // Map of position -> number of timelines at that position
    let init_state: HashMap<Position, usize> = HashMap::from([(init_position, 1)]);

    // pre-collect splitters for each row to avoid repeated iteration
    let splitters_by_row: Vec<HashSet<Position>> = grid
        .iter_rows()
        .enumerate()
        .map(|(row_idx, row_iter)| {
            row_iter
                .enumerate()
                .filter_map(|(col_idx, &c)| {
                    if c == SPLITTER {
                        Some(Position::new(col_idx, row_idx))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    (1..grid.height())
        // fold over each row, tracking position -> timeline count
        .fold(init_state, |current, row_idx| {
            let splitters = &splitters_by_row[row_idx];

            // compute next positions with counts
            current
                .iter()
                .flat_map(|(&pos, &count)| {
                    let next_pos = pos.down();
                    // if it hits a splitter, split into two positions
                    if splitters.contains(&next_pos) {
                        vec![
                            (next_pos.left().expect("splitter at left edge"), count),
                            (next_pos.right(), count),
                        ]
                    } else {
                        vec![(next_pos, count)]
                    }
                })
                .fold(HashMap::new(), |mut acc, (pos, count)| {
                    *acc.entry(pos).or_insert(0) += count;
                    acc
                })
        })
        .values()
        .sum()
}

pub struct Solution;

impl Day for Solution {
    fn part1(input: &str) -> crate::solution::Solution {
        let grid = parse_input(input);
        count_beams(&grid).into()
    }

    fn part2(input: &str) -> crate::solution::Solution {
        let grid = parse_input(input);
        count_timelines(&grid).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_input() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(count_beams(&grid), 21);
    }

    #[test]
    fn test_input_2() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(count_timelines(&grid), 40);
    }
}
