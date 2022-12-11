use std::{
    cmp::{max, min},
    str::FromStr,
};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: usize,
    part2: usize,
}

fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day08.txt");
    println!("{:#?}", solve(d)?);
    Ok(())
}

struct Grid {
    rows: Vec<Vec<u8>>,
}

impl Grid {
    fn height(&self) -> usize {
        self.rows.len()
    }

    fn width(&self) -> usize {
        self.rows[0].len()
    }

    fn row_iter(&self, r: usize) -> impl DoubleEndedIterator<Item = (usize, u8)> + '_ {
        self.rows[r].iter().enumerate().map(|(c, t)| (c, *t))
    }

    fn inner_row_iter(&self, r: usize) -> impl DoubleEndedIterator<Item = (usize, u8)> + '_ {
        self.rows[r][1..self.width() - 1]
            .iter()
            .enumerate()
            .map(|(c, t)| (c + 1, *t))
    }

    fn col_iter(&self, c: usize) -> impl DoubleEndedIterator<Item = (usize, u8)> + '_ {
        self.rows
            .iter()
            .enumerate()
            .map(move |(r, row)| (r, row[c]))
    }

    fn inner_col_iter(&self, c: usize) -> impl DoubleEndedIterator<Item = (usize, u8)> + '_ {
        self.rows[1..self.height() - 1]
            .iter()
            .enumerate()
            .map(move |(r, row)| (r + 1, row[c]))
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let rows = s
            .lines()
            .map(|l| l.as_bytes().iter().map(|c| c - b'0').collect())
            .collect();
        Ok(Self { rows })
    }
}

struct VisCount {
    max_height: i16,
    visible: Vec<Vec<bool>>,
}

impl VisCount {
    fn new() -> Self {
        Self {
            max_height: i16::MIN,
            visible: Vec::new(),
        }
    }

    fn reset_line(&mut self) {
        self.max_height = i16::MIN
    }

    fn visit_tree(&mut self, r: usize, c: usize, t: u8) {
        let t = t as i16;
        self.visible[r][c] = self.visible[r][c] || t > self.max_height;
        self.max_height = max(self.max_height, t)
    }

    fn compute(mut self, grid: &Grid) -> usize {
        self.visible = (0..(grid.height()))
            .map(|_| vec![false; grid.width()])
            .collect();

        for r in 0..(grid.height()) {
            self.reset_line();
            for (c, t) in grid.row_iter(r) {
                self.visit_tree(r, c, t);
            }
        }
        for r in 0..(grid.height()) {
            self.reset_line();
            for (c, t) in grid.row_iter(r).rev() {
                self.visit_tree(r, c, t);
            }
        }
        for c in 0..(grid.width()) {
            self.reset_line();
            for (r, t) in grid.col_iter(c) {
                self.visit_tree(r, c, t);
            }
        }
        for c in 0..(grid.width()) {
            self.reset_line();
            for (r, t) in grid.col_iter(c).rev() {
                self.visit_tree(r, c, t);
            }
        }
        self.visible
            .iter()
            .map(|row| row.iter().filter(|v| **v).count())
            .sum()
    }
}

struct ViewScore {
    last_seen: [usize; 10],
    scores: Vec<Vec<usize>>,
}

impl ViewScore {
    fn new() -> Self {
        Self {
            last_seen: Default::default(),
            scores: Vec::new(),
        }
    }

    // note where we last saw a taller tree
    fn spot(&mut self, p: usize, t: u8) {
        for v in 0..=t {
            self.last_seen[v as usize] = p;
        }
    }

    // init_p should be the position just beyond the first tree we visit
    fn reset_line(&mut self, init_p: usize) {
        self.last_seen.fill(init_p)
    }

    fn visit_tree(&mut self, r: usize, c: usize, p: usize, t: u8) {
        // we might be iterating in either direction
        let high = max(p, self.last_seen[t as usize]);
        let low = min(p, self.last_seen[t as usize]);
        self.scores[r][c] *= high - low;

        self.spot(p, t)
    }

    fn compute(mut self, grid: &Grid) -> usize {
        self.scores = (0..(grid.height()))
            .map(|_| vec![1; grid.width()])
            .collect();

        for r in 1..(grid.height() - 1) {
            self.reset_line(0);
            for (c, t) in grid.inner_row_iter(r) {
                self.visit_tree(r, c, c, t);
            }
        }
        for r in 1..(grid.height() - 1) {
            self.reset_line(grid.width() - 1);
            for (c, t) in grid.inner_row_iter(r).rev() {
                self.visit_tree(r, c, c, t);
            }
        }
        for c in 1..(grid.width() - 1) {
            self.reset_line(0);
            for (r, t) in grid.inner_col_iter(c) {
                self.visit_tree(r, c, r, t);
            }
        }
        for c in 1..(grid.width() - 1) {
            self.reset_line(grid.height() - 1);
            for (r, t) in grid.inner_col_iter(c).rev() {
                self.visit_tree(r, c, r, t);
            }
        }

        *self
            .scores
            .iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap()
    }
}

fn part1(input: &str) -> Result<usize> {
    let grid: Grid = input.parse()?;
    Ok(VisCount::new().compute(&grid))
}

fn part2(input: &str) -> Result<usize> {
    let grid: Grid = input.parse()?;
    Ok(ViewScore::new().compute(&grid))
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
        let answer = solve(include_str!("../../data/example/day08.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: 21,
                part2: 8
            }
        );
    }

    #[test]
    fn challenge() {
        let answer = solve(include_str!("../../data/challenge/day08.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: 1803,
                part2: 268912
            }
        );
    }
}
