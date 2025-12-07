use crate::{
    grid::Grid,
    solution::{Solution, SolutionPair},
};

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

/// Returns a Vec of columns where each column is a sequence of numbers
/// (read top-to-bottom as digits of RTL numbers) and a Vec of operators
fn parse_input_rtl(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let raw_lines = input.lines().collect::<Vec<_>>();
    let (operators_line, raw_num_lines) = raw_lines
        .split_last()
        .expect("Invalid input, could not split");

    // For RTL, keep track of how many digits are on each row so we can use this info
    // to parse numbers differently
    let mut operator_rows: Vec<(usize, char)> =
        operators_line.chars().fold(Vec::new(), |mut acc, ch| {
            // increment count of digits on this row
            if ch.is_ascii_whitespace() {
                if let Some((count, _)) = acc.last_mut() {
                    *count += 1;
                }
            } else if ch == MULT_CHAR || ch == ADD_CHAR {
                // operator found, add with 0 count
                acc.push((0, ch));
            } else {
                panic!("Invalid char found");
            }
            acc
        });
    // increment last operator count by 1 to account for the separators
    operator_rows.last_mut().map(|(count, _)| *count += 1);

    // First do a pass through each row, separating each number into its own string digits
    // utilizing the digit count to split correctly with padding
    // e.g.
    // ["123, "328", " 51", "64 "] -> reversed -> [[" ", "4", "6"], ["1", "5", " "], ...]
    let num_rows: Vec<Vec<Vec<char>>> = raw_num_lines
        .iter()
        .map(|&line| {
            operator_rows
                .iter()
                .scan(line, |remaining, (digit_count, _)| {
                    // stop scanning once we've reached the end of the line
                    if remaining.is_empty() {
                        return None;
                    }

                    if remaining.len() < (*digit_count + 1) {
                        // Pad to the right with spaces to match the digit count
                        let spaces = std::iter::repeat_n(' ', *digit_count - remaining.len());
                        let next_num = remaining.chars().chain(spaces).rev().collect();
                        *remaining = "";
                        return Some(next_num);
                    }

                    let (next_num, res) = remaining.split_at(*digit_count + 1);
                    *remaining = res;
                    // slice off the separator
                    let next_num = next_num[0..*digit_count].chars().rev().collect();
                    Some(next_num)
                })
                .collect()
        })
        .collect();

    let row_chars_grid = Grid::from_rows(num_rows);

    let parsed_columns: Vec<Vec<u64>> = row_chars_grid
        .iter_cols()
        .map(|col_iter| {
            // Turn the column into a sub-grid of chars, which are now in the right order
            let sub_grid = Grid::from_rows(col_iter.cloned().collect());

            // now, we have shape of e.g.
            // [" ", "4, "6"]
            // [" ", "3", "2"]
            // ["4", "1", "3"]
            // Most significant digit is at the top.
            // So go through each column and collect the digits in order
            sub_grid
                .iter_cols()
                .map(|sub_col_iter| {
                    // filter out non-digits (gets rid of leading whitespace) and fold into a number
                    sub_col_iter
                        .filter(|&ch| ch.is_ascii_digit())
                        .fold(0, |acc, ch| {
                            acc * 10 + ch.to_digit(10).expect("Invalid digit") as u64
                        })
                })
                .collect()
        })
        .collect();

    // extract just the operators, no need for the counts anymore
    let operators: Vec<char> = operator_rows.iter().map(|(_, op)| *op).collect();

    // Return parsed_columns directly - each column may have different number of values
    // (different digit counts), so we can't use a uniform Grid
    (parsed_columns, operators)
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

pub fn solve(input: &str) -> SolutionPair {
    let (columns, operator_rows) = parse_input(input);
    let result = compute_puzzle(&columns, &operator_rows);

    let (columns_rtl, operator_rows_rtl) = parse_input_rtl(input);
    let result_rtl = compute_puzzle(&columns_rtl, &operator_rows_rtl);

    (Solution::from(result), Solution::from(result_rtl))
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
