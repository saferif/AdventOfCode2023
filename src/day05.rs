use crate::error::AoCError;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::num::ParseIntError;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let mut items = ("", Vec::<u64>::new());
    let mut mappings = BTreeMap::<&str, (&str, Vec<(u64, u64, u64)>)>::new();
    input
        .trim()
        .split("\n\n")
        .map(|block| {
            if block.starts_with("seeds: ") {
                let nums = block
                    .trim_start_matches("seeds: ")
                    .split(' ')
                    .map(|d| d.parse::<u64>())
                    .collect::<Result<Vec<u64>, ParseIntError>>()?;
                items = ("seed", nums);
            } else {
                let mut lines = block.lines();
                let map = lines.next().ok_or(AoCError::from("map name expected"))?;
                let map = map
                    .split(' ')
                    .next()
                    .ok_or(AoCError::from("map name expected"))?;
                let mut map = map.split('-');
                let from = map.next().ok_or(AoCError::from("map from name expected"))?;
                map.next();
                let dest = map.next().ok_or(AoCError::from("map dest name expected"))?;
                let triplets = lines
                    .map(|line| {
                        let nums = line.split(' ').map(|d| d.parse::<u64>()).collect::<Result<
                            Vec<u64>,
                            ParseIntError,
                        >>(
                        )?;
                        Ok((nums[0], nums[1], nums[2]))
                    })
                    .collect::<Result<Vec<(u64, u64, u64)>, AoCError>>()?;
                mappings.insert(from, (dest, triplets));
            }
            Ok(())
        })
        .collect::<Result<Vec<()>, AoCError>>()?;
    progress(&mut items, &mappings, "location")?;
    let min = items
        .1
        .into_iter()
        .min()
        .ok_or(AoCError::from("no minimum"))?;
    Ok(min.to_string())
}

fn progress<'a>(
    items: &mut (&'a str, Vec<u64>),
    mappings: &BTreeMap<&str, (&'a str, Vec<(u64, u64, u64)>)>,
    finish: &str,
) -> Result<(), AoCError> {
    while items.0 != finish {
        let mapping = mappings
            .get(items.0)
            .ok_or(AoCError::from("unknown mapping"))?;
        let new_items = items
            .1
            .iter()
            .map(|item| {
                let mapping = mapping
                    .1
                    .iter()
                    .find(|mapping| *item >= mapping.1 && *item < mapping.1 + mapping.2);
                if let Some(mapping) = mapping {
                    let delta = (mapping.0 as i64) - (mapping.1 as i64);
                    ((*item as i64) + delta) as u64
                } else {
                    *item
                }
            })
            .collect::<Vec<u64>>();
        *items = (mapping.0, new_items);
    }
    Ok(())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let mut items = ("", Vec::<(u64, u64)>::new());
    let mut mappings = BTreeMap::<&str, (&str, Vec<(u64, u64, u64)>)>::new();
    let mut mappings_back = BTreeMap::<&str, (&str, Vec<(u64, u64, u64)>)>::new();
    input
        .trim()
        .split("\n\n")
        .map(|block| {
            if block.starts_with("seeds: ") {
                let nums = block
                    .trim_start_matches("seeds: ")
                    .split(' ')
                    .map(|d| d.parse::<u64>())
                    .collect::<Result<Vec<u64>, ParseIntError>>()?;
                let mut ranges: Vec<(u64, u64)> = Vec::new();
                nums.chunks_exact(2)
                    .for_each(|chunk| ranges.push((chunk[0], chunk[1])));
                items = ("seed", ranges);
            } else {
                let mut lines = block.lines();
                let map = lines.next().ok_or(AoCError::from("map name expected"))?;
                let map = map
                    .split(' ')
                    .next()
                    .ok_or(AoCError::from("map name expected"))?;
                let mut map = map.split('-');
                let from = map.next().ok_or(AoCError::from("map from name expected"))?;
                map.next();
                let dest = map.next().ok_or(AoCError::from("map dest name expected"))?;
                let triplets = lines
                    .map(|line| {
                        let nums = line.split(' ').map(|d| d.parse::<u64>()).collect::<Result<
                            Vec<u64>,
                            ParseIntError,
                        >>(
                        )?;
                        Ok((nums[0], nums[1], nums[2]))
                    })
                    .collect::<Result<Vec<(u64, u64, u64)>, AoCError>>()?;
                let triplets_back = triplets
                    .iter()
                    .map(|t| (t.1, t.0, t.2))
                    .collect::<Vec<(u64, u64, u64)>>();
                mappings.insert(from, (dest, triplets));
                mappings_back.insert(dest, (from, triplets_back));
            }
            Ok(())
        })
        .collect::<Result<Vec<()>, AoCError>>()?;

    let mut minimum = u64::MAX;
    mappings
        .iter()
        .map(|mapping| {
            mapping
                .1
                 .1
                .iter()
                .map(|range| {
                    let mut point = (mapping.1 .0, vec![range.0]);
                    progress(&mut point, &mappings, "location")?;
                    let cur = point.1[0];

                    let mut point = (mapping.1 .0, vec![range.0]);
                    progress(&mut point, &mappings_back, "seed")?;
                    let back = point.1[0];
                    if items
                        .1
                        .iter()
                        .any(|range| back >= range.0 && back < range.0 + range.1)
                        && cur < minimum
                    {
                        minimum = cur;
                    }
                    Ok(())
                })
                .collect::<Result<Vec<()>, AoCError>>()?;
            Ok(())
        })
        .collect::<Result<Vec<()>, AoCError>>()?;
    Ok(minimum.to_string())
}
