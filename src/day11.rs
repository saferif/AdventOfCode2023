use crate::error::AoCError;
use alloc::collections::BTreeSet;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::{max, min};

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    solve(input, 2)
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    solve(input, 1000000)
}

fn solve(input: String, expansion: u64) -> Result<String, AoCError> {
    let map = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let stars = map
        .iter()
        .enumerate()
        .flat_map(|row| {
            row.1
                .iter()
                .enumerate()
                .filter(|c| *c.1 == '#')
                .map(move |c| (row.0, c.0))
        })
        .collect::<Vec<(usize, usize)>>();
    let star_rows = stars.iter().map(|c| c.0).collect::<BTreeSet<usize>>();
    let star_columns = stars.iter().map(|c| c.1).collect::<BTreeSet<usize>>();
    let s = stars
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            stars[(i + 1)..].iter().map(|y| {
                let rmm = min(x.0, y.0);
                let rmx = max(x.0, y.0);
                let cmm = min(x.1, y.1);
                let cmx = max(x.1, y.1);
                let er = (rmm..rmx).filter(|v| !star_rows.contains(v)).count();
                let ec = (cmm..cmx).filter(|v| !star_columns.contains(v)).count();
                let s = (rmx - rmm) + (cmx - cmm);
                s as u64 + (er + ec) as u64 * (expansion - 1)
            })
        })
        .sum::<u64>();
    Ok(s.to_string())
}
