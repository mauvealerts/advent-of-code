use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::{anyhow, bail, Context, Result};

fn main() -> Result<()> {
    for s in [
        include_str!("../../data/example/day12.txt"),
        include_str!("../../data/challenge/day12.txt"),
    ] {
        eprintln!("{:#?}", solve(s)?)
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    rows: Vec<Vec<u8>>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.rows.get(x).map(|r| r.get(y)).flatten().copied()
    }

    fn position(&self, needle: u8) -> Option<Coord> {
        for x in 0..self.rows.len() {
            if let Some(y) = self.rows[x].iter().position(|hay| hay == &needle) {
                return Some((x, y));
            }
        }
        None
    }

    fn find_all(&self, needle: u8) -> Vec<Coord> {
        let mut ret = Vec::new();
        for x in 0..self.rows.len() {
            if let Some(y) = self.rows[x].iter().position(|hay| hay == &needle) {
                ret.push((x, y));
            }
        }
        ret
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let rows = s
            .lines()
            .map(|l| l.as_bytes().iter().copied().collect())
            .collect();
        Ok(Self { rows })
    }
}

type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct Graph {
    start: Coord,
    end: Coord,
    grid: Grid,
}

fn reach(from: u8, to: u8) -> bool {
    // We obviously want the start to reach anything, and anything to reach the end.
    // Additionally, nothing can reach start and the end can reach nothing.
    match (from, to) {
        (b'S', _) => true,
        (b'E', _) => false,
        (_, b'E') => b'z' <= (from + 1),
        (_, b'S') => false,
        (from, to) => to <= (from + 1),
    }
}

impl Graph {
    fn adj(&self, x: usize, y: usize) -> Result<Vec<Coord>> {
        let (ix, iy): (isize, isize) = (
            x.try_into().context("x to isize")?,
            y.try_into().context("y to isize")?,
        );
        let from = self
            .grid
            .get(x, y)
            .ok_or_else(|| anyhow!("{x}, {y} is out of bounds"))?;

        let mut ret = Vec::with_capacity(4);
        for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
            let tx: usize = if let Ok(v) = (ix + dx).try_into() {
                v
            } else {
                continue;
            };
            let ty: usize = if let Ok(v) = (iy + dy).try_into() {
                v
            } else {
                continue;
            };

            if let Some(to) = self.grid.get(tx, ty) {
                if reach(from, to) {
                    ret.push((tx, ty));
                }
            }
        }
        Ok(ret)
    }

    fn shortest_path(&self, x: usize, y: usize) -> Result<usize> {
        let mut seen = HashSet::with_capacity(self.grid.rows.len() * self.grid.rows[0].len());
        let mut q = VecDeque::with_capacity(self.grid.rows.len());
        q.push_back(((x, y), 0));
        while let Some(((x, y), dist)) = q.pop_front() {
            if seen.contains(&(x, y)) {
                continue;
            }
            seen.insert((x, y));
            if b'E' == self.grid.get(x, y).context("visiting")? {
                return Ok(dist);
            }
            for dest in self.adj(x, y).context("traversing")? {
                // This check isn't necesesary
                if !seen.contains(&dest) {
                    q.push_back((dest, dist + 1))
                }
            }
        }
        bail!("No path found")
    }
}

impl TryFrom<Grid> for Graph {
    type Error = anyhow::Error;

    fn try_from(grid: Grid) -> Result<Self> {
        let start = grid.position(b'S').context("find S")?;
        let end = grid.position(b'E').context("find E")?;
        Ok(Self { start, end, grid })
    }
}

fn part1(input: &str) -> Result<usize> {
    let g: Graph = input.parse::<Grid>()?.try_into()?;
    g.shortest_path(g.start.0, g.start.1)
}

fn part2(input: &str) -> Result<usize> {
    let g: Graph = input.parse::<Grid>()?.try_into()?;
    let m = g
        .grid
        .find_all(b'a')
        .into_iter()
        .map(|(x, y)| g.shortest_path(x, y).ok())
        .flatten()
        .min()
        .unwrap();
    Ok(m)
}

fn solve(input: &str) -> Result<(usize, usize)> {
    let part1 = part1(input).context("part 1")?;
    let part2 = part2(input).context("part 2")?;

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn example() {
    //     let answer = solve(include_str!("../../data/example/template.txt")).unwrap();
    //     assert_eq!(answer, (0, 0));
    // }

    // #[test]
    // fn challenge() {
    //     let answer = solve(include_str!("../../data/challenge/template.txt")).unwrap();
    //     assert_eq!(answer, (0, 0));
    // }

    #[test]
    fn test_grid_parse() {
        for (s, want_rows) in [
            ("", vec![]),
            ("v", vec![vec![b'v']]),
            ("ab\ncd", vec![vec![b'a', b'b'], vec![b'c', b'd']]),
        ] {
            assert_eq!(
                s.parse::<Grid>().with_context(|| format!("{s:?}")).unwrap(),
                Grid { rows: want_rows },
                "{s:?}"
            )
        }
    }

    #[test]
    fn test_reach() {
        for ((from, to), want) in [
            (('S', 'a'), true),
            (('S', 'z'), true),
            (('a', 'S'), false),
            (('z', 'S'), false),
            (('a', 'E'), false),
            (('x', 'E'), false),
            (('y', 'E'), true),
            (('z', 'E'), true),
            (('E', 'a'), false),
            (('E', 'z'), false),
            (('a', 'a'), true),
            (('x', 'y'), true),
            (('x', 'a'), true),
            (('x', 'z'), false),
            (('a', 'z'), false),
        ] {
            assert_eq!(reach(from as u8, to as u8), want, "{from} -> {to}")
        }
    }

    #[test]
    fn test_position() {
        let g = Grid {
            rows: vec![vec![b'S', b'a'], vec![b'x', b'E']],
        };
        for (needle, want) in [
            (b'S', (0, 0)),
            (b'a', (0, 1)),
            (b'x', (1, 0)),
            (b'E', (1, 1)),
        ] {
            assert_eq!(g.position(needle), Some(want), "{:?}", needle as char)
        }
    }

    #[test]
    fn test_adj() {
        let g = Graph {
            start: (0, 0),
            end: (0, 2),
            grid: Grid {
                rows: vec![
                    vec![b'S', b'a', b'E'],
                    vec![b'x', b'b', b'c'],
                    vec![b'y', b'c', b'd'],
                ],
            },
        };
        for ((x, y), mut want) in [
            ((0, 0), vec![(0, 1), (1, 0)]),
            ((0, 1), vec![(1, 1)]),
            ((1, 1), vec![(0, 1), (1, 2), (2, 1)]),
            ((0, 2), vec![]),
            ((2, 2), vec![(2, 1), (1, 2)]),
        ] {
            let mut got = g.adj(x, y).with_context(|| format!("{x}, {y}")).unwrap();
            got.sort();
            want.sort();
            assert_eq!(got, want, "({x}, {y})")
        }
    }

    // #[test]
    // fn test_part1() {
    //     for (s, want) in [
    //         ("SE", 1),
    //         ("SabE", 3),
    //         (concat!("Sab\n", "mzc\n", "nzE"), 4),
    //     ] {
    //         assert_eq!(
    //             part1(s).with_context(|| format!("{s:?}")).unwrap(),
    //             want,
    //             "{s:?}"
    //         )
    //     }
    // }
}
