use std::collections::LinkedList;

fn find_largest_pair_linear(line: impl Iterator<Item = u8>) -> u8 {
    let mut first_largest = 0;
    let mut second_largest = 0;

    for c in line {
        // first initialize the pair
        if first_largest == 0 {
            first_largest = c;
            continue;
        }

        if second_largest == 0 {
            second_largest = c;
            continue;
        }

        // then if second is larger than first, move it to first and take whatever number we're on
        // as second
        if second_largest > first_largest {
            first_largest = second_largest;
            second_largest = c;
            continue;
        } else if c > second_largest {
            // otherwise if current number is larger than second, move it to second
            second_largest = c;
        }
    }

    return first_largest * 10 + second_largest;
}

fn find_largest_twelve_linear(line: impl Iterator<Item = u8>) -> u64 {
    let mut outputs = LinkedList::from_iter([0; 12]);

    for (i, c) in line.enumerate() {
        // first initialize the 12 numbers, this is a bit inefficient but it happens 12 times per
        // row
        if i < 12 {
            *outputs.iter_mut().nth(i).unwrap() = c;
            continue;
        }

        // then every time we iterate, add the new number to the end
        outputs.push_back(c);

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

fn solve_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| find_largest_pair_linear(line.chars().map(|c| c.to_digit(10).unwrap() as u8)))
        .map(u64::from)
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| find_largest_twelve_linear(line.chars().map(|c| c.to_digit(10).unwrap() as u8)))
        .map(u64::from)
        .sum()
}

pub fn run(input: &str) {
    // let sum = solve_part1(input);
    let sum = solve_part2(input);
    dbg!(sum);
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
        assert_eq!(solve_part1(TEST_INPUT), 357);
    }

    #[test]
    fn test_largest_12_1() {
        let line: [u8; 15] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(find_largest_twelve_linear(line.into_iter()), 987654321111);
    }

    #[test]
    fn test_largest_12_2() {
        let line: [u8; 15] = [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        assert_eq!(find_largest_twelve_linear(line.into_iter()), 811111111119);
    }

    #[test]
    fn test_largest_12_3() {
        let line: [u8; 15] = [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        assert_eq!(find_largest_twelve_linear(line.into_iter()), 434234234278);
    }

    #[test]
    fn test_input_2() {
        assert_eq!(solve_part2(TEST_INPUT), 3121910778619);
    }
}
