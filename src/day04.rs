use crate::error::AoCError;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::num::ParseIntError;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let cnts = input
        .trim()
        .lines()
        .map(|line| {
            let (_, cards) = line.split_once(":").ok_or(AoCError::from("no cards"))?;
            let (winning, have) = cards.split_once("|").ok_or(AoCError::from("no winning"))?;
            let winning = winning
                .trim()
                .split(" ")
                .filter(|card| !card.is_empty())
                .map(|card| card.parse::<u32>())
                .collect::<Result<BTreeSet<u32>, ParseIntError>>()?;
            let have = have
                .trim()
                .split(" ")
                .filter(|card| !card.is_empty())
                .map(|card| card.parse::<u32>())
                .collect::<Result<Vec<u32>, ParseIntError>>()?;
            Ok(have
                .into_iter()
                .filter(|card| winning.contains(card))
                .count())
        })
        .collect::<Result<Vec<usize>, AoCError>>()?;
    Ok(cnts
        .into_iter()
        .filter(|cnt| *cnt > 0)
        .map(|cnt| 1 << (cnt - 1))
        .sum::<usize>()
        .to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
    input
        .trim()
        .lines()
        .map(|line| {
            let (id, cards) = line.split_once(":").ok_or(AoCError::from("no cards"))?;
            let id = id
                .split(" ")
                .last()
                .ok_or(AoCError::from("invalid card ID"))?;
            let id = id.parse::<usize>()?;
            let card_count = *counts.entry(id).or_insert(1);
            let (winning, have) = cards.split_once("|").ok_or(AoCError::from("no winning"))?;
            let winning = winning
                .trim()
                .split(" ")
                .filter(|card| !card.is_empty())
                .map(|card| card.parse::<u32>())
                .collect::<Result<BTreeSet<u32>, ParseIntError>>()?;
            let have = have
                .trim()
                .split(" ")
                .filter(|card| !card.is_empty())
                .map(|card| card.parse::<u32>())
                .collect::<Result<Vec<u32>, ParseIntError>>()?;
            let winning_count = have
                .into_iter()
                .filter(|card| winning.contains(card))
                .count();
            (0..winning_count).for_each(|i| {
                counts
                    .entry(id + i + 1)
                    .and_modify(|v| *v += card_count)
                    .or_insert(1 + card_count);
            });
            Ok(())
        })
        .collect::<Result<Vec<()>, AoCError>>()?;
    Ok(counts.values().sum::<usize>().to_string())
}
