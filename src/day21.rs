use crate::error::AoCError;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let map = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(ri, row)| {
            row.iter()
                .enumerate()
                .find_map(|(ci, &col)| (col == 'S').then_some((ri, ci)))
        })
        .ok_or(AoCError::from("no start"))?;
    let visited = bfs(&map, start);
    Ok(visited
        .into_values()
        .filter(|&v| v <= 64 && v % 2 == 0)
        .count()
        .to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let map = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(ri, row)| {
            row.iter()
                .enumerate()
                .find_map(|(ci, &col)| (col == 'S').then_some((ri, ci)))
        })
        .ok_or(AoCError::from("no start"))?;
    let visited = bfs(&map, start);
    let even_corners = visited
        .values()
        .filter(|&&v| v % 2 == 0 && v > map.len() as u64 / 2)
        .count() as u64;
    let odd_corners = visited
        .values()
        .filter(|&&v| v % 2 == 1 && v > map.len() as u64 / 2)
        .count() as u64;
    let even_full = visited.values().filter(|&&v| v % 2 == 0).count() as u64;
    let odd_full = visited.values().filter(|&&v| v % 2 == 1).count() as u64;
    let n = ((26501365 - map.len() / 2) / map.len()) as u64;
    Ok(
        (((n + 1) * (n + 1)) * odd_full + n * n * even_full - (n + 1) * odd_corners
            + n * even_corners)
            .to_string(),
    )
}

fn bfs(map: &Vec<Vec<char>>, start: (usize, usize)) -> BTreeMap<(usize, usize), u64> {
    let mut queue = VecDeque::from([(start, 0u64)]);
    let mut visited = BTreeMap::new();
    while let Some((coord @ (row, column), steps)) = queue.pop_front() {
        if visited.contains_key(&coord) {
            continue;
        }
        visited.insert(coord, steps);
        [(0, 1), (0, -1), (-1, 0), (1, 0)]
            .into_iter()
            .for_each(|(dr, dc)| {
                let new_row = row.wrapping_add_signed(dr);
                let new_column = column.wrapping_add_signed(dc);
                if let Some(&tile) = map.get(new_row).and_then(|row| row.get(new_column)) {
                    if tile != '#' {
                        queue.push_back(((new_row, new_column), steps + 1));
                    }
                }
            });
    }
    visited
}
