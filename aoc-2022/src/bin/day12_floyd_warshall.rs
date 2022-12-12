use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    usize,
};

use anyhow::{anyhow, Context, Result};

// This implements Floyd-Warshall, but it's hideously slow because the algorithm is O(V^3).
// The challenge input is 3,280 vertices. So the inner loop runs roughly 3.5e10 times

fn main() -> Result<()> {
    for s in [
        include_str!("../../data/example/day12.txt"),
        include_str!("../../data/challenge/day12.txt"),
    ] {
        println!("{:#?}", solve(s)?)
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    rows: Vec<Vec<u8>>,
}

impl Grid {
    fn coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.rows.len()).flat_map(|x| {
            (0..self.rows[0].len()).map(move |y| (x.try_into().unwrap(), y.try_into().unwrap()))
        })
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y): (usize, usize) = (x.try_into().unwrap(), y.try_into().unwrap());
        self.rows.get(x).and_then(|r| r.get(y)).copied()
    }

    fn position(&self, needle: u8) -> Option<Coord> {
        for x in 0..self.rows.len() {
            if let Some(y) = self.rows[x].iter().position(|hay| hay == &needle) {
                return Some((x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
        None
    }

    fn find_all(&self, needle: u8) -> Vec<Coord> {
        let mut ret = Vec::new();
        for x in 0..self.rows.len() {
            if let Some(y) = self.rows[x].iter().position(|hay| hay == &needle) {
                ret.push((x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
        ret
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let rows = s.lines().map(|l| l.as_bytes().to_vec()).collect();
        Ok(Self { rows })
    }
}

type Coord = (isize, isize);

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
    fn adj(&self, x: isize, y: isize) -> Result<Vec<Coord>> {
        let from = self
            .grid
            .get(x, y)
            .ok_or_else(|| anyhow!("{x}, {y} is out of bounds"))?;

        let mut ret = Vec::with_capacity(4);
        for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
            let (tx, ty) = (x + dx, y + dy);
            if let Some(to) = self.grid.get(tx, ty) {
                if reach(from, to) {
                    ret.push((tx, ty));
                }
            }
        }
        Ok(ret)
    }

    fn shortest_path(&self, x: isize, y: isize) -> Result<Option<usize>> {
        let mut seen = HashSet::with_capacity(self.grid.rows.len() * self.grid.rows[0].len());
        let mut q = VecDeque::with_capacity(self.grid.rows.len());
        q.push_back(((x, y), 0));
        while let Some(((x, y), dist)) = q.pop_front() {
            if seen.contains(&(x, y)) {
                continue;
            }
            seen.insert((x, y));
            if b'E' == self.grid.get(x, y).context("visiting")? {
                return Ok(Some(dist));
            }
            for dest in self.adj(x, y).context("traversing")? {
                // This check isn't necesesary
                if !seen.contains(&dest) {
                    q.push_back((dest, dist + 1))
                }
            }
        }
        Ok(None)
    }

    fn shortest_path_among(&self, coords: &[Coord]) -> Result<Option<usize>> {
        // init distance for all pairs
        let mut dist: Vec<Vec<Vec<Vec<_>>>> = self
            .grid
            .rows
            .iter()
            .map(|r| {
                {
                    r.iter()
                        .map(|_| vec![vec![usize::MAX; r.len()]; self.grid.rows.len()])
                }
                .collect()
            })
            .collect();

        let all_coords = self
            .grid
            .coords()
            .map(|(x, y)| Ok((x.try_into()?, y.try_into()?)))
            .collect::<Result<Vec<(usize, usize)>>>()?;

        // give edges len 1
        for (fx, fy) in self.grid.coords() {
            for (tx, ty) in self.adj(fx, fy)? {
                dist[fx as usize][fy as usize][tx as usize][ty as usize] = 1
            }
        }
        // give self len 0
        for (x, y) in all_coords.iter().copied() {
            dist[x][y][x][y] = 0
        }
        // all shortest paths
        for (kx, ky) in all_coords.iter().copied() {
            // print for progress
            println!("{kx}, {ky}");
            for (ix, iy) in all_coords.iter().copied() {
                for (jx, jy) in all_coords.iter().copied() {
                    let new = dist[ix][iy][kx][ky].saturating_add(dist[kx][ky][jx][jy]);
                    let old = &mut dist[ix][iy][jx][jy];
                    if *old > new {
                        *old = new;
                    }
                }
            }
        }

        let (tx, ty) = (self.end.0 as usize, self.end.1 as usize);
        let ret = coords
            .iter()
            .map(|(fx, fy)| {
                let (fx, fy) = (*fx as usize, *fy as usize);
                dist[fx][fy][tx][ty]
            })
            .min();
        Ok(ret)
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
    g.shortest_path(g.start.0, g.start.1)?
        .ok_or_else(|| anyhow!("No path found"))
}

fn part2(input: &str) -> Result<usize> {
    let g: Graph = input.parse::<Grid>()?.try_into()?;
    g.shortest_path_among(&g.grid.find_all(b'a'))?
        .ok_or_else(|| anyhow!("No path found"))
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
        let answer = solve(include_str!("../../data/example/day12.txt")).unwrap();
        assert_eq!(answer, (31, 29));
    }

    // Don't test the challenge input. It's way too slow

    #[test]
    fn test_grid_coords() {
        for (grid_rows, mut want) in [
            (vec![], vec![]),
            (vec![vec![b'a']], vec![(0, 0)]),
            (
                vec![vec![b'a', b'b'], vec![b'c', b'd']],
                vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            ),
        ] {
            let mut got: Vec<_> = Grid {
                rows: grid_rows.clone(),
            }
            .coords()
            .collect();
            got.sort();
            want.sort();
            assert_eq!(got, want, "{grid_rows:?}")
        }
    }

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
}
