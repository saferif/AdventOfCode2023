use crate::error::AoCError;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::Range;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    solve(
        input,
        [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ],
        9..10,
    )
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    solve(
        input,
        [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ],
        1..13,
    )
}

fn solve(input: String, cards: [char; 13], replacements: Range<usize>) -> Result<String, AoCError> {
    let mut hands = parse_hands(&input)?;
    hands.sort_by(|a, b| {
        let ord = make_strongest(a.0, &cards[replacements.clone()])
            .cmp(&make_strongest(b.0, &cards[replacements.clone()]));
        if ord != Ordering::Equal {
            return ord;
        }
        cmp_cards(a.0, b.0, cards)
    });
    let s = hands
        .iter()
        .enumerate()
        .map(|(p, v)| (p as u64 + 1) * v.1)
        .sum::<u64>();
    Ok(s.to_string())
}

fn parse_hands(input: &str) -> Result<Vec<(&str, u64)>, AoCError> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').ok_or(AoCError::from("not a pair"))?;
            let bid = bid.parse::<u64>()?;
            Ok((hand, bid))
        })
        .collect::<Result<Vec<(&str, u64)>, AoCError>>()
}

fn cmp_cards(a: &str, b: &str, cards: [char; 13]) -> Ordering {
    a.chars()
        .zip(b.chars())
        .map(|(a, b)| {
            let pa = cards
                .iter()
                .position(|m| a == *m)
                .map(|m| m + 1)
                .unwrap_or(0);
            let pb = cards
                .iter()
                .position(|m| b == *m)
                .map(|m| m + 1)
                .unwrap_or(0);
            pa.cmp(&pb)
        })
        .find(|p| *p != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}

fn hand_type(hand: &str) -> usize {
    let mut counts: BTreeMap<char, usize> = BTreeMap::new();
    hand.chars()
        .for_each(|c| *counts.entry(c).or_insert(0) += 1);
    let mut counts = counts.into_values().collect::<Vec<usize>>();
    counts.sort();
    match &counts[..] {
        &[5] => 7,
        &[1, 4] => 6,
        &[2, 3] => 5,
        &[1, 1, 3] => 4,
        &[1, 2, 2] => 3,
        &[1, 1, 1, 2] => 2,
        &[1, 1, 1, 1, 1] => 1,
        _ => 0,
    }
}

fn make_strongest(hand: &str, cards: &[char]) -> usize {
    cards
        .iter()
        .map(|card| hand_type(hand.replace('J', card.to_string().as_str()).as_str()))
        .max()
        .unwrap_or(0)
}
