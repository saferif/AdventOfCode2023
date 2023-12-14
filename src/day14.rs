use crate::error::AoCError;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let input = input.trim();
    let height = input.lines().count();
    Ok(score(tilt(parse_stones(input)), height).to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let input = input.trim();
    let height = input.lines().count();
    let mut stones = parse_stones(input);
    let target = 1000000000;
    let mut idx = 0;
    let mut cache = BTreeMap::<BTreeSet<(usize, usize, bool)>, usize>::new();
    stones = loop {
        for _ in 0..4 {
            stones = tilt(stones);
            stones = rotate(stones, height);
        }
        idx += 1;
        if let Some(t) = cache.get(&stones) {
            let n = idx - t;
            let x = (target - t) % n + t;
            let res = cache
                .iter()
                .find_map(|(key, value)| (*value == x).then_some(key))
                .ok_or(AoCError::from("not found in cache"))?;
            break res.clone();
        }
        cache.insert(stones.clone(), idx);
    };
    Ok(score(stones, height).to_string())
}

fn parse_stones(input: &str) -> BTreeSet<(usize, usize, bool)> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(ri, row)| {
            row.chars().enumerate().filter_map(move |(ci, c)| match c {
                '#' => Some((ci, ri, false)),
                'O' => Some((ci, ri, true)),
                _ => None,
            })
        })
        .collect()
}

fn tilt(stones: BTreeSet<(usize, usize, bool)>) -> BTreeSet<(usize, usize, bool)> {
    let mut limit = (0usize, 0usize);
    stones
        .into_iter()
        .map(|(ci, ri, moving)| {
            if ci != limit.0 {
                limit = (ci, 0);
            }
            if moving {
                let new = (ci, limit.1, true);
                limit = (ci, limit.1 + 1);
                new
            } else {
                limit = (ci, ri + 1);
                (ci, ri, false)
            }
        })
        .collect()
}

fn score(stones: BTreeSet<(usize, usize, bool)>, height: usize) -> u64 {
    stones
        .into_iter()
        .filter_map(|(_, ri, moving)| moving.then_some((height - ri) as u64))
        .sum()
}

fn rotate(stones: BTreeSet<(usize, usize, bool)>, height: usize) -> BTreeSet<(usize, usize, bool)> {
    stones
        .into_iter()
        .map(|(ci, ri, moving)| (height - ri - 1, ci, moving))
        .collect()
}
