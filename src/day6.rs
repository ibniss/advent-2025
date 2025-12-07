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

fn parse_input_rtl(input: &str) -> (Grid<&str>, Vec<(usize, char)>) {
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
    // ["123, "328", " 51", "64 "]
    let num_rows: Vec<Vec<&str>> = raw_lines[0..line_count - 1]
        .iter()
        .map(|&line| {
            dbg!(line);
            operator_rows
                .iter()
                .scan(line, |remaining, (digit_count, _)| {
                    // stop scanning once we've reached the end of the line
                    if remaining.is_empty() {
                        return None;
                    }

                    if remaining.len() < (*digit_count + 1) {
                        let next_num = *remaining;
                        dbg!(next_num);
                        *remaining = "";
                        return Some(next_num);
                    }

                    let (next_num, res) = remaining.split_at(*digit_count + 1);
                    *remaining = res;
                    // slice off the separator
                    let next_num = &next_num[0..*digit_count];
                    Some(next_num)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // TODO: for each col e.g. ["64 ", "23 ", "314"], need to
    // go through the numbers (maybe reverse strings first?) and reorient the matrix

    (Grid::from_rows(num_rows), operator_rows)
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

fn compute_puzzle_rtl(rows_grid: &Grid<&str>, operator_rows: &[char]) -> u64 {
    let (mult_ops, add_ops): (Vec<_>, Vec<_>) = operator_rows
        .iter()
        .enumerate()
        .partition(|(_, op)| **op == MULT_CHAR);

    let mult_total: u64 = mult_ops
        .iter()
        .map(|(x, _)| {
            rows_grid.iter_col(*x).map(|num_str|{
            })
        })
        .sum();
}

pub fn solve(input: &str) -> SolutionPair {
    let (rows_grid, operator_rows) = parse_input(input);
    let result = compute_puzzle(&rows_grid, &operator_rows);
    (Solution::from(result), Solution::from(0))
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
        let (rows_grid, operator_rows) = parse_input_rtl(TEST_INPUT);
        dbg!(&rows_grid);
        dbg!(&operator_rows);
        assert_eq!(false, true);
    }
}
