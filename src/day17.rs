use crate::error::AoCError;
use alloc::collections::{BTreeMap, BinaryHeap};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::Reverse;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    solve(input, |direction, cnt| {
        let mut new_directions = Vec::new();
        if cnt < 3 {
            new_directions.push((direction, cnt + 1));
        }
        new_directions.push(((direction.1, -direction.0), 1));
        new_directions.push(((-direction.1, direction.0), 1));
        new_directions
    })
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    solve(input, |direction, cnt| {
        let mut new_directions = Vec::new();
        if cnt < 10 {
            new_directions.push((direction, cnt + 1));
        }
        if cnt >= 4 {
            new_directions.push(((direction.1, -direction.0), 1));
            new_directions.push(((-direction.1, direction.0), 1));
        }
        new_directions
    })
}

fn solve<F: Fn((isize, isize), u8) -> Vec<((isize, isize), u8)>>(
    input: String,
    new_directions: F,
) -> Result<String, AoCError> {
    let map = input
        .trim()
        .lines()
        .map(|line| {
            Ok(line
                .chars()
                .map(|c| c.to_digit(10).ok_or(AoCError::from("not a digit")))
                .collect::<Result<Vec<_>, _>>()?)
        })
        .collect::<Result<Vec<_>, AoCError>>()?;

    let n = map.len();
    let m = map[0].len();
    let mut losses = BTreeMap::new();

    dijkstra(&map, &mut losses, new_directions);

    let res = losses
        .into_iter()
        .filter_map(|(key, value)| (key.0 == (n - 1, m - 1)).then_some(value))
        .min()
        .unwrap_or(u32::MAX);
    Ok(res.to_string())
}

fn dijkstra<F: Fn((isize, isize), u8) -> Vec<((isize, isize), u8)>>(
    map: &Vec<Vec<u32>>,
    losses: &mut BTreeMap<((usize, usize), (isize, isize), u8), u32>,
    new_directions: F,
) {
    let mut queue = BinaryHeap::<Reverse<(u32, (usize, usize), (isize, isize), u8)>>::from([
        Reverse((0, (0, 0), (0, 1), 0)),
        Reverse((0, (0, 0), (1, 0), 0)),
    ]);
    while !queue.is_empty() {
        let (path_loss, position, direction, cnt) = queue.pop().unwrap().0;
        if losses.contains_key(&(position, direction, cnt)) {
            continue;
        }
        losses.insert((position, direction, cnt), path_loss);

        new_directions(direction, cnt)
            .into_iter()
            .for_each(|(new_direction, new_cnt)| {
                let new_row = position.0.wrapping_add_signed(new_direction.0);
                let new_column = position.1.wrapping_add_signed(new_direction.1);
                if let Some(&loss) = map.get(new_row).and_then(|row| row.get(new_column)) {
                    let new_losses = path_loss + loss;
                    queue.push(Reverse((
                        new_losses,
                        (new_row, new_column),
                        new_direction,
                        new_cnt,
                    )));
                }
            });
    }
}
