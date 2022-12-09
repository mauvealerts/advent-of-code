use std::cmp::max;

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

type Grid = Vec<Vec<i8>>;

fn trees(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| (c - b'0') as i8).collect())
        .collect()
}

fn vis_corner(
    trees: &Grid,
    vis: &mut [Vec<bool>],
    row_nums: impl Iterator<Item = usize> + Clone,
    col_nums: impl Iterator<Item = usize> + Clone,
) {
    let mut cols = vec![i8::MIN; trees[0].len()];
    for r in row_nums {
        let mut r_max = i8::MIN;
        for c in col_nums.clone() {
            let t = trees[r][c];
            vis[r][c] = vis[r][c] || t > cols[c] || t > r_max;
            cols[c] = max(t, cols[c]);
            r_max = max(t, r_max);
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let trees = trees(input);
    let num_rows = trees.len();
    let num_cols = trees[0].len();

    let mut vis: Vec<_> = trees.iter().map(|r| vec![false; r.len()]).collect();
    vis_corner(&trees, &mut vis, 0..num_rows, 0..num_cols);
    vis_corner(&trees, &mut vis, (0..num_rows).rev(), (0..num_cols).rev());
    Ok(vis.iter().map(|r| r.iter().filter(|v| **v).count()).sum())
}

fn part2(input: &str) -> Result<usize> {
    let trees = trees(input);
    let r_max = trees.len() - 1;
    let c_max = trees[0].len() - 1;

    let mut scores: Vec<_> = trees.iter().map(|r| vec![0_usize; r.len()]).collect();

    let mut r_last: Vec<_> = (0..=c_max).map(|_| vec![0_usize; 10]).collect();
    for r in 1..r_max {
        let mut c_last = vec![0; 10];

        for (c, cur_last) in r_last.iter_mut().enumerate().take(c_max).skip(1) {
            let t = trees[r][c] as usize;
            scores[r][c] = (r - cur_last[t]) * (c - c_last[t]);

            for v in 0..=t {
                c_last[v] = c;
                cur_last[v] = r;
            }
        }
    }

    let mut r_last: Vec<_> = (0..=c_max).map(|_| vec![r_max; 10]).collect();
    for r in (1..r_max).rev() {
        let mut c_last = vec![c_max; 10];

        for c in (1..r_max).rev() {
            let t = trees[r][c] as usize;
            scores[r][c] *= (r_last[c][t] - r) * (c_last[t] - c);

            for (v, cur_last) in c_last.iter_mut().enumerate().take(t + 1) {
                *cur_last = c;
                r_last[c][v] = r;
            }
        }
    }

    Ok(*scores
        .iter()
        .map(|r| r.iter().max().unwrap())
        .max()
        .unwrap())
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
}
