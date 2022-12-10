use std::str::FromStr;

use anyhow::{anyhow, ensure, Context, Ok, Result};

fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day10.txt");
    let s = solve(d)?;
    println!("{}", s.0);
    println!("{}", s.1);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Inst {
    Noop,
    Addx(i8),
}

impl Inst {
    fn into_iter(self) -> impl Iterator<Item = i8> {
        match self {
            Self::Noop => vec![0].into_iter(),
            Self::Addx(v) => vec![0, v].into_iter(),
        }
    }
}

impl FromStr for Inst {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "noop" => Ok(Self::Noop),
            _ => {
                let (name, num) = s.split_once(' ').ok_or_else(|| anyhow!("No space"))?;
                ensure!(name == "addx", "Unexpected first part {name:?}");
                let i = num.parse().context("Parsing addx")?;
                Ok(Self::Addx(i))
            }
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Inst>> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| l.parse::<Inst>().with_context(|| format!("Line {i} {l:?}")))
        .collect()
}

fn cycles(inst: Vec<Inst>) -> impl Iterator<Item = (i32, i32)> {
    inst.into_iter()
        .map(|op| op.into_iter())
        .flatten()
        .enumerate()
        .scan(1_i32, |state, (pc, delta)| {
            let prev = *state;
            let pc = (pc + 1) as i32;
            *state += delta as i32;
            Some((pc, prev))
        })
}

fn part1(input: &str) -> Result<i32> {
    let mut score = 0;
    for (pc, val) in cycles(parse_input(input)?) {
        match pc {
            20 | 60 | 100 | 140 | 180 | 220 => score += val * pc,
            _ => {}
        }
    }
    Ok(score)
}

fn part2(input: &str) -> Result<String> {
    let mut screen = "".to_owned();
    for (pc, val) in cycles(parse_input(input)?) {
        let y = (pc - 1) % 40;
        let ch = if y.abs_diff(val) < 2 { "#" } else { "." };
        screen += ch;
        if y == 39 {
            screen += "\n"
        }
    }
    Ok(screen)
}

fn solve(input: &str) -> Result<(i32, String)> {
    let part1 = part1(input).context("part 1")?;
    let part2 = part2(input).context("part 2")?;

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let answer = solve(include_str!("../../data/example/day10.txt")).unwrap();
        assert_eq!(
            answer,
            (
                13140,
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
                .to_owned()
            )
        );
    }
}
