use crate::error::AoCError;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let mut sum = 0;
    let map = parse_input(input);
    (0..map.len()).for_each(|row| {
        let mut adj = false;
        let mut num = 0;
        (0..map[row].len()).for_each(|col| {
            if let Some(d) = map[row][col].to_digit(10) {
                num = num * 10 + d;
                adj |= (-1i32..=1).any(|dr| {
                    (-1i32..=1).any(|dc| {
                        let nr = (row as i32) + dr;
                        let nc = (col as i32) + dc;
                        if nr >= 0 && nr < map.len() as i32 && nc >= 0 && nc < map[row].len() as i32
                        {
                            let nr = nr as usize;
                            let nc = nc as usize;
                            let c = map[nr][nc];
                            !(c.is_numeric() || c == '.')
                        } else {
                            false
                        }
                    })
                });
            } else {
                if adj {
                    sum += num;
                }
                adj = false;
                num = 0;
            }
        });
    });
    Ok(sum.to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let mut gears: BTreeMap<(usize, usize), (u32, usize)> = BTreeMap::new();
    let map = parse_input(input);
    (0..map.len()).for_each(|row| {
        let mut adj: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut num = 0;
        (0..map[row].len()).for_each(|col| {
            if let Some(d) = map[row][col].to_digit(10) {
                num = num * 10 + d;
                (-1i32..=1).for_each(|dr| {
                    (-1i32..=1).for_each(|dc| {
                        let nr = (row as i32) + dr;
                        let nc = (col as i32) + dc;
                        if nr >= 0 && nr < map.len() as i32 && nc >= 0 && nc < map[row].len() as i32
                        {
                            let nr = nr as usize;
                            let nc = nc as usize;
                            if map[nr][nc] == '*' {
                                adj.insert((nr, nc));
                            }
                        }
                    })
                });
            } else {
                adj.iter().for_each(|gear| {
                    gears
                        .entry(*gear)
                        .and_modify(|v| *v = (v.0 * num, v.1 + 1))
                        .or_insert((num, 1));
                });
                adj.clear();
                num = 0;
            }
        });
    });
    let sum = gears
        .values()
        .filter_map(|gear| if gear.1 == 2 { Some(gear.0) } else { None })
        .sum::<u32>();
    Ok(sum.to_string())
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut line = line.chars().collect::<Vec<char>>();
            line.push('.');
            line
        })
        .collect()
}
