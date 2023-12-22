use crate::error::AoCError;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::{max_by_key, min_by_key};
use core::ops::RangeInclusive;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let (answer, _) = solve(input)?;
    Ok(answer.to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let (_, answer) = solve(input)?;
    Ok(answer.to_string())
}

fn solve(input: String) -> Result<(u64, u64), AoCError> {
    let (bricks, _) = fall_bricks(parse_bricks(input)?);
    let (count, sum) = bricks
        .iter()
        .filter_map(|brick| {
            let (_, count) = fall_bricks(bricks.iter().filter(|&b| b != brick).copied());
            (count > 0).then_some(count)
        })
        .fold((0, 0), |(c, s), x| (c + 1, s + x));
    Ok(((bricks.len() - count) as u64, sum))
}

fn parse_bricks(input: String) -> Result<Vec<((u64, u64, u64), (u64, u64, u64))>, AoCError> {
    let mut bricks = input
        .trim()
        .lines()
        .map(|line| {
            let (p1, p2) = line
                .split_once('~')
                .ok_or(AoCError::from("invalid format"))?;

            let (x1, rest) = p1.split_once(',').ok_or(AoCError::from("invalid format"))?;
            let (y1, z1) = rest
                .split_once(',')
                .ok_or(AoCError::from("invalid format"))?;

            let (x2, rest) = p2.split_once(',').ok_or(AoCError::from("invalid format"))?;
            let (y2, z2) = rest
                .split_once(',')
                .ok_or(AoCError::from("invalid format"))?;

            let x1 = x1.parse::<u64>()?;
            let y1 = y1.parse::<u64>()?;
            let z1 = z1.parse::<u64>()?;
            let p1 = (x1, y1, z1);

            let x2 = x2.parse::<u64>()?;
            let y2 = y2.parse::<u64>()?;
            let z2 = z2.parse::<u64>()?;
            let p2 = (x2, y2, z2);

            Ok((
                min_by_key(p1, p2, point_to_key),
                max_by_key(p1, p2, point_to_key),
            ))
        })
        .collect::<Result<Vec<_>, AoCError>>()?;
    bricks.sort_by_key(brick_to_key);
    Ok(bricks)
}

fn fall_bricks<B: IntoIterator<Item = ((u64, u64, u64), (u64, u64, u64))>>(
    bricks: B,
) -> (Vec<((u64, u64, u64), (u64, u64, u64))>, u64) {
    let mut map = BTreeMap::new();
    let mut count = 0;
    let new_bricks = bricks
        .into_iter()
        .map(|brick @ (p1, p2)| {
            let max_height = cartesian_product_iter(p1.0..=p2.0, p1.1..=p2.1)
                .filter_map(|pr| map.get(&pr).copied())
                .max()
                .unwrap_or(0u64);
            let drop = p1.2 - max_height - 1;
            let new_brick = ((p1.0, p1.1, p1.2 - drop), (p2.0, p2.1, p2.2 - drop));
            cartesian_product_iter(p1.0..=p2.0, p1.1..=p2.1).for_each(|pr| {
                map.insert(pr, new_brick.1 .2);
            });
            count += (brick != new_brick) as u64;
            new_brick
        })
        .collect();
    (new_bricks, count)
}

fn point_to_key(p: &(u64, u64, u64)) -> (u64, u64, u64) {
    (p.2, p.0, p.1)
}

fn brick_to_key(b: &((u64, u64, u64), (u64, u64, u64))) -> ((u64, u64, u64), (u64, u64, u64)) {
    (point_to_key(&b.0), point_to_key(&b.1))
}

fn cartesian_product_iter(
    p1: RangeInclusive<u64>,
    p2: RangeInclusive<u64>,
) -> impl Iterator<Item = (u64, u64)> {
    p1.flat_map(move |x| p2.clone().map(move |y| (x, y)))
}
