use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let d = include_str!("../../data/challenge/day02.txt");
    println!("{:#?}", solve(d)?);
    Ok(())
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_opponent(c: char) -> Result<Self> {
        match c {
            'A' => Ok(Choice::Rock),
            'B' => Ok(Choice::Paper),
            'C' => Ok(Choice::Scissors),
            _ => Err(anyhow!("Unrecognized {}", c))
        }
    }

    fn from_self(c: char) -> Result<Self> {
        match c {
            'X' => Ok(Choice::Rock),
            'Y' => Ok(Choice::Paper),
            'Z' => Ok(Choice::Scissors),
            _ => Err(anyhow!("Unrecognized {}", c))
        }
    }

    fn shape_score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn match_score(&self, other: &Choice) -> u32 {
        match (self, other) {
            (Choice::Rock, Choice::Rock) => 3,
            (Choice::Rock, Choice::Paper) => 0,
            (Choice::Rock, Choice::Scissors) => 6,
            (Choice::Paper, Choice::Rock) => 6,
            (Choice::Paper, Choice::Paper) => 3,
            (Choice::Paper, Choice::Scissors) => 0,
            (Choice::Scissors, Choice::Rock) => 0,
            (Choice::Scissors, Choice::Paper) => 6,
            (Choice::Scissors, Choice::Scissors) => 3,
        }
    }
}

fn solve(input: &str) -> Result<u32> {
    let mut total = 0_u32;
    for (i,l) in input.lines().enumerate() {
        let l: Vec<_> = l.chars().collect(); 
        if l.len() < 3 {
            return Err(anyhow!("line {} length is {}, expected 3", i, l.len()));
        }
        let other = Choice::from_opponent(l[0])?;
        let mine = Choice::from_self(l[2])?;
        total += mine.shape_score() + mine.match_score(&other);
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let answer = solve(include_str!("../../data/example/day02.txt")).unwrap();
        assert_eq!(answer, 15);
    }
}
