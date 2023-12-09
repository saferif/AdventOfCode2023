use crate::error::AoCError;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::max;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let bag: BTreeMap<&str, u32> = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let games = input
        .trim()
        .lines()
        .map(|game| {
            let (id, game) = game
                .split_once(": ")
                .ok_or(AoCError::from("invalid format"))?;
            let id = id.trim_start_matches("Game ").parse::<u32>()?;
            let results = game
                .split("; ")
                .map(|rounds| {
                    let rounds = parse_rounds(rounds)?;
                    Ok(rounds
                        .into_iter()
                        .all(|round| bag.get(round.0).map(|cnt| *cnt >= round.1).unwrap_or(false)))
                })
                .collect::<Result<Vec<bool>, AoCError>>()?;
            Ok(if results.into_iter().all(|r| r) {
                id
            } else {
                0
            })
        })
        .collect::<Result<Vec<u32>, AoCError>>()?;
    Ok(games.into_iter().sum::<u32>().to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let games = input
        .trim()
        .lines()
        .map(|game| {
            let (_, game) = game
                .split_once(": ")
                .ok_or(AoCError::from("invalid format"))?;
            let mut bag: BTreeMap<&str, u32> =
                BTreeMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            game.split("; ")
                .map(|rounds| {
                    let rounds = parse_rounds(rounds)?;
                    rounds.into_iter().for_each(|round| {
                        bag.get_mut(round.0)
                            .into_iter()
                            .for_each(|v| *v = max(*v, round.1))
                    });
                    Ok(())
                })
                .collect::<Result<Vec<()>, AoCError>>()?;
            Ok(bag.into_values().fold(1, |a, v| a * v))
        })
        .collect::<Result<Vec<u32>, AoCError>>()?;
    Ok(games.into_iter().sum::<u32>().to_string())
}

fn parse_rounds(rounds: &str) -> Result<Vec<(&str, u32)>, AoCError> {
    rounds
        .split(", ")
        .map(|cubes| {
            let (cnt, color) = cubes
                .split_once(" ")
                .ok_or(AoCError::from("invalid format"))?;
            let cnt = cnt.parse::<u32>()?;
            Ok((color, cnt))
        })
        .collect::<Result<Vec<(&str, u32)>, AoCError>>()
}
