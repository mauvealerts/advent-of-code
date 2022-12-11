use std::collections::HashMap;

use anyhow::{anyhow, ensure, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, char, digit1, line_ending},
    combinator::{all_consuming, map, map_res, value},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    Finish,
};

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: usize,
    part2: usize,
}

fn main() -> Result<()> {
    println!(
        "{:#?}",
        solve(include_str!("../../data/challenge/day07.txt"))?
    );
    Ok(())
}
type IError<'a> = nom::error::VerboseError<&'a str>;

type IResult<'a, T> = nom::IResult<&'a str, T, IError<'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum DirEntry {
    File(usize, String),
    Dir(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cmd {
    Down(String),
    Up,
    ToTop,
    Ls(Vec<DirEntry>),
}

fn dir_entry(input: &str) -> IResult<DirEntry> {
    alt((
        map_res(
            tuple((
                digit1::<&str, IError>,
                char(' '),
                take_while(|c: char| c == '.' || c.is_alphanumeric()),
            )),
            |(s, _, n)| Ok::<DirEntry, anyhow::Error>(DirEntry::File(s.parse()?, n.to_owned())),
        ),
        map_res(tuple((tag("dir "), alpha1::<&str, IError>)), |(_, n)| {
            Ok::<DirEntry, anyhow::Error>(DirEntry::Dir(n.to_owned()))
        }),
    ))(input)
}

fn cmd_up(input: &str) -> IResult<Cmd> {
    value(Cmd::Up, tag("cd .."))(input)
}

fn cmd_to_top(input: &str) -> IResult<Cmd> {
    value(Cmd::ToTop, tag("cd /"))(input)
}

fn cmd_down(input: &str) -> IResult<Cmd> {
    map(tuple((tag("cd "), alpha1::<&str, IError>)), |(_, d)| {
        Cmd::Down(d.to_owned())
    })(input)
}

fn cmd_ls(input: &str) -> IResult<Cmd> {
    map(
        tuple((
            tag("ls"),
            line_ending,
            separated_list0(line_ending, dir_entry),
        )),
        |(_, _, e)| Cmd::Ls(e),
    )(input)
}

fn cmd_line(input: &str) -> IResult<Cmd> {
    map(
        tuple((tag("$ "), alt((cmd_up, cmd_to_top, cmd_down, cmd_ls)))),
        |(_, c)| c,
    )(input)
}

fn shell_session(input: &str) -> Result<Vec<Cmd>> {
    all_consuming(separated_list1(line_ending, cmd_line))(input)
        .finish()
        .map(|(_, c)| c)
        .map_err(|e| anyhow!("{e}"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SubDirs(Vec<String>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct DirTree {
    tree: HashMap<String, (usize, SubDirs, bool)>,
}

impl DirTree {
    fn from_session(session: &Vec<Cmd>) -> Result<Self> {
        let mut tree = HashMap::new();
        let mut cur_path = Vec::new();
        for cmd in session {
            match cmd {
                Cmd::ToTop => {
                    cur_path = vec!["".to_owned()];
                    tree.insert("".to_owned(), (0, SubDirs(Vec::new()), false));
                }
                Cmd::Down(d) => {
                    cur_path.push(d.clone());
                    let cur_name = cur_path.join("/");
                    tree.entry(cur_name)
                        .or_insert((0, SubDirs(Vec::new()), false));
                }
                Cmd::Up => {
                    ensure!(cur_path.pop().is_some(), "cd .. but nothing above")
                }
                Cmd::Ls(entries) => {
                    let cur_name = cur_path.join("/");
                    let (total, sub_dirs, visited) = tree
                        .get_mut(&cur_name)
                        .ok_or_else(|| anyhow!("ls in unknown_dir {cur_name:?}"))?;

                    ensure!(!*visited, "already visited {cur_name:?}");
                    *visited = true;

                    for ent in entries {
                        match ent {
                            DirEntry::Dir(sub) => {
                                let sub_name = cur_path.join("/") + "/" + sub;
                                sub_dirs.0.push(sub_name)
                            }
                            DirEntry::File(size, _) => *total += size,
                        }
                    }
                }
            }
        }
        Ok(Self { tree })
    }

    fn trans_totals(&self, root: &str) -> Result<(usize, Vec<usize>)> {
        let mut totals = Vec::new();
        let (mut my_size, subs, _) = self
            .tree
            .get(root)
            .ok_or_else(|| anyhow!("no dir {root:?}"))?;
        for d in subs.0.iter() {
            let (s, sub_totals) = self.trans_totals(d)?;
            my_size += s;
            totals.extend(sub_totals)
        }
        totals.push(my_size);
        Ok((my_size, totals))
    }
}

fn part1(input: &str) -> Result<usize> {
    let sess = shell_session(input)?;
    let tree = DirTree::from_session(&sess)?;
    let (_, totals) = tree.trans_totals("")?;
    Ok(totals.iter().filter(|t| t <= &&100000).sum())
}

fn part2(input: &str) -> Result<usize> {
    let sess = shell_session(input)?;
    let tree = DirTree::from_session(&sess)?;
    let (top_total, totals) = tree.trans_totals("")?;
    let (disk_size, req_size) = (70000000, 30000000);
    let to_free = (top_total + req_size).saturating_sub(disk_size);
    ensure!(to_free > 0, "No need to free");
    Ok(*totals.iter().filter(|s| s > &&to_free).min().unwrap())
}

fn solve(input: &str) -> Result<Answer> {
    let part1 = part1(input)?;
    let part2 = part2(input)?;

    Ok(Answer { part1, part2 })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn example() {
        let answer = solve(include_str!("../../data/example/day07.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: 95437,
                part2: 24933642
            }
        );
    }

    #[test]
    fn challenge() {
        let answer = solve(include_str!("../../data/challenge/day07.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                part1: 1845346,
                part2: 3636703
            }
        );
    }

    #[test]
    fn test_dir_entry() {
        for (input, want) in [
            ("dir a", DirEntry::Dir("a".to_owned())),
            ("123 hey", DirEntry::File(123, "hey".to_string())),
            ("1 hi.txt", DirEntry::File(1, "hi.txt".to_string())),
        ] {
            let (leftover, got) = dir_entry(input)
                .with_context(|| format!("input {input:?}"))
                .unwrap();
            assert_eq!(leftover, "", "leftover input {input:?}");
            assert_eq!(got, want, "wrong out for input {input:?}")
        }
    }
}
