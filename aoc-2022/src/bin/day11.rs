#![feature(type_alias_impl_trait)]

use anyhow::{anyhow, bail, Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{all_consuming, map, map_res},
    error::convert_error,
    multi::separated_list0,
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    Finish,
};

fn main() -> Result<()> {
    for s in [
        include_str!("../../data/example/day11.txt"),
        include_str!("../../data/challenge/day11.txt"),
    ] {
        println!("{:#?}", solve(s)?)
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rhs {
    Const(u64),
    Old,
}

impl Rhs {
    fn val(&self, old: u64) -> u64 {
        match self {
            Rhs::Const(x) => *x,
            Rhs::Old => old,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Add(Rhs),
    Mult(Rhs),
}

impl Op {
    fn apply(&self, v: u64) -> u64 {
        match self {
            Op::Add(rhs) => v + rhs.val(v),
            Op::Mult(rhs) => v * rhs.val(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    fact: u64,
    t_dest: usize,
    f_dest: usize,
    inspected: usize,
}

impl Monkey {
    fn inspect(&mut self, adjust: impl Fn(u64) -> u64) -> Vec<(usize, u64)> {
        self.inspected += self.items.len();
        self.items
            .drain(..)
            .map(|v| {
                let mut new = self.op.apply(v);
                new = adjust(new);
                let dest = if new % self.fact == 0 {
                    self.t_dest
                } else {
                    self.f_dest
                };
                (dest, new)
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MonkeySim {
    monkeys: Vec<Monkey>,
}

impl MonkeySim {
    fn run_once(&mut self, adjust: impl Fn(u64) -> u64) {
        for i in 0..self.monkeys.len() {
            for (dest, val) in self.monkeys[i].inspect(&adjust) {
                self.monkeys[dest].items.push(val)
            }
        }
    }
}

type IError<'a> = nom::error::VerboseError<&'a str>;
type IResult<'a, T> = nom::IResult<&'a str, T, IError<'a>>;

fn starting_items(input: &str) -> IResult<Vec<u64>> {
    preceded(
        tag("  Starting items: "),
        separated_list0(tag(", "), map_res(digit1, str::parse)),
    )(input)
}

fn rhs(input: &str) -> IResult<Rhs> {
    map_res::<_, _, _, _, anyhow::Error, _, _>(alt((tag("old"), digit1)), |v| match v {
        "old" => Ok(Rhs::Old),
        num => Ok(Rhs::Const(num.parse().context("rhs")?)),
    })(input)
}

fn operation(input: &str) -> IResult<Op> {
    map(
        preceded(
            tag("  Operation: new = old "),
            separated_pair(alt((char('*'), char('+'))), char(' '), rhs),
        ),
        |(op, rhs)| match op {
            '*' => Op::Mult(rhs),
            '+' => Op::Add(rhs),
            _ => unreachable!(),
        },
    )(input)
}

fn test_clause(input: &str) -> IResult<(u64, usize, usize)> {
    map_res::<_, _, _, _, anyhow::Error, _, _>(
        tuple((
            preceded(tag("  Test: divisible by "), digit1::<&str, _>),
            line_ending,
            preceded(tag("    If true: throw to monkey "), digit1),
            line_ending,
            preceded(tag("    If false: throw to monkey "), digit1),
        )),
        |(fact, _, t, _, f)| {
            Ok((
                fact.parse().context("divisor")?,
                t.parse().context("true dest")?,
                f.parse().context("false dest")?,
            ))
        },
    )(input)
}

fn monkey(input: &str) -> IResult<Monkey> {
    map(
        tuple((
            tuple((tag("Monkey "), digit1, char(':'), line_ending)),
            terminated(starting_items, line_ending),
            terminated(operation, line_ending),
            test_clause,
        )),
        |(_, items, op, (fact, t_dest, f_dest))| Monkey {
            items,
            op,
            fact,
            t_dest,
            f_dest,
            inspected: 0,
        },
    )(input)
}

fn monkey_sim(input: &str) -> IResult<MonkeySim> {
    map(
        separated_list0(pair(line_ending, line_ending), monkey),
        |monkeys| MonkeySim { monkeys },
    )(input)
}

fn run_parser<'a, T>(parser: fn(&'a str) -> IResult<T>, input: &'a str) -> Result<T> {
    all_consuming(parser)(input)
        .finish()
        .map(|(_, out)| out)
        .map_err(|e| {
            let msg = convert_error(input, e);
            anyhow!("Parse error: {msg}")
        })
}

fn part1(input: &str) -> Result<usize> {
    let mut sim = run_parser(monkey_sim, input)?;
    for _ in 0..20 {
        sim.run_once(|i| i / 3);
    }
    let mut business: Vec<_> = sim.monkeys.into_iter().map(|m| m.inspected).collect();
    business.sort();
    let [.., m, n] = &business[..] else { bail!("Too few monkeys") };

    Ok(m * n)
}

fn part2(input: &str) -> Result<usize> {
    let mut sim = run_parser(monkey_sim, input)?;
    for i in 1..=10000 {
        sim.run_once(|x| x % 96577);
        if i % 1000 == 0 {
            println!(
                "after {i}; {:?}",
                sim.monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>()
            )
        }
    }
    let mut business: Vec<_> = sim.monkeys.into_iter().map(|m| m.inspected).collect();
    business.sort();
    let [.., m, n] = &business[..] else { bail!("Too few monkeys") };
    Ok(m * n)
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
        let answer = solve(include_str!("../../data/example/day11.txt")).unwrap();
        assert_eq!(answer, (10605, 2713310158));
    }

    // #[test]
    // fn challenge() {
    //     let answer = solve(include_str!("../../data/challenge/day11.txt")).unwrap();
    //     assert_eq!(answer, (0, 0));
    // }

    fn try_parser<'a, T>(parser: fn(&'a str) -> IResult<T>, input: &'a str) -> T {
        run_parser(parser, input)
            .with_context(|| format!("parsing: {input:?}"))
            .unwrap()
    }

    #[test]
    fn test_starting_items() {
        for (input, want) in [
            ("  Starting items: 1", vec![1]),
            ("  Starting items: 23", vec![23]),
            ("  Starting items: 40, 50, 60", vec![40, 50, 60]),
        ] {
            assert_eq!(try_parser(starting_items, input), want, "input: {input:?}")
        }
    }

    #[test]
    fn test_rhs() {
        for (input, want) in [
            ("9", Rhs::Const(9)),
            ("42", Rhs::Const(42)),
            ("old", Rhs::Old),
        ] {
            assert_eq!(try_parser(rhs, input), want, "input: {input:?}")
        }
    }

    #[test]
    fn test_operation() {
        for (input, want) in [
            ("  Operation: new = old + 5", Op::Add(Rhs::Const(5))),
            ("  Operation: new = old * 30", Op::Mult(Rhs::Const(30))),
            ("  Operation: new = old * old", Op::Mult(Rhs::Old)),
        ] {
            assert_eq!(try_parser(operation, input), want, "input: {input:?}")
        }
    }

    #[test]
    fn test_test_clause() {
        for (input, want) in [(
            concat!(
                "  Test: divisible by 17\n",
                "    If true: throw to monkey 0\n",
                "    If false: throw to monkey 1"
            ),
            (17, 0, 1),
        )] {
            assert_eq!(try_parser(test_clause, input), want, "input: {input:?}")
        }
    }

    #[test]
    fn test_monkey() {
        for (input, want) in [(
            concat!(
                "Monkey 3:\n",
                "  Starting items: 74\n",
                "  Operation: new = old + 3\n",
                "  Test: divisible by 17\n",
                "    If true: throw to monkey 0\n",
                "    If false: throw to monkey 1"
            ),
            Monkey {
                items: vec![74],
                op: Op::Add(Rhs::Const(3)),
                fact: 17,
                t_dest: 0,
                f_dest: 1,
                inspected: 0,
            },
        )] {
            assert_eq!(try_parser(monkey, input), want, "input: {input:?}")
        }
    }
}
