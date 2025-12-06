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
                .split_once('-')
                .expect("Invalid line, could not find - to split");
            let start: usize = parts.0.parse().expect("Invalid start");
            let end: usize = parts.1.parse().expect("Invalid end");
            // handle ranges that are backwards
            start.min(end)..=start.max(end)
        })
        .collect::<Vec<_>>();

    // Ensure ranges are sorted by start
    ranges.sort_by_key(|range| *range.start());

    let merged_ranges: Vec<RangeInclusive<usize>> =
        ranges.into_iter().fold(Vec::new(), |mut acc, range| {
            match acc.last_mut() {
                // there is a previous range and it overlaps, merge
                Some(last) if *range.start() <= (last.end() + 1) => {
                    *last = RangeInclusive::new(*last.start(), *range.end().max(last.end()));
                }
                // initial case or range is not overlapping, just proceed to next
                _ => {
                    acc.push(range);
                }
            }
            acc
        });

    let values = values
        .lines()
        .map(|line| line.trim().parse().expect("Invalid value"))
        .collect::<Vec<usize>>();

    (merged_ranges, values)
}

// Naive linear search, first attempt
// fn filter_ingredients(ranges: &[RangeInclusive<usize>], values: &[usize]) -> Vec<usize> {
//     values
//         .iter()
//         .filter(|&v| ranges.iter().any(|r| r.contains(v)))
//         .cloned()
//         .collect()
// }

// Binary search
fn filter_ingredients_bs(ranges: &[RangeInclusive<usize>], values: &[usize]) -> Vec<usize> {
    values
        .iter()
        .filter(|&v| {
            ranges
                .binary_search_by(|r| {
                    if v < r.start() {
                        std::cmp::Ordering::Greater
                    } else if v > r.end() {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .is_ok()
        })
        .cloned()
        .collect()
}

fn count_all_fresh(ranges: &[RangeInclusive<usize>]) -> usize {
    ranges
        .iter()
        // inclusive so +1
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

pub fn solve(input: &str) -> SolutionPair {
    let (ranges, values) = parse_input(input);
    let ingredients = filter_ingredients_bs(&ranges, &values);
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
        let ingredients = filter_ingredients_bs(&ranges, &values);
        assert_eq!(ingredients.len(), 3);
    }

    #[test]
    fn test_solve_part_2() {
        let (ranges, _) = parse_input(TEST_INPUT);
        let count = count_all_fresh(&ranges);
        assert_eq!(count, 14);
    }
}
