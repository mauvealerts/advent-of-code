use anyhow::{Context, Result};

fn main() -> Result<()> {
    for s in [
        include_str!("../../data/example/template.txt"),
        include_str!("../../data/challenge/template.txt"),
    ] {
        println!("{:#?}", solve(s)?)
    }
    Ok(())
}

fn part1(_input: &str) -> Result<usize> {
    Ok(0)
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
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
        let answer = solve(include_str!("../../data/example/template.txt")).unwrap();
        assert_eq!(answer, (0, 0));
    }

    #[test]
    fn challenge() {
        let answer = solve(include_str!("../../data/challenge/template.txt")).unwrap();
        assert_eq!(answer, (0, 0));
    }
}
