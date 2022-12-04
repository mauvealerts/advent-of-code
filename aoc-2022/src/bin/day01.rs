use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    max: u32,
    top3: u32,
}

fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day01.txt");
    println!("{:#?}", solve(d)?);
    Ok(())
}

fn solve(input: &str) -> Result<Answer> {
    let mut acc: u32 = 0;
    let mut elves = vec![];
    for l in input.lines() {
        if !l.is_empty() {
            acc += l.parse::<u32>()?;
            continue;
        }
        elves.push(acc);
        acc = 0;
    }
    elves.push(acc);
    elves.sort();
    elves.reverse();
    let top3: u32 = elves[..3].iter().sum();
    let max = elves.into_iter().max().unwrap_or(0);
    Ok(Answer { max, top3 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let answer = solve(include_str!("../../data/example/day01.txt")).unwrap();
        assert_eq!(
            answer,
            Answer {
                max: 24000,
                top3: 45000
            }
        );
    }
}
