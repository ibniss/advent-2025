use crate::solution::Day;

const MULT_CHAR: char = '*';
const ADD_CHAR: char = '+';

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let raw_lines: Vec<_> = input.lines().map(|line| line.trim()).collect();
    let (operators_line, raw_num_lines) = raw_lines
        .split_last()
        .expect("Invalid input, could not split");

    let num_rows: Vec<Vec<u64>> = raw_num_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().expect("Invalid number"))
                .collect()
        })
        .collect();

    // Transpose rows to columns
    let col_count = num_rows[0].len();
    let columns: Vec<Vec<u64>> = (0..col_count)
        .map(|col_idx| num_rows.iter().map(|row| row[col_idx]).collect())
        .collect();

    let operators: Vec<char> = operators_line
        .split_whitespace()
        .map(|s| s.chars().next().expect("Empty operator"))
        .collect();

    (columns, operators)
}

/// Returns a Vec of problems where each problem is a sequence of numbers
/// (each column read top-to-bottom is one number) and a Vec of operators.
/// Columns are read right-to-left.
fn parse_input_rtl(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let raw_lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let (op_line, num_lines) = raw_lines
        .split_last()
        .expect("Invalid input, could not split");

    let max_len = raw_lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut operators: Vec<char> = Vec::new();
    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut current_numbers: Vec<u64> = Vec::new();

    // Iterate columns RTL - each column is one number (digits top-to-bottom)
    for col_idx in (0..max_len).rev() {
        // Parse this column's digits top-to-bottom into a single number,
        // tracking whether we found any digits to skip space-only columns
        let (num, has_digits) =
            num_lines.iter().fold((0u64, false), |(acc, found), line| {
                match line.get(col_idx) {
                    Some(b @ b'0'..=b'9') => (acc * 10 + (b - b'0') as u64, true),
                    _ => (acc, found),
                }
            });

        if has_digits {
            current_numbers.push(num);
        }

        // Check for operator - finalizes current problem
        match op_line.get(col_idx) {
            Some(b'*') => {
                problems.push(std::mem::take(&mut current_numbers));
                operators.push('*');
            }
            Some(b'+') => {
                problems.push(std::mem::take(&mut current_numbers));
                operators.push('+');
            }
            _ => {}
        }
    }

    (problems, operators)
}

/// Given a slice of columns, each column is a Vec of numbers, compute the puzzle
/// by applying the operators to the numbers
fn compute_puzzle(columns: &[Vec<u64>], operator_rows: &[char]) -> u64 {
    columns
        .iter()
        .zip(operator_rows)
        .map(|(col, op)| match *op {
            MULT_CHAR => col.iter().product::<u64>(),
            ADD_CHAR => col.iter().sum::<u64>(),
            _ => panic!("Invalid operator"),
        })
        .sum()
}

pub struct Solution;

impl Day for Solution {
    fn part1(input: &str) -> crate::solution::Solution {
        let (columns, operator_rows) = parse_input(input);
        compute_puzzle(&columns, &operator_rows).into()
    }

    fn part2(input: &str) -> crate::solution::Solution {
        let (columns, operator_rows) = parse_input_rtl(input);
        compute_puzzle(&columns, &operator_rows).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_solve() {
        let (columns, operator_rows) = parse_input(TEST_INPUT);
        let result = compute_puzzle(&columns, &operator_rows);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_solve_rtl() {
        let (columns, operator_rows) = parse_input_rtl(TEST_INPUT);
        let result = compute_puzzle(&columns, &operator_rows);
        assert_eq!(result, 3263827);
    }
}
