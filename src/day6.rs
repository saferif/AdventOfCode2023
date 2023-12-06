use crate::error::AoCError;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let mut lines = input.trim().lines();
    let times = lines.next().ok_or(AoCError::from("no times line"))?;
    let distances = lines.next().ok_or(AoCError::from("no distances line"))?;
    let times = times
        .split(' ')
        .filter_map(|d| d.parse::<u64>().ok())
        .collect::<Vec<u64>>();
    let distances = distances
        .split(' ')
        .filter_map(|d| d.parse::<u64>().ok())
        .collect::<Vec<u64>>();
    let ans = times
        .into_iter()
        .zip(distances)
        .map(|pair| solve(pair.0, pair.1))
        .product::<u64>();
    Ok(ans.to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let input = input.trim().replace(' ', "");
    let mut lines = input.lines();
    let times = lines.next().ok_or(AoCError::from("no time line"))?;
    let distances = lines.next().ok_or(AoCError::from("no distances line"))?;
    let time = times.trim_start_matches("Time:").parse::<u64>()?;
    let distance = distances.trim_start_matches("Distance:").parse::<u64>()?;
    Ok(solve(time, distance).to_string())
}

fn solve(time: u64, distance: u64) -> u64 {
    let tip = binary_search(0, time, |t| {
        let x = calc_distance(t, time);
        let y = calc_distance(t + 1, time);
        x > y
    });
    let left = binary_search(0, tip, |t| calc_distance(t, time) > distance);
    let right = binary_search(tip, time, |t| calc_distance(t, time) <= distance);
    right - left
}

fn calc_distance(button: u64, time: u64) -> u64 {
    button * (time - button)
}

fn binary_search<F: Fn(u64) -> bool>(n: u64, m: u64, f: F) -> u64 {
    let (mut i, mut j) = (n, m);
    while i < j {
        let h = (i + j) / 2;
        if !f(h) {
            i = h + 1
        } else {
            j = h
        }
    }
    i
}
