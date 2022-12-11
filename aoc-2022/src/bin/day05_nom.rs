use std::cmp::max;
use std::str::FromStr;

use anyhow::{anyhow, bail, ensure, Context, Result};
use nom::branch::alt;
use nom::character::complete::{alpha1, char as nom_char, digit1, line_ending, multispace0};
use nom::combinator::{all_consuming, map, map_res, value};
use nom::error::{convert_error, VerboseError};
use nom::multi::{count, separated_list1};
use nom::Finish;
use nom::{bytes::complete::tag, sequence::tuple};

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

#[derive(Debug, PartialEq, Eq)]
struct Move {
    count: usize,
    src: usize,
    dest: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Crates {
    columns: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq)]
struct CrateLine {
    line: Vec<Option<char>>,
}

impl Move {
    fn from_parsed((count, src, dest): (usize, usize, usize)) -> Result<Self> {
        ensure!(count > 0, "count was {count}, must be positive");
        ensure!(src > 0, "source was {src}, must be positive");
        ensure!(dest > 0, "dest was {dest}, must be positive");
        let (src, dest) = (src - 1, dest - 1);
        Ok(Move { count, src, dest })
    }
}

impl Crates {
    fn from_crate_lines(lines: Vec<CrateLine>) -> Result<Self> {
        let expected_len = lines[0].line.len();
        for (i, l) in lines.iter().enumerate() {
            if l.line.len() != expected_len {
                bail!(
                    "Crate line {i} had {} entries, expected {expected_len}",
                    l.line.len()
                );
            }
        }
        let mut columns: Vec<Vec<char>> = (0..expected_len)
            .map(|_| Vec::with_capacity(lines.len()))
            .collect();
        for l in lines.iter().rev() {
            for (i, a_crate) in l.line.iter().enumerate() {
                if let Some(ch) = a_crate {
                    columns[i].push(*ch)
                }
            }
        }
        Ok(Self { columns })
    }

    fn check_bounds(&self, m: &Move) -> Result<()> {
        ensure!(
            (0..self.columns.len()).contains(&m.src),
            "source was {}, expected within [0, {})",
            m.src,
            self.columns.len()
        );
        ensure!(
            (0..self.columns.len()).contains(&m.dest),
            "destination was {}, expected within [0, {})",
            m.dest,
            self.columns.len()
        );
        ensure!(
            m.count <= self.columns[m.src].len(),
            "count is {} but column only has {}",
            m.count,
            self.columns[m.src].len()
        );
        Ok(())
    }

    fn run_move(&mut self, m: &Move, mover: Mover) {
        if m.src == m.dest {
            return;
        }
        let (a, b) = self.columns.split_at_mut(max(m.src, m.dest));
        let (src, dest) = if m.src < m.dest {
            (&mut a[m.src], &mut b[0])
        } else {
            (&mut b[0], &mut a[m.dest])
        };
        mover(src, dest, m.count)
    }
}

type Mover = fn(&mut Vec<char>, &mut Vec<char>, usize);

fn move_slow(src: &mut Vec<char>, dest: &mut Vec<char>, count: usize) {
    dest.extend(src.drain((src.len() - count)..).rev());
}

fn move_fast(src: &mut Vec<char>, dest: &mut Vec<char>, count: usize) {
    dest.extend(src.drain((src.len() - count)..));
}

type IResult<'a, T> = nom::IResult<&'a str, T, VerboseError<&'a str>>;

fn a_crate(input: &str) -> IResult<Option<char>> {
    alt((
        map(
            tuple((nom_char::<&str, _>('['), alpha1, nom_char(']'))),
            |(_, items, _)| Some(items.chars().next().unwrap()),
        ),
        value(None, count(nom_char(' '), 3)),
    ))(input)
}

fn crate_line(input: &str) -> IResult<CrateLine> {
    let (input, line) = separated_list1(nom_char(' '), a_crate)(input)?;
    Ok((input, CrateLine { line }))
}

fn label_line(input: &str) -> IResult<()> {
    let (input, _) =
        separated_list1(nom_char(' '), tuple((nom_char(' '), digit1, nom_char(' '))))(input)?;
    Ok((input, ()))
}

fn crates_section(input: &str) -> IResult<Crates> {
    let (input, (crates, _, _, _)) = tuple((
        map_res(
            separated_list1(line_ending, crate_line),
            Crates::from_crate_lines,
        ),
        line_ending,
        label_line,
        line_ending,
    ))(input)?;
    Ok((input, crates))
}

fn a_move(input: &str) -> IResult<(usize, usize, usize)> {
    let (input, (_, count, _, src, _, dest)) = tuple((
        tag("move "),
        map_res(digit1, usize::from_str),
        tag(" from "),
        map_res(digit1, usize::from_str),
        tag(" to "),
        map_res(digit1, usize::from_str),
    ))(input)?;
    Ok((input, (count, src, dest)))
}

fn parse_input(input: &str) -> IResult<(Crates, Vec<Move>)> {
    let (input, (crates, _, moves, _)) = all_consuming(tuple((
        crates_section,
        line_ending,
        separated_list1(line_ending, map_res(a_move, Move::from_parsed)),
        multispace0,
    )))(input)?;
    Ok((input, (crates, moves)))
}

fn simulate(input: &str, mover: Mover) -> Result<String> {
    // Force stringification of error to avoid pain of lifetime
    let (_, (mut crates, moves)) = parse_input(input)
        .finish()
        .map_err(|e| anyhow!("Parse error: {}", convert_error(input, e)))?;

    for (i, m) in moves.iter().enumerate() {
        crates
            .check_bounds(m)
            .with_context(|| format!("Move {i}"))?;
        crates.run_move(m, mover);
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
    simulate(input, move_slow)
}

fn part2(input: &str) -> Result<String> {
    simulate(input, move_fast)
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

    #[test]
    fn challenge() {
        let answer = solve(include_str!("../../data/challenge/day05.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: "QNHWJVJZW".to_owned(),
                part2: "BPCZJLFJW".to_owned()
            }
        );
    }

    #[test]
    fn test_parse_crates() {
        let (leftover, ch) = a_crate("[a]").unwrap();
        assert_eq!(leftover, "");
        assert_eq!(ch, Some('a'));

        let (leftover, ch) = a_crate("   ").unwrap();
        assert_eq!(leftover, "");
        assert_eq!(ch, None);

        let (leftover, line) = crate_line("[a]").unwrap();
        assert_eq!(leftover, "");
        assert_eq!(
            line,
            CrateLine {
                line: vec![Some('a')]
            }
        );

        let (leftover, line) = crate_line("    [a] [b]").unwrap();
        assert_eq!(leftover, "");
        assert_eq!(
            line,
            CrateLine {
                line: vec![None, Some('a'), Some('b')]
            }
        );
    }

    #[test]
    fn test_parse_labels() {
        let (leftover, ()) = label_line(" 1 ").unwrap();
        assert_eq!(leftover, "");

        let (leftover, ()) = label_line(" 1   2 ").unwrap();
        assert_eq!(leftover, "");
    }

    #[test]
    fn test_parse_moves() {
        let (leftover, val) = a_move("move 1 from 2 to 3").unwrap();
        assert_eq!(leftover, "");
        assert_eq!(val, (1, 2, 3));

        let (leftover, val) = a_move("move 10 from 8 to 9").unwrap();
        assert_eq!(leftover, "");
        assert_eq!(val, (10, 8, 9));
    }

    #[test]
    fn test_parse_crates_section() {
        let (leftover, crates) = crates_section(
            "[a]
 1 
",
        )
        .unwrap();
        assert_eq!(leftover, "");
        assert_eq!(
            crates,
            Crates {
                columns: vec![vec!['a']]
            }
        );
    }

    #[test]
    fn test_parse_input() {
        let (leftover, (crates, moves)) = parse_input(
            "[a]
 1 

move 1 from 1 to 1",
        )
        .unwrap();
        assert_eq!(leftover, "");
        assert_eq!(
            crates,
            Crates {
                columns: vec![vec!['a']]
            }
        );
        assert_eq!(
            moves,
            vec![Move {
                count: 1,
                src: 0,
                dest: 0
            }]
        );
    }
}
