#![allow(dead_code)]

pub fn digits_to_number(digits: impl Iterator<Item = char>) -> u64 {
    digits.fold(0, |acc, ch| {
        acc * 10 + ch.to_digit(10).expect("Invalid digit") as u64
    })
}
