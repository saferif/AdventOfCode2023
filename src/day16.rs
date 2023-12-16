use crate::error::AoCError;
use alloc::collections::{BTreeSet, VecDeque};
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}

type Beam = (usize, usize, Direction);

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let map = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    solve(&map, vec![(0, 0, Direction::Right)])
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let map = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let starts = (0..map.len())
        .flat_map(|row| {
            [
                (row, 0, Direction::Right),
                (row, map[row].len() - 1, Direction::Left),
            ]
            .into_iter()
        })
        .chain((0..map[0].len()).flat_map(|column| {
            [
                (0, column, Direction::Down),
                (map.len() - 1, column, Direction::Up),
            ]
            .into_iter()
        }))
        .collect::<Vec<Beam>>();
    solve(&map, starts)
}

fn solve(map: &Vec<Vec<char>>, starts: Vec<Beam>) -> Result<String, AoCError> {
    Ok(starts
        .into_iter()
        .map(|start| {
            Ok(bfs(&map, start)?
                .into_iter()
                .map(|(row, column, _)| (row, column))
                .collect::<BTreeSet<(usize, usize)>>()
                .len())
        })
        .collect::<Result<Vec<usize>, AoCError>>()?
        .into_iter()
        .max()
        .ok_or(AoCError::from("no path found"))?
        .to_string())
}

fn bfs(map: &Vec<Vec<char>>, start: Beam) -> Result<BTreeSet<Beam>, AoCError> {
    let mut queue = VecDeque::<Beam>::from([start]);
    let mut visited = BTreeSet::<Beam>::new();
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if visited.contains(&current) {
            continue;
        }
        if let Some(c) = map.get(current.0).and_then(|row| row.get(current.1)) {
            visited.insert(current);
            match c {
                '.' => {
                    let delta = current.2.delta();
                    let new_row = current.0.wrapping_add_signed(delta.0);
                    let new_column = current.1.wrapping_add_signed(delta.1);
                    queue.push_back((new_row, new_column, current.2));
                }
                '\\' => {
                    let new_direction = match current.2 {
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                    };
                    let delta = new_direction.delta();
                    let new_row = current.0.wrapping_add_signed(delta.0);
                    let new_column = current.1.wrapping_add_signed(delta.1);
                    queue.push_back((new_row, new_column, new_direction));
                }
                '/' => {
                    let new_direction = match current.2 {
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                    };
                    let delta = new_direction.delta();
                    let new_row = current.0.wrapping_add_signed(delta.0);
                    let new_column = current.1.wrapping_add_signed(delta.1);
                    queue.push_back((new_row, new_column, new_direction));
                }
                '-' => match current.2 {
                    Direction::Left | Direction::Right => {
                        let delta = current.2.delta();
                        let new_row = current.0.wrapping_add_signed(delta.0);
                        let new_column = current.1.wrapping_add_signed(delta.1);
                        queue.push_back((new_row, new_column, current.2));
                    }
                    Direction::Up | Direction::Down => {
                        queue.push_back((
                            current.0,
                            current.1.wrapping_add_signed(-1),
                            Direction::Left,
                        ));
                        queue.push_back((current.0, current.1.wrapping_add(1), Direction::Right));
                    }
                },
                '|' => match current.2 {
                    Direction::Left | Direction::Right => {
                        queue.push_back((
                            current.0.wrapping_add_signed(-1),
                            current.1,
                            Direction::Up,
                        ));
                        queue.push_back((current.0.wrapping_add(1), current.1, Direction::Down));
                    }
                    Direction::Up | Direction::Down => {
                        let delta = current.2.delta();
                        let new_row = current.0.wrapping_add_signed(delta.0);
                        let new_column = current.1.wrapping_add_signed(delta.1);
                        queue.push_back((new_row, new_column, current.2));
                    }
                },
                _ => return Err(AoCError::from("unknown tile")),
            }
        }
    }
    Ok(visited)
}
