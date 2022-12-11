use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use tinyvec::ArrayVec;

// This is the same as day04, but uses ArrayVec to avoid heap allocations.
// This still isn't no_std (due to formatting), but drastically reduces allocations.
// On x86_64-pc-windows-msvc with Rust 1.65, it goes from ~6K to ~120

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: u32,
    part2: u32,
}

fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day04.txt");
    println!("{:#?}", solve(d)?);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct R {
    lower: u32,
    upper: u32,
}

impl R {
    fn contains(&self, other: &R) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn overlaps(&self, other: &R) -> bool {
        let r = self.lower..=self.upper;
        r.contains(&other.lower) || r.contains(&other.upper)
    }
}

impl FromStr for R {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: ArrayVec<[_; 2]> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(anyhow!(
                "Range {:?} had {} parts, expected 2",
                s,
                parts.len()
            ));
        }
        let lower = parts[0].parse()?;
        let upper = parts[1].parse()?;
        Ok(R { lower, upper })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct P {
    first: R,
    second: R,
}

impl FromStr for P {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: ArrayVec<[_; 2]> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(anyhow!(
                "Pair {:?} had {} parts, expected 2",
                s,
                parts.len()
            ));
        }
        let first = parts[0].parse()?;
        let second = parts[1].parse()?;
        Ok(P { first, second })
    }
}

fn part1(input: &str) -> Result<u32> {
    let mut subset = 0;
    for (i, l) in input.lines().enumerate() {
        let p: P = l.parse().with_context(|| format!("Line {i}"))?;
        if p.first.contains(&p.second) || p.second.contains(&p.first) {
            subset += 1
        }
    }
    Ok(subset)
}

fn part2(input: &str) -> Result<u32> {
    let mut overlap = 0;
    for l in input.lines() {
        let p: P = l.parse()?;
        if p.first.overlaps(&p.second) || p.second.overlaps(&p.first) {
            overlap += 1
        }
    }
    Ok(overlap)
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
        let answer = solve(include_str!("../../data/example/day04.txt")).unwrap();
        assert_eq!(answer, Answer { part1: 2, part2: 4 });
    }

    #[test]
    fn challenge() {
        let answer = solve(include_str!("../../data/challenge/day04.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: 477,
                part2: 830
            }
        );
    }
}
