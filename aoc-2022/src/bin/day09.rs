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

#[derive(Default, Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn follow(&mut self, lead: &Knot) -> Result<()> {
        match (lead.x.abs_diff(self.x), lead.y.abs_diff(self.y)) {
            // touching, nothing to do
            (0, 0) => {}
            (0, 1) => {}
            (1, 0) => {}
            (1, 1) => {}
            (2, 0) => Knot::update_coord(lead.x, &mut self.x),
            (0, 2) => Knot::update_coord(lead.y, &mut self.y),
            _ => {
                Knot::update_coord(lead.x, &mut self.x);
                Knot::update_coord(lead.y, &mut self.y)
            }
        }
        Ok(())
    }

    fn update_coord(l: i32, t: &mut i32) {
        if *t < l {
            *t += 1
        } else if *t > l {
            *t -= 1
        }
    }
}

fn simulate<F>(input: &str, mut visit: F) -> Result<()>
where
    F: FnMut(&Knot) -> Result<()>,
{
    // Parse input up-front so visit isn't called if there's a parse error
    let moves = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Move>, _>>()?;

    let mut k = Knot::default();
    visit(&k)?;

    for m in moves {
        for (dx, dy) in m {
            k.x += dx;
            k.y += dy;
            visit(&k)?;
        }
    }
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut seen = HashSet::new();
    let mut tail = Knot::default();
    simulate(input, |head| {
        tail.follow(head)?;
        seen.insert(tail.clone());
        Ok(())
    })?;
    Ok(seen.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut seen = HashSet::new();
    let mut tails = [Knot::default(); 9];
    simulate(input, |head| {
        let mut lead = head;
        for t in tails.iter_mut() {
            t.follow(lead)?;
            lead = t
        }
        seen.insert(lead.clone());
        Ok(())
    })?;
    Ok(seen.len())
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
                part2: 1
            }
        );
    }

    #[test]
    fn example_larger() {
        let p2 = part2(include_str!("../../data/example/day09_larger.txt")).unwrap();
        assert_eq!(p2, 36);
    }
}
