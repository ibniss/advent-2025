use std::ops::RangeInclusive;

use crate::solution::{Solution, SolutionPair};

fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (ranges, values) = input
        .split_once("\n\n")
        .expect("Invalid input, could not find split");

    let mut ranges = ranges
        .lines()
        .map(|line| {
            let parts = line
                .trim()
                .split_once("-")
                .expect("Invalid line, could not find - to split");
            let start: usize = parts.0.parse().expect("Invalid start");
            let end: usize = parts.1.parse().expect("Invalid end");
            start..=end
        })
        .collect::<Vec<_>>();

    // Ensure ranges are sorted by start
    ranges.sort_by(|a, b| a.start().cmp(&b.start()));

    let mut merged_ranges: Vec<RangeInclusive<usize>> = Vec::new();

    // Merge overlapping ranges
    let mut current_range: Option<RangeInclusive<usize>> = None;

    for i in 0..ranges.len() {
        let next_range = ranges.get(i).expect("No next range found");

        if let Some(cur_range) = &current_range {
            // check if range overlaps with current range
            if next_range.start() <= cur_range.end() {
                // make sure to take max as they could be subsets
                current_range = Some(RangeInclusive::new(
                    *cur_range.start(),
                    *cur_range.end().max(next_range.end()),
                ));
            } else {
                // otherwise push current range to merged_ranges
                merged_ranges.push(cur_range.clone());
                current_range = Some(next_range.clone());
            }
        } else {
            current_range = Some(next_range.clone());
        }
    }

    // include last range
    if let Some(cur_range) = current_range {
        merged_ranges.push(cur_range);
    }

    let values = values
        .lines()
        .map(|line| line.trim().parse().expect("Invalid value"))
        .collect::<Vec<usize>>();

    (merged_ranges, values)
}

fn filter_ingredients(ranges: &[RangeInclusive<usize>], values: &[usize]) -> Vec<usize> {
    return values
        .iter()
        .filter(|&v| ranges.iter().any(|r| r.contains(v)))
        .cloned()
        .collect();
}

fn count_all_fresh(ranges: &[RangeInclusive<usize>]) -> usize {
    return ranges
        .iter()
        // inclusive so +1
        .fold(0, |acc, range| acc + (range.end() - range.start() + 1));
}

pub fn solve(input: &str) -> SolutionPair {
    let (ranges, values) = parse_input(input);
    let ingredients = filter_ingredients(&ranges, &values);
    let count = count_all_fresh(&ranges);
    (Solution::from(ingredients.len()), Solution::from(count))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_solve() {
        let (ranges, values) = parse_input(TEST_INPUT);
        let ingredients = filter_ingredients(&ranges, &values);
        assert_eq!(ingredients.len(), 3);
    }

    #[test]
    fn test_solve_part_2() {
        let (ranges, _) = parse_input(TEST_INPUT);
        let count = count_all_fresh(&ranges);
        assert_eq!(count, 14);
    }
}
