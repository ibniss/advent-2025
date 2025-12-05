use crate::solution::{Solution, SolutionPair};

const POSITION_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const ROLL: char = '@';
const BLANK: char = '.';

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_ascii_lowercase()).collect())
        .collect()
}

/// One-pass find of removable rolls
fn find_rolls(input: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut accessible_rolls = Vec::new();

    let max_x = input[0].len() as i32 - 1;
    let max_y = input.len() as i32 - 1;

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            // when a roll is found (@), count number of adjacent rolls (@)
            if *c == ROLL {
                let valid_positions: Vec<(i32, i32)> = POSITION_OFFSETS
                    .iter()
                    .map(|offset| (x as i32 + offset.0, y as i32 + offset.1))
                    .filter(|(px, py)| {
                        // check bounds
                        *px >= 0 && *py >= 0 && *px <= max_x && *py <= max_y
                    })
                    .collect();

                // Find rolls in valid positions
                let mut all_around_rolls = Vec::new();
                for (px, py) in &valid_positions {
                    let c = input
                        .get(*py as usize)
                        .and_then(|line| line.get(*px as usize))
                        .unwrap();

                    if *c == ROLL {
                        all_around_rolls.push((px, py));
                    }
                }

                // too many rolls around, skip
                if all_around_rolls.len() >= 4 {
                    continue;
                }

                // otherwise add it to the list of accessible rolls
                accessible_rolls.push((x as i32, y as i32));
            }
        }
    }

    accessible_rolls
}

/// Pass through the input and remove all rolls that can be removed.
/// Mutates the input to remove rolls and returns the list of removed rolls.
fn find_remove_rolls(input: &mut [Vec<char>]) -> Vec<(i32, i32)> {
    let mut accessible_rolls = Vec::new();

    let max_x = input[0].len() as i32 - 1;
    let max_y = input.len() as i32 - 1;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            // when a roll is found (@), count number of adjacent rolls (@)
            if input[y][x] == ROLL {
                let valid_positions: Vec<(i32, i32)> = POSITION_OFFSETS
                    .iter()
                    .map(|offset| (x as i32 + offset.0, y as i32 + offset.1))
                    .filter(|(px, py)| {
                        // check bounds
                        *px >= 0 && *py >= 0 && *px <= max_x && *py <= max_y
                    })
                    .collect();

                // Find rolls in valid positions
                let mut all_around_rolls = Vec::new();
                for (px, py) in &valid_positions {
                    let c = input
                        .get(*py as usize)
                        .and_then(|line| line.get(*px as usize))
                        .unwrap();

                    if *c == ROLL {
                        all_around_rolls.push((*px, *py));
                    }
                }

                // too many rolls around, skip
                if all_around_rolls.len() >= 4 {
                    continue;
                }

                // otherwise add it to the list of accessible rolls
                accessible_rolls.push((x as i32, y as i32));

                // remove the roll by setting the char to '.'
                input[y][x] = BLANK;
            }
        }
    }

    accessible_rolls
}

// Remove as many rolls as possible. Keeps running remove_rolls until no more rolls can be removed
fn remove_all_rolls(input: &mut [Vec<char>]) -> usize {
    let mut total_removed = 0;

    loop {
        let removed = find_remove_rolls(input);
        if removed.is_empty() {
            break;
        }
        total_removed += removed.len();
    }

    total_removed
}

pub fn solve(input: &str) -> SolutionPair {
    let parsed = parse_input(input);
    let one_pass_size = find_rolls(&parsed.clone()).len();
    let infinite_pass_size = remove_all_rolls(&mut parsed.clone());
    (
        Solution::from(one_pass_size),
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
        let rolls = find_rolls(&parsed);
        assert_eq!(rolls.len(), 13);
    }

    #[test]
    fn test_input_2() {
        let mut parsed = parse_input(TEST_INPUT);
        let total_removed = remove_all_rolls(&mut parsed);
        assert_eq!(total_removed, 43);
    }
}
