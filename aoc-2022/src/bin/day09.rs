use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, bail, Result};

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: usize,
    part2: usize,
}

fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day09.txt");
    println!("{:#?}", solve(d)?);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let d = match s {
            "D" => Self::Down,
            "U" => Self::Up,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => bail!("Unrecognized direction {s}"),
        };
        Ok(d)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Move {
    dir: Dir,
    amount: u8,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dir, amount) = s.split_once(' ').ok_or_else(|| anyhow!("No space"))?;
        Ok(Self {
            dir: dir.parse()?,
            amount: amount.parse()?,
        })
    }
}

impl Iterator for Move {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.amount == 0 {
            return None;
        }

        let delta = match self.dir {
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };

        self.amount -= 1;
        return Some(delta);
    }
}

#[derive(Default, Hash, Debug, Clone, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct State {
    head: Coord,
    tail: Coord,
}

fn update_coord(h: &mut i32, t: &mut i32) {
    if *t < *h {
        *t += 1
    } else if *t > *h {
        *t -= 1
    }
}

fn simulate<F>(input: &str, mut visit: F) -> Result<()>
where
    F: FnMut(&State) -> Result<()>,
{
    // Parse input up-front so visit isn't called if there's a parse error
    let moves = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Move>, _>>()?;

    let mut s: State = Default::default();
    visit(&s)?;

    for m in moves {
        println!("{m:?}");
        for (dx, dy) in m {
            println!("({dx}, {dy})");
            s.head.x += dx;
            s.head.y += dy;

            match (s.head.x.abs_diff(s.tail.x), s.head.y.abs_diff(s.tail.y)) {
                // touching, nothing to do
                (0, 0) => {}
                (0, 1) => {}
                (1, 0) => {}
                (1, 1) => {}
                (2, 0) => update_coord(&mut s.head.x, &mut s.tail.x),
                (0, 2) => update_coord(&mut s.head.y, &mut s.tail.y),
                (2, 1) | (1, 2) => {
                    update_coord(&mut s.head.x, &mut s.tail.x);
                    update_coord(&mut s.head.y, &mut s.tail.y)
                }
                _ => bail!("Unhandled delta amount for {s:?}"),
            }
            println!("{s:?}");
            visit(&s)?;
        }
    }
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut seen = HashSet::new();
    simulate(input, |s| {
        seen.insert(s.tail.clone());
        Ok(())
    })?;
    Ok(seen.len())
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

fn solve(input: &str) -> Result<Answer> {
    let part1 = part1(input)?;
    let part2 = part2(input)?;

    Ok(Answer { part1, part2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let answer = solve(include_str!("../../data/example/day09.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: 13,
                part2: 0
            }
        );
    }
}
