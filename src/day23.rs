use crate::error::AoCError;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

const ALL_DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    solve(input, |c| match c {
        '^' => &ALL_DIRECTIONS[0..=0],
        '>' => &ALL_DIRECTIONS[1..=1],
        'v' => &ALL_DIRECTIONS[2..=2],
        '<' => &ALL_DIRECTIONS[3..=3],
        '.' => ALL_DIRECTIONS.as_slice(),
        _ => &[],
    })
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    solve(input, |c| match c {
        '#' => &[],
        _ => ALL_DIRECTIONS.as_slice(),
    })
}

fn solve(
    input: String,
    possible_directions: fn(char) -> &'static [(isize, isize)],
) -> Result<String, AoCError> {
    let map: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let start = map
        .get(0)
        .and_then(|row| {
            row.iter()
                .enumerate()
                .find_map(|(i, &c)| (c == '.').then_some((0, i)))
        })
        .ok_or(AoCError::from("no start"))?;
    let finish = map
        .get(map.len() - 1)
        .and_then(|row| {
            row.iter()
                .enumerate()
                .find_map(|(i, &c)| (c == '.').then_some((map.len() - 1, i)))
        })
        .ok_or(AoCError::from("no finish"))?;

    let mut nodes = nodes(&map);
    nodes.extend([start, finish]);
    let edges = edges(&map, &nodes, possible_directions);

    let mut visited = BTreeSet::new();
    let answer = dfs(&edges, start, finish, &mut visited).ok_or(AoCError::from("no path"))?;
    Ok(answer.to_string())
}

fn nodes(map: &Vec<Vec<char>>) -> BTreeSet<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(ri, row)| {
            row.iter().enumerate().filter_map(move |(ci, c)| {
                (*c != '#'
                    && ALL_DIRECTIONS
                        .iter()
                        .filter(|(dr, dc)| {
                            let new_row = ri.wrapping_add_signed(*dr);
                            let new_column = ci.wrapping_add_signed(*dc);
                            map.get(new_row)
                                .and_then(|row| row.get(new_column))
                                .filter(|c| **c != '#')
                                .is_some()
                        })
                        .count()
                        > 2)
                .then_some((ri, ci))
            })
        })
        .collect()
}

fn edges(
    map: &Vec<Vec<char>>,
    nodes: &BTreeSet<(usize, usize)>,
    directions: fn(char) -> &'static [(isize, isize)],
) -> BTreeMap<(usize, usize), BTreeMap<(usize, usize), u64>> {
    nodes
        .iter()
        .map(|node| {
            let mut visited = BTreeSet::new();
            let mut edges = BTreeMap::new();
            let mut stack = Vec::from([(*node, 0)]);
            while let Some((current @ (row, column), dist)) = stack.pop() {
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current);

                if dist > 0 && nodes.contains(&current) {
                    edges.insert(current, dist);
                    continue;
                }

                if let Some(c) = map.get(row).and_then(|row| row.get(column)) {
                    directions(*c).iter().for_each(|(dr, dc)| {
                        let new_row = row.wrapping_add_signed(*dr);
                        let new_column = column.wrapping_add_signed(*dc);
                        stack.push(((new_row, new_column), dist + 1));
                    });
                }
            }
            (*node, edges)
        })
        .collect()
}

fn dfs(
    edges: &BTreeMap<(usize, usize), BTreeMap<(usize, usize), u64>>,
    current: (usize, usize),
    finish: (usize, usize),
    visited: &mut BTreeSet<(usize, usize)>,
) -> Option<u64> {
    if visited.contains(&current) {
        return None;
    }

    if current == finish {
        return Some(0);
    }

    visited.insert(current);

    let mut lengths = Vec::new();
    edges
        .get(&current)
        .iter()
        .flat_map(|m| m.iter())
        .for_each(|(next_pos, next_dist)| {
            lengths.push(dfs(edges, *next_pos, finish, visited).map(|v| v + next_dist));
        });

    visited.remove(&current);

    lengths.into_iter().flatten().max()
}
