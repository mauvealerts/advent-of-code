use std::collections::HashSet;

use anyhow::{anyhow, Result};

// This was the solution I wrote first

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: usize,
    part2: usize,
}

fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day06.txt");
    println!("{:#?}", solve(d)?);
    Ok(())
}

fn find_distinct(input: &str, win_size: usize) -> Result<usize> {
    input
        .as_bytes()
        .windows(win_size)
        .position(|w| win_size == w.iter().collect::<HashSet<_>>().len())
        .map(|i| i + win_size)
        .ok_or_else(|| anyhow!("No starting point found"))
}

fn part1(input: &str) -> Result<usize> {
    find_distinct(input, 4)
}

fn part2(input: &str) -> Result<usize> {
    find_distinct(input, 14)
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
        for (inp, (part1, part2)) in include_str!("../../data/example/day06.txt").lines().zip([
            (7, 19),
            (5, 23),
            (6, 23),
            (10, 29),
            (11, 26),
        ]) {
            let answer = solve(inp).unwrap();
            assert_eq!(answer, Answer { part1, part2 }, "input: {inp:?}");
        }
    }

    #[test]
    fn challenge() {
        let answer = solve(include_str!("../../data/challenge/day06.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: 1275,
                part2: 3605
            }
        );
    }
}
