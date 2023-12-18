use crate::error::AoCError;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    solve(input, |instruction| {
        let (direction, rest) = instruction
            .split_once(' ')
            .ok_or(AoCError::from("no direction"))?;
        let (count, _) = rest.split_once(' ').ok_or(AoCError::from("no count"))?;
        let count = count.parse::<isize>()?;
        let direction = match direction {
            "U" => Ok((-1, 0)),
            "D" => Ok((1, 0)),
            "L" => Ok((0, -1)),
            "R" => Ok((0, 1)),
            _ => Err(AoCError::from("unknown direction")),
        }?;
        Ok((direction, count))
    })
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    solve(input, |instruction| {
        let (_, instruction) = instruction
            .rsplit_once(' ')
            .ok_or(AoCError::from("no instruction"))?;
        let instruction = instruction.trim_matches(['(', ')', '#'].as_slice());
        let count = isize::from_str_radix(&instruction[..5], 16)?;
        let direction = match &instruction[5..] {
            "0" => Ok((0, 1)),
            "1" => Ok((1, 0)),
            "2" => Ok((0, -1)),
            "3" => Ok((-1, 0)),
            _ => Err(AoCError::from("unknown direction")),
        }?;
        Ok((direction, count))
    })
}

fn solve<F: Fn(&str) -> Result<((isize, isize), isize), AoCError>>(
    input: String,
    parse_instruction: F,
) -> Result<String, AoCError> {
    let mut current_row = 0;
    let mut current_column = 0;
    let mut map = input
        .trim()
        .lines()
        .map(|line| {
            let (direction, count) = parse_instruction(line)?;
            current_row += (direction.0 * count) as i64;
            current_column += (direction.1 * count) as i64;
            Ok((current_row, current_column))
        })
        .collect::<Result<Vec<(i64, i64)>, AoCError>>()?;
    map.insert(0, (0, 0));

    let border = map
        .windows(2)
        .fold(0, |a, c| {
            a + (c[0].0 - c[1].0).abs() + (c[0].1 - c[1].1).abs()
        })
        .abs();
    let area = map
        .windows(2)
        .fold(0, |a, c| a + c[0].0 * c[1].1 - c[0].1 * c[1].0)
        .abs()
        >> 1;
    let interior = area - (border >> 1) + 1;

    Ok((border + interior).to_string())
}
