use std::ops::RangeInclusive;

/// get divisors of a number (with square root optimization)
fn divisors(n: u32) -> Vec<u32> {
    let mut res = Vec::new();
    let sqrt_n = (n as f64).sqrt() as u32;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            res.push(i);
            let pair = n / i;
            if pair != i {
                res.push(pair);
            }
        }
    }

    res.sort_unstable();
    res
}

/// Check if id is invalid
/// i.e. ID is constructed by a sequence of repeated digits,e.g. 1212 or 234234
fn is_id_invalid(id: u64) -> bool {
    // if the number of digits is odd, it's always valid
    // let digit_count = id.ilog10() + 1;
    let digit_count = (id as f64).log10().floor() as u32 + 1;
    if digit_count % 2 == 1 {
        return false;
    }

    // otherwise, we can safely compare the front and back digits
    // based on the number of digits

    // split number is the number divided by 10^n, where n is the number of digits divided by 2
    let divisor = 10u64.pow(digit_count / 2);
    let front_digits = id / divisor;
    let back_digits = id % divisor;

    // found invalid ids
    if front_digits == back_digits {
        return true;
    }

    return false;
}

/// Check if id is invalid strictly
/// i.e. ID is constructed by a sequence of repeated digits,e.g. 1212 or 234234, where the repeated
/// digits can happen N times (2+)
fn is_id_invalid_strict(id: u64) -> bool {
    let digit_count = (id as f64).log10().floor() as u32 + 1;

    // Possible divisors are divisors of the digit count (including 1)
    let divisor_lengths = divisors(digit_count);

    // for each divisor, check if the sequence of that length is repeated
    'outer: for divisor_len in divisor_lengths {
        // don't consider the divisor itself as then sequence wouldn't be repeated
        if divisor_len == digit_count {
            continue;
        }

        // 10^n is the divisor
        let divisor = 10u64.pow(divisor_len);

        // mod gives us the potential repeating sequence, e.g. 121212 mod 100 (divisor len 2) = 12
        let sequence = id % divisor;

        // keep track of a current number to keep dividing N times by the divisor to check if its
        // still repeating the pattern
        let mut num = id;

        // now, the front digits will be repeated N-1 times again where N is digit_count / divisor_len
        for _ in 0..(digit_count / divisor_len - 1) {
            num = num / divisor;

            let new_sequence = num % divisor;

            if new_sequence != sequence {
                // if the new seq are not the same then sequence is not repeating,
                // thus we can continue with the next divisor
                continue 'outer;
            }
        }

        // if we haven't broken the inner loop, then the back sequence is repeating
        return true;
    }

    return false;
}

/// Finds all invalid IDs in a range of IDs
/// i.e. items where the ID is constructed by a sequence of repeated digits,e.g. 1212 or 234234
fn get_invalid_ids(range: RangeInclusive<u64>) -> impl Iterator<Item = u64> {
    range.filter(|id| is_id_invalid(*id))
}

/// Finds all invalid IDs in a range of IDs using strict rules
/// i.e. items where the ID is constructed by a sequence of repeated digits,e.g. 1212 or 234234 of
/// length 2+
fn get_invalid_ids_strict(range: RangeInclusive<u64>) -> impl Iterator<Item = u64> {
    range.filter(|id| is_id_invalid_strict(*id))
}

fn get_ranges(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    return input.trim().split(',').filter_map(|ids| {
        let mut parts = ids.split('-').map(|id| id.parse::<u64>());
        let first = parts.next().expect("Missing first ID");
        let last = parts.next().expect("Missing last ID");

        if let Err(err) = first {
            println!("Invalid first part of ID: {ids}, err: {err}");
            return None;
        }

        if let Err(err) = last {
            println!("Invalid last part of ID: {ids}, err: {err}");
            return None;
        }

        return Some(first.unwrap()..=last.unwrap());
    });
}

pub fn run(input: &str) {
    // PART 1:
    // let sum_invalid = get_ranges(input).flat_map(|range| get_invalid_ids(range)).count();
    // PART 2:
    let sum_invalid: u64 = get_ranges(input)
        .flat_map(|range| get_invalid_ids_strict(range))
        .sum();
    dbg!(sum_invalid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_id_invalid() {
        assert_eq!(is_id_invalid(12), false);
        assert_eq!(is_id_invalid(55), true);
        assert_eq!(is_id_invalid(6464), true);
        assert_eq!(is_id_invalid(123123), true);
    }

    #[test]
    fn test_get_invalid_ids() {
        assert_eq!(get_invalid_ids(11..=22).collect::<Vec<_>>(), vec![11, 22]);
        assert_eq!(get_invalid_ids(95..=115).collect::<Vec<_>>(), vec![99]);
    }

    #[test]
    fn test_input_1() {
        let input = "\
    11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
    824824821-824824827,2121212118-2121212124";
        let count_invalid = get_ranges(input)
            .flat_map(|range| get_invalid_ids_strict(range))
            .count();
        assert_eq!(count_invalid, 1227775554);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(12), [1, 2, 3, 4, 6, 12]);
        assert_eq!(divisors(13), [1, 13]);
    }

    #[test]
    fn test_is_id_invalid_strict() {
        assert_eq!(is_id_invalid_strict(69), false);
        assert_eq!(is_id_invalid_strict(5), false);
        assert_eq!(is_id_invalid_strict(100), false);
        assert_eq!(is_id_invalid_strict(99), true);
        assert_eq!(is_id_invalid_strict(111), true);
        assert_eq!(is_id_invalid_strict(12341234), true);
        assert_eq!(is_id_invalid_strict(1111111), true);
    }

    #[test]
    fn test_get_invalid_ids_strict() {
        assert_eq!(
            get_invalid_ids_strict(11..=22).collect::<Vec<_>>(),
            vec![11, 22]
        );
        assert_eq!(
            get_invalid_ids_strict(95..=115).collect::<Vec<_>>(),
            vec![99, 111]
        );
        assert_eq!(
            get_invalid_ids_strict(2121212118..=2121212124).collect::<Vec<_>>(),
            vec![2121212121]
        );
    }
}
