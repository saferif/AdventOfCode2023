use crate::error::AoCError;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::num::ParseIntError;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    Ok(input
        .trim()
        .split(',')
        .map(|v| hash(v) as u64)
        .sum::<u64>()
        .to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let mut buckets = Vec::<Vec<(&str, u64)>>::new();
    for _ in 0..256 {
        buckets.push(Vec::new())
    }
    input
        .trim()
        .split(',')
        .map(|command| {
            if let Some((label, length)) = command.split_once('=') {
                let key = hash(label);
                let focal_length = length.parse::<u64>()?;
                if let Some(pos) = buckets[key].iter().position(|e| e.0 == label) {
                    buckets[key][pos].1 = focal_length;
                } else {
                    buckets[key].push((label, focal_length));
                }
            } else {
                let label = command.trim_end_matches('-');
                let key = hash(label);
                if let Some(pos) = buckets[key].iter().position(|e| e.0 == label) {
                    buckets[key].remove(pos);
                }
            }
            Ok(())
        })
        .collect::<Result<Vec<()>, ParseIntError>>()?;
    let s = buckets
        .into_iter()
        .enumerate()
        .flat_map(|(bucket_idx, bucket)| {
            bucket.into_iter().enumerate().map(move |(slot_idx, lens)| {
                (1 + bucket_idx as u64) * (1 + slot_idx as u64) * lens.1
            })
        })
        .sum::<u64>();
    Ok(s.to_string())
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |h, c| (h + (c as usize)) * 17 % 256)
}
