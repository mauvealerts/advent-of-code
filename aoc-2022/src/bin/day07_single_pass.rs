use anyhow::{anyhow, ensure, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, char, digit1, line_ending},
    combinator::{all_consuming, eof, map, map_res, value},
    error::convert_error,
    multi::many0,
    sequence::{separated_pair, terminated, tuple},
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
        solve(include_str!("../../data/example/day07.txt"))?
    );
    Ok(())
}

type IError<'a> = nom::error::VerboseError<&'a str>;
type IResult<'a, T> = nom::IResult<&'a str, T, IError<'a>>;

fn dir_entry(input: &str) -> IResult<Option<usize>> {
    alt((
        map_res(
            separated_pair(
                digit1::<_, IError>,
                char(' '),
                take_while(|c: char| c == '.' || c.is_alphanumeric()),
            ),
            |(s, _)| Ok::<_, anyhow::Error>(Some(s.parse()?)),
        ),
        value(None, tuple((tag("dir "), alpha1))),
    ))(input)
}

fn line_end_or_eof(input: &str) -> IResult<&str> {
    alt((line_ending, eof))(input)
}

fn list_dir(input: &str) -> IResult<usize> {
    map(
        tuple((
            tag("$ ls"),
            line_end_or_eof,
            many0(tuple((dir_entry, line_end_or_eof))),
        )),
        |(_, _, e)| e.iter().filter_map(|(s, _)| s.as_ref()).sum(),
    )(input)
}

fn visit_dir(input: &str) -> IResult<Vec<usize>> {
    map(
        tuple((
            tuple((tag("$ cd "), alt((alpha1, tag("/"))), line_ending)),
            list_dir,
            many0(visit_dir),
            alt((terminated(tag("$ cd .."), line_end_or_eof), eof)),
        )),
        |(_, size, visits, _)| total_dir(size, visits),
    )(input)
}

fn parse_session(input: &str) -> Result<Vec<usize>> {
    let (_, ret) = all_consuming(visit_dir)(input)
        .finish()
        .map_err(|e| anyhow!("{}", convert_error(input, e)))?;
    Ok(ret)
}

fn total_dir(size: usize, visits: Vec<Vec<usize>>) -> Vec<usize> {
    let total = size + visits.iter().map(|v| v.last().unwrap_or(&0)).sum::<usize>();
    let mut dirs: Vec<_> = visits.into_iter().flatten().collect();
    dirs.push(total);
    dirs
}

fn part1(input: &str) -> Result<usize> {
    let dirs = parse_session(input)?;
    Ok(dirs.iter().filter(|t| t <= &&100000).sum())
}

fn part2(input: &str) -> Result<usize> {
    let dirs = parse_session(input)?;
    let (disk_size, req_size) = (70000000, 30000000);
    let to_free = (dirs.last().unwrap() + req_size).saturating_sub(disk_size);
    ensure!(to_free > 0, "No need to free");
    Ok(*dirs.iter().filter(|s| s > &&to_free).min().unwrap())
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

    fn run_parser<'a, T>(parser: fn(&'a str) -> IResult<T>, input: &'a str) -> T {
        parser(input)
            .finish()
            .map(|(left, v)| {
                assert_eq!(left, "");
                v
            })
            .map_err(|e| {
                let msg = convert_error(input, e);
                eprintln!("{msg}");
                msg
            })
            .unwrap()
    }

    #[test]
    fn test_list_dir() {
        let input = "$ ls
dir w
dir x
1 a
dir y
10 b.txt
100 c.txt
dir z
";
        assert_eq!(run_parser(list_dir, input), 111, "{input}");

        let input = "$ ls
";
        assert_eq!(run_parser(list_dir, input), 0, "{input}");

        let input = "$ ls";
        assert_eq!(run_parser(list_dir, input), 0, "{input}");
    }

    #[test]
    fn test_visit_simple() {
        let input = "$ cd foo
$ ls";
        assert_eq!(run_parser(visit_dir, input), vec![0], "{input}");

        let input = "$ cd foo
$ ls
999 a.txt
dir hi";
        assert_eq!(run_parser(visit_dir, input), vec![999], "{input}");

        let input = "$ cd foo
$ ls
999 a.txt
dir hi
$ cd bar
$ ls
10 something";
        assert_eq!(run_parser(visit_dir, input), vec![10, 1009], "{input}");

        let input = "$ cd top
$ ls
7 what
$ cd midone
$ ls
10 something
$ cd deep
$ ls
100 something
$ cd ..
$ cd ..
$ cd midtwo
$ ls
2 something";
        assert_eq!(
            run_parser(visit_dir, input),
            vec![100, 110, 2, 119],
            "{input}"
        );
    }
}
