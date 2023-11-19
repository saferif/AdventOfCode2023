use crate::error::AoCError;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

fn points(turn: &str) -> Option<u64> {
    match turn {
        "A" | "X" => Some(1),
        "B" | "Y" => Some(2),
        "C" | "Z" => Some(3),
        _ => None,
    }
}

fn round(my: u64, other: u64) -> u64 {
    if my == other {
        3
    } else if (my == 2 && other == 1) || (my == 3 && other == 2) || (my == 1 && other == 3) {
        6
    } else {
        0
    }
}

fn choose(other: u64, result: &str) -> u64 {
    let win = other % 3 + 1;
    let loss = 6 - win - other;
    if result == "X" {
        loss
    } else if result == "Y" {
        other
    } else {
        win
    }
}

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let points = input
        .trim()
        .lines()
        .map(|line| {
            let (other, my) = line
                .split_once(" ")
                .ok_or(AoCError::from("invalid line format"))?;
            let other = points(other).ok_or(AoCError::from("invalid value of other"))?;
            let my = points(my).ok_or(AoCError::from("invalid value of my"))?;
            Ok(round(my, other) + my)
        })
        .collect::<Result<Vec<u64>, AoCError>>()?;
    Ok(points.into_iter().sum::<u64>().to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let points = input
        .trim()
        .lines()
        .map(|line| {
            let (other, result) = line
                .split_once(" ")
                .ok_or(AoCError::from("invalid line format"))?;
            let other = points(other).ok_or(AoCError::from("invalid value of other"))?;
            let my = choose(other, result);
            Ok(round(my, other) + my)
        })
        .collect::<Result<Vec<u64>, AoCError>>()?;
    Ok(points.into_iter().sum::<u64>().to_string())
}
