use crate::error::AoCError;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    solve(input, 0)
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    solve(input, 1)
}

fn solve(input: String, smudges: usize) -> Result<String, AoCError> {
    Ok(input
        .trim()
        .split("\n\n")
        .map(|pattern| {
            let rows = pattern.split('\n').collect::<Vec<_>>();
            (1..rows[0].len())
                .find(|i| {
                    rows.iter()
                        .map(|row| {
                            row[..*i]
                                .chars()
                                .rev()
                                .zip(row[*i..].chars())
                                .filter(|(x, y)| x != y)
                                .count()
                        })
                        .sum::<usize>()
                        == smudges
                })
                .or_else(|| {
                    (1..rows.len())
                        .find(|i| {
                            rows[..*i]
                                .iter()
                                .rev()
                                .zip(rows[*i..].iter())
                                .map(|(x, y)| {
                                    x.chars().zip(y.chars()).filter(|(u, v)| u != v).count()
                                })
                                .sum::<usize>()
                                == smudges
                        })
                        .map(|v| v * 100)
                })
                .unwrap_or(0)
        })
        .sum::<usize>()
        .to_string())
}
