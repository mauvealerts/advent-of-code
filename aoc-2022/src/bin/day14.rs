#![feature(array_windows)]

use std::cmp::{max, min};

use anyhow::{ensure, Context, Result};
use aoc_2022::nom_util::{run_parser, IResult};
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::map_res,
    error::VerboseError,
    multi::separated_list1,
    sequence::separated_pair,
};

fn main() -> Result<()> {
    for s in [
        include_str!("../../data/example/day14.txt"),
        include_str!("../../data/challenge/day14.txt"),
    ] {
        println!("{:#?}", solve(s)?)
    }
    Ok(())
}

type RLine = Vec<(usize, usize)>;

fn rock_line(input: &str) -> IResult<RLine> {
    separated_list1(
        tag::<_, _, VerboseError<_>>(" -> "),
        map_res(
            separated_pair(digit1::<&str, _>, char(','), digit1),
            |(x, y)| -> Result<_> { Ok((x.parse()?, y.parse()?)) },
        ),
    )(input)
}

fn all_lines(input: &str) -> IResult<Vec<RLine>> {
    separated_list1(line_ending, rock_line)(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    g: Vec<Vec<bool>>,
    w: usize,
    h: usize,
}

impl Grid {
    fn floored(mut self) -> Self {
        self.g.push(vec![false; self.w]);
        self.g.push(vec![true; self.w]);
        self.h += 2;
        self
    }

    fn fill_to_abyss(&mut self) -> usize {
        'grain: for n in 0.. {
            let mut x = 500;
            // Note: it's impossible for sand to rest on the last row (right???)
            for y in 0..self.h - 1 {
                if !self.g[y + 1][x] {
                    continue;
                }

                if x == 0 {
                    return n;
                }
                if !self.g[y + 1][x - 1] {
                    x -= 1;
                    continue;
                }

                if x == self.w - 1 {
                    return n;
                }
                if !self.g[y + 1][x + 1] {
                    x += 1;
                    continue;
                }

                self.g[y][x] = true;
                continue 'grain;
            }
            return n;
        }
        unreachable!()
    }

    fn fill_to_top(&mut self) -> usize {
        'grain: for n in 1.. {
            let mut x = 500;
            // Note: it's impossible for sand to rest on the last row (right???)
            for y in 0..self.h - 1 {
                if !self.g[y + 1][x] {
                    continue;
                }

                if x == 0 {
                    return n;
                }
                if !self.g[y + 1][x - 1] {
                    x -= 1;
                    continue;
                }

                if x == self.w - 1 {
                    return n;
                }
                if !self.g[y + 1][x + 1] {
                    x += 1;
                    continue;
                }

                self.g[y][x] = true;
                if y == 0 && x == 500 {
                    return n;
                }
                continue 'grain;
            }
        }
        unreachable!()
    }
}

impl TryFrom<Vec<RLine>> for Grid {
    type Error = anyhow::Error;

    fn try_from(lines: Vec<RLine>) -> Result<Self> {
        ensure!(None != lines.iter().flatten().next(), "No points");
        let want_w = 1 + lines.iter().flatten().map(|(x, _)| x).max().unwrap();
        let h = 1 + lines.iter().flatten().map(|(_, y)| y).max().unwrap();

        // Part 2 requires the area to extend further. There isn't an obvious way to pick a number
        // based on input data.
        let w = 1000;
        ensure!(want_w < w, "Rock outside area");
        let mut g = vec![vec![false; w]; h];
        for l in lines {
            for [a, b] in l.array_windows() {
                let (fx, tx) = (min(a.0, b.0), max(a.0, b.0));
                let (fy, ty) = (min(a.1, b.1), max(a.1, b.1));
                for i in fy..=ty {
                    for j in fx..=tx {
                        g[i][j] = true;
                    }
                }
            }
        }
        Ok(Self { g, w, h })
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut g: Grid = run_parser(all_lines, input)?.try_into()?;
    Ok(g.fill_to_abyss())
}

fn part2(input: &str) -> Result<usize> {
    let mut g: Grid = run_parser(all_lines, input)?.try_into()?;
    g = g.floored();
    let n = g.fill_to_top();
    // for r in g.g.iter() {
    //     let s: String = (&r[488..])
    //         .iter()
    //         .map(|f| if *f { '#' } else { '.' })
    //         .collect();
    //     println!("{s}")
    // }

    Ok(n)
}

fn solve(input: &str) -> Result<(usize, usize)> {
    let part1 = part1(input).context("part 1")?;
    let part2 = part2(input).context("part 2")?;

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let answer = solve(include_str!("../../data/example/day14.txt")).unwrap();
        assert_eq!(answer, (24, 93));
    }

    #[test]
    fn challenge() {
        let answer = solve(include_str!("../../data/challenge/day14.txt")).unwrap();
        assert_eq!(answer, (1199, 23925));
    }
}
