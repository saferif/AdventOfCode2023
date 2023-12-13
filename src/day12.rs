use crate::error::AoCError;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::num::ParseIntError;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    solve(input, 1)
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    solve(input, 5)
}

fn solve(input: String, multiplier: usize) -> Result<String, AoCError> {
    let s = input
        .lines()
        .map(|line| {
            let (template, numbers) = line
                .split_once(' ')
                .ok_or(AoCError::from("invalid format"))?;
            let mut template = template.chars().collect::<Vec<char>>();
            template.push('?');
            template = template.repeat(multiplier);
            let n = template.len() - 1;
            template[n] = '.';
            let numbers = numbers
                .split(',')
                .map(|x| x.parse::<usize>())
                .collect::<Result<Vec<usize>, ParseIntError>>()?;
            let numbers = numbers.repeat(multiplier);
            let mut cache = BTreeMap::<(usize, usize, usize), u64>::new();
            Ok(find_maps(&mut template, 0, &numbers, 0, 0, &mut cache))
        })
        .collect::<Result<Vec<u64>, AoCError>>()?;
    Ok(s.iter().sum::<u64>().to_string())
}

fn find_maps(
    template: &mut Vec<char>,
    idx: usize,
    correct: &Vec<usize>,
    block: usize,
    cur: usize,
    cache: &mut BTreeMap<(usize, usize, usize), u64>,
) -> u64 {
    let key = (idx, block, cur);
    if let Some(res) = cache.get(&key) {
        return *res;
    }
    if idx == template.len() {
        return (block == correct.len()) as u64;
    }
    let res = ['.', '#']
        .iter()
        .map(|c| {
            if template[idx] == '?' || template[idx] == *c {
                if *c == '.' {
                    if cur == 0 {
                        find_maps(template, idx + 1, correct, block, 0, cache)
                    } else if correct.get(block) == Some(&cur) {
                        find_maps(template, idx + 1, correct, block + 1, 0, cache)
                    } else {
                        0
                    }
                } else {
                    find_maps(template, idx + 1, correct, block, cur + 1, cache)
                }
            } else {
                0
            }
        })
        .sum::<u64>();
    cache.insert(key, res);
    res
}
