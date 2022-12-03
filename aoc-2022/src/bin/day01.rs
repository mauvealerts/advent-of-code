use anyhow::Result;

fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day01.txt");
    println!("{}", solve(d)?);
    Ok(())
}

fn solve(input: &str) -> Result<String> {
    let mut acc: u32 = 0;
    let mut elves = vec![];
    for l in input.lines() {
        if l != "" {
            acc += l.parse::<u32>()?;
            continue;
        }
        elves.push(acc);
        acc = 0;
    }
    elves.push(acc);
    Ok(elves.into_iter().max().unwrap_or(0).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let answer = solve(include_str!("../../data/example/input/day01.txt")).unwrap();
        assert_eq!(answer, "24000");
    }
}
