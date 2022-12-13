use std::cmp::Ordering;

use anyhow::{Context, Result};
use aoc_2022::nom_util::{run_parser, IResult};
use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
};

fn main() -> Result<()> {
    for s in [
        include_str!("../../data/example/day13.txt"),
        include_str!("../../data/challenge/day13.txt"),
    ] {
        println!("{:#?}", solve(s)?)
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum P {
    N(u8),
    L(Vec<P>),
}

fn packet(input: &str) -> IResult<P> {
    alt((
        map_res(digit1, |s: &str| -> Result<P> { Ok(P::N(s.parse()?)) }),
        map(
            delimited(char('['), separated_list0(char(','), packet), char(']')),
            P::L,
        ),
    ))(input)
}

fn packet_pair_list(input: &str) -> IResult<Vec<(P, P)>> {
    separated_list0(
        tuple((line_ending, line_ending)),
        separated_pair(packet, line_ending, packet),
    )(input)
}

fn compare_lists(x: &[P], y: &[P]) -> Ordering {
    for i in 0..x.len() {
        if i >= y.len() {
            return Ordering::Greater;
        }

        let ans = x[i].cmp(&y[i]);
        if ans != Ordering::Equal {
            return ans;
        }
    }
    if x.len() < y.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

impl Ord for P {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (P::N(x), P::N(y)) => x.cmp(y),
            (P::L(x), P::L(y)) => compare_lists(x, y),
            (P::N(x), P::L(y)) => compare_lists(&[P::N(*x)], y),
            (P::L(x), P::N(y)) => compare_lists(x, &[P::N(*y)]),
        }
    }
}

impl PartialOrd for P {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> Result<usize> {
    Ok(run_parser(packet_pair_list, input)?
        .into_iter()
        .enumerate()
        .flat_map(|(i, (a, b))| match a.cmp(&b) {
            Ordering::Less => Some(i + 1),
            Ordering::Greater => None,
            Ordering::Equal => panic!("{} was equal", i + 1),
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let div1 = run_parser(packet, "[[2]]").context("div1")?;
    let div2 = run_parser(packet, "[[6]]").context("div2")?;

    let mut packets: Vec<P> = run_parser(packet_pair_list, input)?
        .into_iter()
        .flat_map(|(x, y)| [x, y])
        .collect();
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();

    let [pos1, pos2] = [&div1, &div2].map(|p| packets.iter().position(|q| p == q).unwrap() + 1);

    Ok(pos1 * pos2)
}

fn solve(input: &str) -> Result<(usize, usize)> {
    let part1 = part1(input).context("part 1")?;
    let part2 = part2(input).context("part 2")?;

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use aoc_2022::nom_util::run_parser;

    use super::*;

    #[test]
    fn example() {
        let answer = solve(include_str!("../../data/example/day13.txt")).unwrap();
        assert_eq!(answer, (13, 140));
    }

    #[test]
    fn challenge() {
        let answer = solve(include_str!("../../data/challenge/day13.txt")).unwrap();
        assert_eq!(answer, (6478, 21922));
    }

    #[test]
    fn test_packet() {
        for (input, want) in [
            ("1", P::N(1)),
            ("[]", P::L(vec![])),
            ("[1]", P::L(vec![P::N(1)])),
            ("[1,2]", P::L(vec![P::N(1), P::N(2)])),
            ("[[]]", P::L(vec![P::L(vec![])])),
            ("[[3]]", P::L(vec![P::L(vec![P::N(3)])])),
            (
                "[4,[5],6]",
                P::L(vec![P::N(4), P::L(vec![P::N(5)]), P::N(6)]),
            ),
            (
                "[[4,[5]]]",
                P::L(vec![P::L(vec![P::N(4), P::L(vec![P::N(5)])])]),
            ),
        ] {
            assert_eq!(
                run_parser(packet, input)
                    .with_context(|| format!("{input:?}"))
                    .unwrap(),
                want,
                "{input:?}"
            )
        }
    }

    #[test]
    fn test_packet_pair_list() {
        for (input, want) in [
            ("", vec![]),
            ("[]\n[]", vec![(P::L(vec![]), P::L(vec![]))]),
            (
                "[]\n[]\n\n[]\n[]",
                vec![(P::L(vec![]), P::L(vec![])), (P::L(vec![]), P::L(vec![]))],
            ),
        ] {
            assert_eq!(
                run_parser(packet_pair_list, input)
                    .with_context(|| format!("{input:?}"))
                    .unwrap(),
                want,
                "{input:?}"
            )
        }
    }
}
