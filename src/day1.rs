use crate::error::AoCError;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

const DIGITS: &[&'static str] = &[
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn solve(input: String, digits: &[&str]) -> Result<String, AoCError> {
    let calibrations = input
        .trim()
        .lines()
        .map(|calibration| {
            let mut it = (0..calibration.len()).filter_map(|start| {
                digits
                    .iter()
                    .position(|digit| calibration[start..].starts_with(digit))
                    .map(|digit| digit % 9 + 1)
            });
            let first = it.next().ok_or(AoCError::from("no digits"))?;
            let last = it.next_back().unwrap_or(first);
            Ok(first * 10 + last)
        })
        .collect::<Result<Vec<usize>, AoCError>>()?;
    Ok(calibrations.into_iter().sum::<usize>().to_string())
}

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    solve(input, &DIGITS[..9])
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    solve(input, DIGITS)
}
