use alloc::collections::BTreeSet;
use crate::day10::Direction::{East, North, South, West};
use crate::error::AoCError;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let map = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (start_row, start_column) = map
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            if let Some(col) = line.iter().position(|c| *c == 'S') {
                Some((row, col))
            } else {
                None
            }
        })
        .ok_or(AoCError::from("no start"))?;
    let (_, longest) = ['|', '-', 'L', 'J', '7', 'F']
        .into_iter()
        .flat_map(|start_tile| {
            let map = &map;
            [North, South, East, West]
                .into_iter()
                .filter_map(move |start_direction| {
                    run_map(map, start_tile, start_direction, start_row, start_column)
                })
        })
        .max()
        .ok_or(AoCError::from("no loop"))?;
    Ok((longest.len() >> 1).to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let mut map = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (start_row, start_column) = map
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            if let Some(col) = line.iter().position(|c| *c == 'S') {
                Some((row, col))
            } else {
                None
            }
        })
        .ok_or(AoCError::from("no start"))?;
    let (start_tile, longest) = [
        ('|', [North, South]),
        ('-', [East, West]),
        ('L', [South, West]),
        ('J', [South, East]),
        ('7', [North, East]),
        ('F', [North, West])]
        .into_iter()
        .filter_map(|(start_tile, directions)| {
            run_map(&map, start_tile, directions[0], start_row, start_column).and_then(|_|{
                run_map(&map, start_tile, directions[1], start_row, start_column)
            })
        })
        .max_by_key(|v| v.1.len())
        .ok_or(AoCError::from("no loop"))?;
    map[start_row][start_column] = start_tile;

    let inside = longest
        .iter()
        .find_map(|v| {
            if map[v.0][v.1] == 'F' {
                Some((v.0 * 2 + 1, v.1 * 2 + 1))
            } else if map[v.0][v.1] == '7' {
                Some((v.0 * 2 + 1, v.1 * 2 - 1))
            } else if map[v.0][v.1] == 'J' {
                Some((v.0 * 2 - 1, v.1 * 2 - 1))
            } else if map[v.0][v.1] == 'L' {
                Some((v.0 * 2 - 1, v.1 * 2 + 1))
            } else {
                None
            }
        })
        .ok_or(AoCError::from("no corner"))?;
    let mut visited = BTreeSet::<(usize, usize)>::new();
    fill(&map, inside.0, inside.1, &mut visited);
    let visited = visited
        .iter()
        .flat_map(|v| {
            let row = v.0;
            let column = v.1;
            let tl = (row / 2, column / 2);
            let tr = (row / 2, column / 2 + 1);
            let bl = (row / 2 + 1, column / 2);
            let br = (row / 2 + 1, column / 2 + 1);
            [tl, tr, bl, br].into_iter()
        })
        .collect::<BTreeSet<(usize, usize)>>();
    Ok((visited.len() - longest.len()).to_string())
}

fn run_map(
    map: &Vec<Vec<char>>,
    start_tile: char,
    start_direction: Direction,
    start_row: usize,
    start_column: usize,
) -> Option<(char, BTreeSet<(usize, usize)>)> {
    let mut row = start_row;
    let mut column = start_column;
    let mut direction = start_direction;
    let mut steps = BTreeSet::<(usize, usize)>::new();
    loop {
        steps.insert((row, column));
        let tile = *map.get(row).and_then(|r| r.get(column))?;
        let tile = (tile != 'S').then_some(tile).unwrap_or(start_tile);
        match tile {
            '|' => match direction {
                North => {
                    row = row.wrapping_sub(1);
                }
                South => {
                    row = row.wrapping_add(1);
                }
                West => {
                    break None;
                }
                East => {
                    break None;
                }
            },
            '-' => match direction {
                North => {
                    break None;
                }
                South => {
                    break None;
                }
                West => {
                    column = column.wrapping_sub(1);
                }
                East => {
                    column = column.wrapping_add(1);
                }
            },
            'L' => match direction {
                North => {
                    break None;
                }
                South => {
                    column = column.wrapping_add(1);
                    direction = East;
                }
                West => {
                    row = row.wrapping_sub(1);
                    direction = North;
                }
                East => {
                    break None;
                }
            },
            'J' => match direction {
                North => {
                    break None;
                }
                South => {
                    column = column.wrapping_sub(1);
                    direction = West;
                }
                West => {
                    break None;
                }
                East => {
                    row = row.wrapping_sub(1);
                    direction = North;
                }
            },
            '7' => match direction {
                North => {
                    column = column.wrapping_sub(1);
                    direction = West;
                }
                South => {
                    break None;
                }
                West => {
                    break None;
                }
                East => {
                    row = row.wrapping_add(1);
                    direction = South;
                }
            },
            'F' => match direction {
                North => {
                    column = column.wrapping_add(1);
                    direction = East;
                }
                South => {
                    break None;
                }
                West => {
                    row = row.wrapping_add(1);
                    direction = South;
                }
                East => {
                    break None;
                }
            },
            _ => {
                break None;
            }
        }
        if (row, column) == (start_row, start_column) {
            break Some((start_tile, steps));
        }
    }
}

fn fill(map: &Vec<Vec<char>>, row: usize, column: usize, visited: &mut BTreeSet<(usize, usize)>) {
    if visited.contains(&(row, column)) {
        return;
    }
    visited.insert((row, column));
    let tl = (row / 2, column / 2);
    let tr = (row / 2, column / 2 + 1);
    let bl = (row / 2 + 1, column / 2);
    let br = (row / 2 + 1, column / 2 + 1);
    let width = map[row / 2].len();

    // north
    let t1 = map[tl.0][tl.1];
    let t2 = map[tr.0][tr.1];
    if (t1 != 'L' && t1 != '-' && t1 != 'F') || (t2 != 'J' && t2 != '-' && t2 != '7') {
        let nr = (row as i64) - 2;
        if nr > 0 {
            fill(map, nr as usize, column, visited);
        }
    }

    // south
    let t1 = map[bl.0][bl.1];
    let t2 = map[br.0][br.1];
    if (t1 != 'L' && t1 != '-' && t1 != 'F') || (t2 != 'J' && t2 != '-' && t2 != '7') {
        let nr = row + 2;
        if nr < (map.len() - 1) * 2 {
            fill(map, nr, column, visited);
        }
    }

    // west
    let t1 = map[tl.0][tl.1];
    let t2 = map[bl.0][bl.1];
    if (t1 != '7' && t1 != '|' && t1 != 'F') || (t2 != 'J' && t2 != '|' && t2 != 'L') {
        let nc = (column as i64) - 2;
        if nc > 0 {
            fill(map, row, nc as usize, visited);
        }
    }

    // east
    let t1 = map[tr.0][tr.1];
    let t2 = map[br.0][br.1];
    if (t1 != '7' && t1 != '|' && t1 != 'F') || (t2 != 'J' && t2 != '|' && t2 != 'L') {
        let nc = column + 2;
        if nc < (width - 1) * 2 {
            fill(map, row, nc, visited);
        }
    }
}