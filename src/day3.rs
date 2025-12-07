use std::collections::LinkedList;

use crate::solution::Day;

fn find_largest_pair_linear(line: &[u8]) -> u8 {
    let mut first = 0;
    let mut second = 0;

    for c in line {
        // then if second is larger than first, move it to first and take whatever number we're on
        // as second
        if second > first {
            first = second;
            second = *c;
            continue;
        } else if *c > second {
            // otherwise if current number is larger than second, move it to second
            second = *c;
        }
    }

    return first * 10 + second;
}

/// First attempt, initializing 12 numbers then considering each number linearly
#[allow(dead_code)]
fn find_largest_twelve_linear(line: &[u8]) -> u64 {
    let mut outputs = LinkedList::from_iter([0; 12]);

    for (i, c) in line.iter().enumerate() {
        // first initialize the 12 numbers, this is a bit inefficient but it happens 12 times per
        // row
        if i < 12 {
            *outputs.iter_mut().nth(i).unwrap() = *c;
            continue;
        }

        // then every time we iterate, add the new number to the end
        outputs.push_back(*c);

        // now we have 13 numbers, we have to pick one to drop
        // e.g. [234,234,234,234|2] (new marked) -> drop first 2 -> [342,342,342,342]
        // e.g. [342,342,342,342|7] (new marked) -> drop first 3-> [423,423,423,427]
        // e.g. [423,423,423,427|8] (new marked) -> drop second 2 -> [434,234,234,278]
        // whenever we encounter a pair where the latter is larger than the former
        // we drop the smaller one from the sequence
        let mut moved = false;
        let mut cursor = outputs.cursor_front_mut();
        while cursor.current().is_some() {
            let first = *cursor.current().unwrap();
            cursor.move_next(); // advance
            if let Some(second) = cursor.current() {
                if *second > first {
                    // move back to the prev element and remove it
                    cursor.move_prev();
                    cursor.remove_current().unwrap();
                    moved = true;
                    break;
                }
            } else {
                // end of sequence
                break;
            }
        }

        // if we haven't moved anything, the last element must be smallest so we just ignore it
        if !moved {
            outputs.pop_back();
        }
    }

    outputs
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| 10u64.pow(i as u32) * *n as u64)
        .sum()
}

/// Second attempt, no need to initialize N numbers, just keep adding to a stack
/// Its always best to remove items at the start, so we simply need to drop enough
/// items at the start to make sure we have N items left
fn find_largest_stack(line: &[u8], keep_count: usize) -> u64 {
    let mut stack: Vec<u8> = Vec::new();
    let to_remove = line.len() - keep_count;

    let mut removed = 0;
    for &digit in line {
        // Only consider this while we still need to remove items (can only drop so many so 12
        // stay) and while we have collected at least one item.
        // If the previous number is smaller than the next one, remove it.
        // We can keep removing the same item until we have the largest possible
        // as this is the most significant digit and is always better than e.g. letting it through
        // and then replacing another one later
        while removed < to_remove && !stack.is_empty() && *stack.last().unwrap() < digit {
            stack.pop();
            removed += 1;
        }
        stack.push(digit);
    }

    // If we didn't remove enough, truncate from the end
    stack.truncate(keep_count);

    stack.iter().fold(0, |acc, &d| acc * 10 + (d as u64))
}

fn process_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn solve_part1(input: &[Vec<u8>]) -> u64 {
    input
        .iter()
        .map(|line| find_largest_pair_linear(line))
        .map(u64::from)
        .sum()
}

fn solve_part2(input: &[Vec<u8>]) -> u64 {
    input
        .iter()
        // .map(|line| find_largest_twelve_linear(line))
        .map(|line| find_largest_stack(line, 12))
        .map(u64::from)
        .sum()
}

pub struct Solution;

impl Day for Solution {
    fn part1(input: &str) -> crate::solution::Solution {
        let processed = process_input(input);
        solve_part1(&processed).into()
    }

    fn part2(input: &str) -> crate::solution::Solution {
        let processed = process_input(input);
        solve_part2(&processed).into()
    }

    // Override to avoid parsing input twice
    fn solve(input: &str) -> crate::solution::SolutionPair {
        let processed = process_input(input);
        (
            solve_part1(&processed).into(),
            solve_part2(&processed).into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_input_1() {
        assert_eq!(solve_part1(&process_input(TEST_INPUT)), 357);
    }

    #[test]
    fn test_largest_12_1() {
        let line = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(find_largest_twelve_linear(&line), 987654321111);
    }

    #[test]
    fn test_largest_12_2() {
        let line = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        assert_eq!(find_largest_twelve_linear(&line), 811111111119);
    }

    #[test]
    fn test_largest_12_3() {
        let line = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        assert_eq!(find_largest_twelve_linear(&line), 434234234278);
    }

    #[test]
    fn test_input_2() {
        assert_eq!(solve_part2(&process_input(TEST_INPUT)), 3121910778619);
    }
}
