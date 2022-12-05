use std::str::FromStr;

use anyhow::{anyhow, Context, Result};

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: String,
    part2: String,
}

fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day05.txt");
    println!("{:#?}", solve(d)?);
    Ok(())
}

struct Move {
    count: usize,
    src: usize,
    dest: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" ");

        parts.next().ok_or_else(|| anyhow!("move has no parts"))?;
        let count: usize = parts
            .next()
            .ok_or_else(|| anyhow!("move has no count"))?
            .parse()
            .context("move count")?;

        parts
            .next()
            .ok_or_else(|| anyhow!("move has only 2 parts"))?;
        let src: usize = parts
            .next()
            .ok_or_else(|| anyhow!("move has no source"))?
            .parse()
            .context("move count")?;
        if src < 1 {
            return Err(anyhow!("source must be positive"));
        }

        parts
            .next()
            .ok_or_else(|| anyhow!("move has only 4 parts"))?;
        let dest: usize = parts
            .next()
            .ok_or_else(|| anyhow!("move has no destination"))?
            .parse()
            .context("move count")?;
        if dest < 1 {
            return Err(anyhow!("source must be positive"));
        }

        let (src, dest) = (src - 1, dest - 1);

        Ok(Self { count, src, dest })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Crates {
    columns: Vec<Vec<char>>,
}

impl Crates {
    fn check_bounds(&self, m: &Move) -> Result<()> {
        if !(0..self.columns.len()).contains(&m.src) {
            return Err(anyhow!(
                "source was {}, expected within [0, {})",
                m.src,
                self.columns.len()
            ));
        }
        if !(0..self.columns.len()).contains(&m.dest) {
            return Err(anyhow!(
                "destination was {}, expected within [0, {})",
                m.dest,
                self.columns.len()
            ));
        }
        if m.count > self.columns[m.src].len() {
            return Err(anyhow!(
                "count is {} but column only has {}",
                m.count,
                self.columns.len()
            ));
        }
        Ok(())
    }

    fn run_slow(&mut self, m: &Move) -> Result<()> {
        self.columns[m.dest].reserve(m.count);
        for _ in 0..m.count {
            let tmp = self.columns[m.src].pop().unwrap();
            self.columns[m.dest].push(tmp)
        }
        Ok(())
    }

    fn run_fast(&mut self, m: &Move) -> Result<()> {
        let start = self.columns[m.src].len() - m.count;
        let items: Vec<_> = self.columns[m.src].drain(start..).collect();
        self.columns[m.dest].extend(items);
        Ok(())
    }
}

impl FromStr for Crates {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // last line is labels
        let num_rows = s.lines().count() - 1;
        let mut columns = Vec::new();
        let mut expected_len = 0;
        for (i, l) in s.lines().enumerate() {
            if i == 0 {
                if l.len() % 4 != 3 {
                    return Err(anyhow!(
                        "Line {} has length {} isn't 1 less than a multiple of 4",
                        i,
                        l.len()
                    ));
                }
                expected_len = l.len();
                let num_columns = l.len() / 4 + 1;
                columns.extend((0..num_columns).map(|_| Vec::with_capacity(num_rows)))
            }
            if l.len() != expected_len {
                return Err(anyhow!("Line {} has length {}, expected", i, expected_len));
            }
            for (j, c) in l.chars().skip(1).step_by(4).enumerate() {
                if c.is_numeric() {
                    break;
                }
                if c.is_whitespace() {
                    continue;
                }
                if !c.is_alphabetic() {
                    return Err(anyhow!("Line {} stack {}: {:?} isn't alphabetic", i, j, c));
                }
                columns[j].push(c)
            }
        }
        for c in columns.iter_mut() {
            c.reverse();
        }
        Ok(Self { columns })
    }
}

fn simulate(input: &str, run: fn(&mut Crates, &Move) -> Result<()>) -> Result<String> {
    const BLANK_UNIX: &str = "\n\n";
    const BLANK_WIN: &str = "\r\n\r\n";
    let (board_end, move_start) = if let Some(pos) = input.find(BLANK_UNIX) {
        (pos, pos + BLANK_UNIX.len())
    } else if let Some(pos) = input.find(BLANK_WIN) {
        (pos, pos + BLANK_WIN.len())
    } else {
        return Err(anyhow!("Didn't find a blank line"));
    };

    let mut crates: Crates = input[..board_end].parse()?;

    for (i, l) in input[move_start..].lines().enumerate() {
        let m: Move = l.parse().with_context(|| format!("Move {}", i))?;
        crates
            .check_bounds(&m)
            .with_context(|| format!("Move {}", i))?;
        run(&mut crates, &m)?;
    }

    let mut ret = "".to_owned();
    for col in crates.columns {
        if let Some(ch) = col.last() {
            ret.push(*ch)
        }
    }
    Ok(ret)
}

fn part1(input: &str) -> Result<String> {
    simulate(input, Crates::run_slow)
}

fn part2(input: &str) -> Result<String> {
    simulate(input, Crates::run_fast)
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
        let answer = solve(include_str!("../../data/example/day05.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: "CMZ".to_owned(),
                part2: "MCD".to_owned()
            }
        );
    }
}
