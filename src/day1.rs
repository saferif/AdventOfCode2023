use crate::error::AoCError;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::num::ParseIntError;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let elfs = input
        .trim()
        .split("\n\n")
        .map(|elf| {
            let weights = elf
                .split("\n")
                .map(|weight| weight.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()?;
            Ok(weights.into_iter().sum())
        })
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    Ok(elfs.into_iter().max().unwrap_or(0).to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let mut elfs = input
        .trim()
        .split("\n\n")
        .map(|elf| {
            let weights = elf
                .split("\n")
                .map(|weight| weight.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()?;
            Ok(weights.into_iter().sum())
        })
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    elfs.sort();
    elfs.reverse();
    Ok(elfs[..3].into_iter().sum::<u64>().to_string())
}
