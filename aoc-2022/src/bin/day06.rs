use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: u32,
    part2: u32,
}

fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day06.txt");
    println!("{:#?}", solve(d)?);
    Ok(())
}

fn part1(_input: &str) -> Result<u32> {
    Ok(0)
}

fn part2(_input: &str) -> Result<u32> {
    Ok(0)
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
        let answer = solve(include_str!("../../data/example/day06.txt")).unwrap();
        assert_eq!(answer, Answer { part1: 0, part2: 0 });
    }
}
