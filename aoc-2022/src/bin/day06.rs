use anyhow::{bail, Result};

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
    let input: Vec<_> = input.chars().collect();
    let mut idx = 0;
    'outer: while idx <= input.len() - win_size {
        let win = &input[idx..idx + win_size];
        for (i, c) in win.iter().enumerate() {
            // Check before and after for duplicates
            if win[i + 1..].iter().any(|d| d == c) {
                idx = idx + i + 1;
                continue 'outer;
            }
        }
        return Ok(idx + win_size);
    }
    bail!("No starting point found")
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
}
