use std::collections::HashSet;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: u32,
    part2: u32,
}
fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day03.txt");
    println!("{:#?}", solve(d)?);
    Ok(())
}

trait Prioritized {
    fn priority(&self) -> Result<u32>;
}

const LC_A: u32 = 'a' as u32;
const LC_Z: u32 = 'z' as u32;
const UC_A: u32 = 'A' as u32;
const UC_Z: u32 = 'Z' as u32;

impl Prioritized for char {
    fn priority(&self) -> Result<u32> {
        let num = *self as u32;

        let val = if (UC_A..=UC_Z).contains(&num) {
            num - UC_A + 27
        } else if (LC_A..=LC_Z).contains(&num) {
            num - LC_A + 1
        } else {
            return Err(anyhow!("{:?} isn't within [a, z] nor [A, Z]", self));
        };
        Ok(val)
    }
}

fn part1(input: &str) -> Result<u32> {
    let mut total = 0;
    for (i, l) in input.lines().enumerate() {
        let l: Vec<_> = l.chars().collect();
        if l.len() % 2 == 1 {
            return Err(anyhow!("line {} had length {}, must be even", i, l.len()));
        }
        let mid = l.len() / 2;
        let first: HashSet<_> = l[..mid].iter().collect();
        let second: HashSet<_> = l[mid..].iter().collect();
        let inter: Vec<_> = first.intersection(&second).collect();
        if inter.len() != 1 {
            return Err(anyhow!(
                "intersection {:?} had length {}, must be 1",
                inter,
                inter.len()
            ));
        }
        total += inter[0].priority()?
    }

    Ok(total)
}

fn part2(input: &str) -> Result<u32> {
    let lines: Vec<_> = input.lines().collect();
    if lines.len() % 3 != 0 {
        return Err(anyhow!("{} lines, must be multiple of 3", lines.len()));
    }
    let mut total = 0;
    for group in lines.chunks_exact(3) {
        let mut all: Vec<HashSet<_>> = group.iter().map(|l| l.chars().collect()).collect();

        let mut inter = all.pop().unwrap();
        for b in all {
            inter.retain(|e| b.contains(e));
        }

        if inter.len() != 1 {
            return Err(anyhow!(
                "intersection {:?} had length {}, must be 1",
                inter,
                inter.len()
            ));
        }
        total += inter.iter().next().unwrap().priority()?
    }

    Ok(total)
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
        let answer = solve(include_str!("../../data/example/day03.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: 157,
                part2: 70
            }
        );
    }

    #[test]
    fn challenge() {
        let answer = solve(include_str!("../../data/challenge/day03.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: 7446,
                part2: 2646
            }
        );
    }
}
