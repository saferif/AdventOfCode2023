use crate::error::AoCError;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bnum::cast::As;
use bnum::types::I512;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let hailstones = parse_hailstones(input)?;
    let hailstones = hailstones
        .into_iter()
        .map(|((x, y, _), (vx, vy, _))| ((x, y), (x + vx, y + vy)))
        .collect::<Vec<_>>();
    let answer = hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, &stone1)| {
            hailstones[i + 1..]
                .iter()
                .filter_map(move |&stone2| intersect2d(stone1, stone2).map(|c| (stone1, stone2, c)))
        })
        .filter_map(|(stone1, stone2, c)| {
            (is_future(stone1, c) && is_future(stone2, c)).then_some(c)
        })
        .filter(|c| {
            (200000000000000.0 <= c.0 && c.0 <= 400000000000000.0)
                && (200000000000000.0 <= c.1 && c.1 <= 400000000000000.0)
        })
        .count();
    Ok(answer.to_string())
}

pub(crate) fn part2(input: String) -> Result<String, AoCError> {
    let hailstones = parse_hailstones(input)?;
    let answer = (-1000..=1000)
        .find_map(|z_speed| {
            relative_speeds(&hailstones, z_speed)
                .and_then(simplify_speeds)
                .map(|v| {
                    v.into_iter()
                        .map(|(x, y)| (I512::from(x), I512::from(y)))
                        .collect()
                })
                .map(|rs| crt(rs))
                .and_then(|z_coord| try_z(&hailstones, z_speed, z_coord.as_()))
        })
        .ok_or(AoCError::from("no answer"))?;
    Ok(answer.to_string())
}

fn parse_hailstones(input: String) -> Result<Vec<((i64, i64, i64), (i64, i64, i64))>, AoCError> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (coordinates, velocity) = line
                .split_once('@')
                .ok_or(AoCError::from("invalid format"))?;

            let (x, rest) = coordinates
                .split_once(',')
                .ok_or(AoCError::from("invalid format"))?;
            let (y, z) = rest
                .split_once(',')
                .ok_or(AoCError::from("invalid format"))?;

            let (vx, rest) = velocity
                .split_once(',')
                .ok_or(AoCError::from("invalid format"))?;
            let (vy, vz) = rest
                .split_once(',')
                .ok_or(AoCError::from("invalid format"))?;

            let x = x.trim().parse::<i64>()?;
            let y = y.trim().parse::<i64>()?;
            let z = z.trim().parse::<i64>()?;

            let vx = vx.trim().parse::<i64>()?;
            let vy = vy.trim().parse::<i64>()?;
            let vz = vz.trim().parse::<i64>()?;

            Ok(((x, y, z), (vx, vy, vz)))
        })
        .collect()
}

fn intersect2d(p1: ((i64, i64), (i64, i64)), p2: ((i64, i64), (i64, i64))) -> Option<(f64, f64)> {
    let s1 = (p1.0 .0 as i128, p1.0 .1 as i128, 1);
    let e1 = (p1.1 .0 as i128, p1.1 .1 as i128, 1);
    let s2 = (p2.0 .0 as i128, p2.0 .1 as i128, 1);
    let e2 = (p2.1 .0 as i128, p2.1 .1 as i128, 1);
    let (kx, ky, k) = cross3d(cross3d(s1, e1), cross3d(s2, e2));
    (k != 0).then_some((kx as f64 / k as f64, ky as f64 / k as f64))
}

fn cross3d(p1: (i128, i128, i128), p2: (i128, i128, i128)) -> (i128, i128, i128) {
    (
        p1.1 * p2.2 - p1.2 * p2.1,
        p1.2 * p2.0 - p1.0 * p2.2,
        p1.0 * p2.1 - p1.1 * p2.0,
    )
}

fn is_future(stone: ((i64, i64), (i64, i64)), point: (f64, f64)) -> bool {
    (stone.1 .0 - stone.0 .0).signum() == f64signum(point.0 - stone.0 .0 as f64)
        && (stone.1 .1 - stone.0 .1).signum() == f64signum(point.1 - stone.0 .1 as f64)
}

fn f64signum(x: f64) -> i64 {
    if x < 0.0 {
        -1
    } else if x > 0.0 {
        1
    } else {
        0
    }
}

fn simplify_speeds<I: IntoIterator<Item = (i64, i64)>>(speeds: I) -> Option<Vec<(i64, i64)>> {
    let mut modulos = BTreeMap::<i64, i64>::new();
    speeds
        .into_iter()
        .all(|(prod, rem)| {
            let mut current = prod;
            let mut primes = BTreeSet::new();
            for i in 2..prod {
                while current % i == 0 {
                    primes.insert(i);
                    current /= i;
                }
            }
            if current != 1 {
                primes.insert(current);
            }

            primes.into_iter().all(|prime| {
                let correct = !modulos
                    .get(&prime)
                    .copied()
                    .into_iter()
                    .any(|v| v != rem % prime);
                if correct {
                    modulos.insert(prime, rem % prime);
                }
                correct
            })
        })
        .then(|| modulos.into_iter().collect())
}

fn try_z(
    stones: &Vec<((i64, i64, i64), (i64, i64, i64))>,
    z_speed: i64,
    z_coord: i64,
) -> Option<i64> {
    let stone0 = stones.get(0)?;
    let stone1 = stones.get(1)?;

    let collision_time1 = (stone0.0 .2 - z_coord) / (z_speed - stone0.1 .2);
    let collision_time_rem = (stone0.0 .2 - z_coord) % (z_speed - stone0.1 .2);
    if collision_time_rem != 0 {
        return None;
    }
    let collision_time2 = (stone1.0 .2 - z_coord) / (z_speed - stone1.1 .2);
    let collision1 = position_at(stone0, collision_time1);
    let collision2 = position_at(stone1, collision_time2);

    let time_delta = collision_time2 - collision_time1;

    let x_speed = (collision2.0 - collision1.0) / time_delta;
    let x_rem = (collision2.0 - collision1.0) % time_delta;
    let y_speed = (collision2.1 - collision1.1) / time_delta;
    let y_rem = (collision2.1 - collision1.1) % time_delta;
    if x_rem != 0 || y_rem != 0 {
        return None;
    }

    let x_coord = collision1.0 - collision_time1 * x_speed;
    let y_coord = collision1.1 - collision_time1 * y_speed;

    let our_stone = ((x_coord, y_coord, z_coord), (x_speed, y_speed, z_speed));
    stones
        .iter()
        .all(|stone| {
            let collision_time = (stone.0 .2 - z_coord) / (z_speed - stone.1 .2);
            position_at(stone, collision_time) == position_at(&our_stone, collision_time)
        })
        .then_some(x_coord + y_coord + z_coord)
}

fn position_at(stone: &((i64, i64, i64), (i64, i64, i64)), time: i64) -> (i64, i64, i64) {
    (
        stone.0 .0 + stone.1 .0 * time,
        stone.0 .1 + stone.1 .1 * time,
        stone.0 .2 + stone.1 .2 * time,
    )
}

fn relative_speeds(
    stones: &Vec<((i64, i64, i64), (i64, i64, i64))>,
    z_speed: i64,
) -> Option<Vec<(i64, i64)>> {
    let mut min_position = i64::MIN;
    let mut max_position = i64::MAX;
    let speeds = stones
        .iter()
        .filter_map(|stone| {
            let rs = stone.1 .2 - z_speed;
            let z_position = stone.0 .2;
            if rs >= 0 {
                min_position = min_position.max(z_position);
            }
            if rs <= 0 {
                max_position = max_position.min(z_position);
            }
            let rs = rs.abs();
            (rs != 0).then(|| (rs, z_position % rs))
        })
        .collect();
    (min_position <= max_position).then_some(speeds)
}

fn mod_inverse(a: I512, m: I512) -> I512 {
    if m == I512::ONE {
        return I512::ZERO;
    }
    let mut temp;
    let mut a = a;
    let mut m = m;
    let mut x0 = I512::ZERO;
    let mut x1 = I512::ONE;
    while a > I512::ONE {
        let q = a.clone() / m.clone();
        temp = m.clone();
        m = a % m;
        a = temp;
        temp = x0.clone();
        x0 = x1 - q * x0;
        x1 = temp;
    }
    x1
}

fn crt(vals: Vec<(I512, I512)>) -> I512 {
    let product = vals.iter().map(|(x, _)| x).product::<I512>();
    let result = vals
        .iter()
        .map(|(n, r)| {
            let pp = product.clone() / n.clone();
            r * mod_inverse(pp.clone(), n.clone()) * pp
        })
        .sum::<I512>();
    let mut base = result % product.clone();
    while base < I512::ZERO {
        base += product.clone();
    }
    base
}
