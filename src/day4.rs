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

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_ascii_lowercase()).collect())
        .collect()
}

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

pub fn solve(input: &str) -> SolutionPair {
    let parsed = parse_input(input);
    let rolls = find_rolls(&parsed);
    (Solution::from(rolls.len()), Solution::from(0))
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
}
