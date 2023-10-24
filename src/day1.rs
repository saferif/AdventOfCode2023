use crate::error::AoCError;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub(crate) fn solve(input: String) -> Result<String, AoCError> {
    let nums = input
        .split(" ")
        .map(|e| e.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;
    Ok(nums.into_iter().sum::<u64>().to_string())
}
