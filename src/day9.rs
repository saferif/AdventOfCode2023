use crate::error::AoCError;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::num::ParseIntError;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    solve(input, &|n, v| n + v, &|v| v.iter().next_back().copied())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    solve(input, &|n, v| v - n, &|v| v.iter().next().copied())
}

fn solve<F: Fn(i64, i64) -> i64, G: Fn(&[i64]) -> Option<i64>>(
    input: String,
    f: &F,
    getter: &G,
) -> Result<String, AoCError> {
    let predictions = input
        .trim()
        .lines()
        .map(|line| {
            let values = line
                .split(' ')
                .map(|v| v.parse::<i64>())
                .collect::<Result<Vec<i64>, ParseIntError>>()?;
            Ok(predict(values, f, getter)?)
        })
        .collect::<Result<Vec<i64>, AoCError>>()?;
    Ok(predictions.into_iter().sum::<i64>().to_string())
}

fn predict<F: Fn(i64, i64) -> i64, G: Fn(&[i64]) -> Option<i64>>(
    values: Vec<i64>,
    f: &F,
    getter: &G,
) -> Result<i64, AoCError> {
    if values.iter().all(|v| *v == 0) {
        Ok(0)
    } else {
        let diffs = values.windows(2).map(|v| v[1] - v[0]).collect::<Vec<i64>>();
        let v = getter(values.as_slice()).ok_or(AoCError::from("empty list"))?;
        let n = predict(diffs, f, getter)?;
        Ok(f(n, v))
    }
}
