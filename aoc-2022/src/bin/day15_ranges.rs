use std::{cmp::max, collections::HashSet};

use anyhow::{bail, ensure, Context, Result};
use aoc_2022::nom_util::{run_parser, IResult};
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::{pair, tuple},
};

fn main() -> Result<()> {
    for (s, p1_y, p2_max_n) in [
        (include_str!("../../data/example/day15.txt"), 10, 20),
        (
            include_str!("../../data/challenge/day15.txt"),
            2000000,
            4000000,
        ),
    ] {
        println!("{:#?}", solve(s, p1_y, p2_max_n)?)
    }
    Ok(())
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn mdist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    pos: Point,
    closest: Point,
}

impl Sensor {
    fn refutes(&self, loc: &Point) -> bool {
        self.pos.mdist(loc) <= self.pos.mdist(&self.closest)
    }
}

fn signed_int(input: &str) -> IResult<i32> {
    map_res(
        pair(opt(char::<&str, _>('-')), digit1),
        |(sign, num)| -> Result<i32> {
            let mut val: i32 = num.parse().context("signed int")?;
            if sign.is_some() {
                val *= -1;
            }
            Ok(val)
        },
    )(input)
}

fn single_sensor(input: &str) -> IResult<Sensor> {
    map_res(
        tuple((
            tag("Sensor at x="),
            signed_int,
            tag(", y="),
            signed_int,
            tag(": closest beacon is at x="),
            signed_int,
            tag(", y="),
            signed_int,
        )),
        |(_, px, _, py, _, bx, _, by)| -> Result<Sensor> {
            Ok(Sensor {
                pos: Point::new(px, py),
                closest: Point::new(bx, by),
            })
        },
    )(input)
}

fn parse_sensors(input: &str) -> IResult<Vec<Sensor>> {
    separated_list1(line_ending, single_sensor)(input)
}

fn merge_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    ranges.sort();

    let mut ret = Vec::with_capacity(ranges.len());
    let mut i = 0;
    while i < ranges.len() {
        let low = ranges[i].0;
        let mut high = ranges[i].1;
        i += 1;
        while i < ranges.len() {
            if high >= ranges[i].0 {
                high = max(high, ranges[i].1);
                i += 1;
            } else {
                break;
            }
        }
        ret.push((low, high))
    }
    ret
}

fn part1(input: &str, query_y: i32) -> Result<usize> {
    let sensors = run_parser(parse_sensors, input)?;
    ensure!(!sensors.is_empty(), "No sensors");

    let ranges: Vec<(i32, i32)> = sensors
        .iter()
        .flat_map(|s| {
            let dx = s.pos.mdist(&s.closest) - (s.pos.y - query_y).abs();
            if dx < 0 {
                None
            } else {
                Some((s.pos.x - dx, s.pos.x + dx))
            }
        })
        .collect();

    let refuted: i32 = merge_ranges(ranges)
        .into_iter()
        .map(|r| r.1 - r.0 + 1)
        .sum();

    let num_beacons = sensors
        .iter()
        .flat_map(|s| {
            if s.closest.y == query_y {
                Some(s.closest.clone())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .len();

    Ok(refuted as usize - num_beacons)
}

fn part2(input: &str, max_n: i32) -> Result<i64> {
    let sensors = run_parser(parse_sensors, input)?;
    ensure!(!sensors.is_empty(), "No sensors");

    for s in sensors.iter() {
        let dist = s.pos.mdist(&s.closest) + 1;
        for dx in (-dist)..=dist {
            let remaining = dist - dx;
            let x = s.pos.x + dx;
            if x < 0 || x > max_n {
                continue;
            }

            for y in [s.pos.y - remaining, s.pos.y + remaining] {
                if y < 0 || y > max_n {
                    continue;
                }
                let p = Point::new(x, y);
                if !sensors.iter().any(|s| s.refutes(&p)) {
                    return Ok(x as i64 * 4000000 + y as i64);
                }
            }
        }
    }
    bail!("No point found")
}

fn solve(input: &str, p1_y: i32, p2_max_n: i32) -> Result<(usize, i64)> {
    let part1 = part1(input, p1_y).context("part 1")?;
    let part2 = part2(input, p2_max_n).context("part 2")?;

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let answer = solve(include_str!("../../data/example/day15.txt"), 10, 20).unwrap();
        assert_eq!(answer, (26, 56000011));
    }

    #[test]
    fn challenge() {
        let answer = solve(
            include_str!("../../data/challenge/day15.txt"),
            2000000,
            4000000,
        )
        .unwrap();
        assert_eq!(answer, (5716881, 10852583132904));
    }

    #[test]
    fn test_mdist() {
        for (a, b, want) in [
            (Point::new(0, 0), Point::new(1, 1), 2),
            (Point::new(0, 0), Point::new(-1, -1), 2),
            (Point::new(-1, -1), Point::new(-1, -1), 0),
            (Point::new(0, 0), Point::new(10, 1), 11),
            (Point::new(0, 0), Point::new(2, 22), 24),
        ] {
            assert_eq!(a.mdist(&b), want, "{a:?} {b:?}");
            assert_eq!(b.mdist(&a), want, "{b:?} {a:?}");
        }
    }

    #[test]
    fn test_merge_ranges() {
        for (ranges, want) in [
            (vec![(0, 0)], vec![(0, 0)]),
            (vec![(0, 0), (1, 1)], vec![(0, 0), (1, 1)]),
            (vec![(0, 1), (1, 2)], vec![(0, 2)]),
            (vec![(0, 2), (1, 3)], vec![(0, 3)]),
            (vec![(0, 3), (1, 2)], vec![(0, 3)]),
            (vec![(1, 3), (0, 2)], vec![(0, 3)]),
            (vec![(0, 2), (1, 3), (2, 4), (5, 6)], vec![(0, 4), (5, 6)]),
        ] {
            assert_eq!(merge_ranges(ranges.clone()), want, "{ranges:?}")
        }
    }

    #[test]
    fn test_refutes() {
        for (beacon, pos, want) in [
            (
                Sensor {
                    pos: Point::new(8, 7),
                    closest: Point::new(2, 10),
                },
                Point::new(8, 7),
                true,
            ),
            (
                Sensor {
                    pos: Point::new(8, 7),
                    closest: Point::new(2, 10),
                },
                Point::new(8, 16),
                true,
            ),
            (
                Sensor {
                    pos: Point::new(8, 7),
                    closest: Point::new(2, 10),
                },
                Point::new(8, 17),
                false,
            ),
            (
                Sensor {
                    pos: Point::new(8, 7),
                    closest: Point::new(2, 10),
                },
                Point::new(14, 4),
                true,
            ),
            (
                Sensor {
                    pos: Point::new(8, 7),
                    closest: Point::new(2, 10),
                },
                Point::new(15, 4),
                false,
            ),
            (
                Sensor {
                    pos: Point::new(8, 7),
                    closest: Point::new(2, 10),
                },
                Point::new(14, 3),
                false,
            ),
        ] {
            assert_eq!(beacon.refutes(&pos), want, "{beacon:?} {pos:?}");
        }
    }
}
