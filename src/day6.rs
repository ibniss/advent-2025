use crate::{
    grid::Grid,
    solution::{Solution, SolutionPair},
};

const MULT_CHAR: char = '*';
const ADD_CHAR: char = '+';

fn parse_input(input: &str) -> (Grid<u64>, Vec<char>) {
    let raw_lines = input.lines().map(|line| line.trim()).collect::<Vec<_>>();
    let line_count = raw_lines.len();

    let num_rows: Vec<Vec<u64>> = raw_lines[0..line_count - 1]
        .iter()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|num| num.parse().expect("Invalid number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let operator_rows: Vec<char> = raw_lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|str| str.chars().next().expect("Empty operator"))
        .collect();

    let rows_grid = Grid::from_rows(num_rows);

    (rows_grid, operator_rows)
}

/// Returns a Vec of columns where each column is a sequence of numbers
/// (read top-to-bottom as digits of RTL numbers) and a Vec of operators
fn parse_input_rtl(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let raw_lines = input.lines().map(|line| line).collect::<Vec<_>>();
    let line_count = raw_lines.len();

    // For RTL, keep track of how many digits are on each row so we can use this info
    // to parse numbers differently
    let mut operator_rows: Vec<(usize, char)> =
        raw_lines
            .last()
            .unwrap()
            .chars()
            .fold(Vec::new(), |mut acc, ch| {
                // increment count of digits on this row
                if ch.is_ascii_whitespace() {
                    acc.last_mut().map(|(count, _)| *count += 1);
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
    let num_rows: Vec<Vec<Vec<char>>> = raw_lines[0..line_count - 1]
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
                        let spaces = std::iter::repeat(' ').take(*digit_count - remaining.len());
                        let next_num = remaining.chars().chain(spaces).rev().collect::<Vec<_>>();
                        *remaining = "";
                        return Some(next_num);
                    }

                    let (next_num, res) = remaining.split_at(*digit_count + 1);
                    *remaining = res;
                    // slice off the separator
                    let next_num = next_num[0..*digit_count].chars().rev().collect::<Vec<_>>();
                    Some(next_num)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

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

fn compute_puzzle(rows_grid: &Grid<u64>, operator_rows: &[char]) -> u64 {
    let (mult_ops, add_ops): (Vec<_>, Vec<_>) = operator_rows
        .iter()
        .enumerate()
        .partition(|(_, op)| **op == MULT_CHAR);

    let mult_total: u64 = mult_ops
        .iter()
        .map(|(x, _)| rows_grid.iter_col(*x).product::<u64>())
        .sum();

    let add_total: u64 = add_ops
        .iter()
        .map(|(x, _)| rows_grid.iter_col(*x).sum::<u64>())
        .sum();

    mult_total + add_total
}

fn compute_puzzle_rtl(columns: &[Vec<u64>], operator_rows: &[char]) -> u64 {
    let (mult_ops, add_ops): (Vec<_>, Vec<_>) = operator_rows
        .iter()
        .enumerate()
        .partition(|(_, op)| **op == MULT_CHAR);

    let mult_total: u64 = mult_ops
        .iter()
        .map(|(idx, _)| columns[*idx].iter().product::<u64>())
        .sum();

    let add_total: u64 = add_ops
        .iter()
        .map(|(idx, _)| columns[*idx].iter().sum::<u64>())
        .sum();

    mult_total + add_total
}

pub fn solve(input: &str) -> SolutionPair {
    let (rows_grid, operator_rows) = parse_input(input);
    let result = compute_puzzle(&rows_grid, &operator_rows);

    let (columns_rtl, operator_rows_rtl) = parse_input_rtl(input);
    let result_rtl = compute_puzzle_rtl(&columns_rtl, &operator_rows_rtl);

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
        let (rows_grid, operator_rows) = parse_input(TEST_INPUT);
        let result = compute_puzzle(&rows_grid, &operator_rows);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_solve_rtl() {
        let (columns, operator_rows) = parse_input_rtl(TEST_INPUT);
        let result = compute_puzzle_rtl(&columns, &operator_rows);
        assert_eq!(result, 3263827);
    }
}
