use crate::error::AoCError;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    solve(input, |n| **n == "AAA")
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    solve(input, |n| n.ends_with('A'))
}

fn solve<F: Fn(&&&str) -> bool>(input: String, starting: F) -> Result<String, AoCError> {
    let (steps, nodes) = input
        .trim()
        .split_once("\n\n")
        .ok_or(AoCError::from("invalid format"))?;
    let nodes = nodes
        .lines()
        .map(|line| {
            let (from, to) = line
                .split_once(" = ")
                .ok_or(AoCError::from("invalid format"))?;
            let (left, right) = to
                .trim_matches(['(', ')'].as_slice())
                .split_once(", ")
                .ok_or(AoCError::from("invalid format"))?;
            Ok((from, (left, right)))
        })
        .collect::<Result<BTreeMap<&str, (&str, &str)>, AoCError>>()?;
    let steps = steps.as_bytes();

    let mut finish_times = BTreeMap::<&str, usize>::new();
    let mut cur_nodes = nodes.keys().filter(starting).collect::<Vec<&&str>>();
    let mut cur_step = 0;
    while finish_times.len() != cur_nodes.len() {
        cur_nodes.iter().filter(|n| n.ends_with('Z')).for_each(|n| {
            finish_times.entry(n).or_insert(cur_step);
        });
        cur_nodes = cur_nodes
            .iter()
            .map(|cur_node| {
                let step = nodes.get(*cur_node).ok_or(AoCError::from("no step"))?;
                if steps[cur_step % steps.len()] == ('L' as u8) {
                    Ok(&step.0)
                } else {
                    Ok(&step.1)
                }
            })
            .collect::<Result<Vec<&&str>, AoCError>>()?;
        cur_step += 1;
    }
    Ok(finish_times
        .into_values()
        .map(|v| v as u64)
        .fold(1, lcm)
        .to_string())
}

fn gcd(x: u64, y: u64) -> u64 {
    let z = x | y;
    if x == 0 || y == 0 {
        return z;
    }
    let e = z.trailing_zeros();
    let mut x = x >> x.trailing_zeros();
    let mut y = y >> y.trailing_zeros();
    while x != y {
        if x < y {
            core::mem::swap(&mut x, &mut y);
        }
        x -= y;
        x >>= x.trailing_zeros();
    }
    x << e
}

fn lcm(x: u64, y: u64) -> u64 {
    x / gcd(x, y) * y
}
