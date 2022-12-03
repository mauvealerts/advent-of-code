use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: u32,
    part2: u32,
}

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

enum Outcome {
    Lose,
    Draw,
    Win,
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

    fn for_outcome(&self, outcome: Outcome) -> Self {
        match (self, outcome) {
            (Choice::Rock, Outcome::Lose) => Choice::Scissors,
            (Choice::Rock, Outcome::Draw) => Choice::Rock,
            (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Paper, Outcome::Lose) => Choice::Rock,
            (Choice::Paper, Outcome::Draw) => Choice::Paper,
            (Choice::Paper, Outcome::Win) => Choice::Scissors,
            (Choice::Scissors, Outcome::Lose) => Choice::Paper,
            (Choice::Scissors, Outcome::Draw) => Choice::Scissors,
            (Choice::Scissors, Outcome::Win) => Choice::Rock,
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

impl Outcome {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(anyhow!("Unrecognized {}", c))
        }
    }
}

fn solve(input: &str) -> Result<Answer> {
    let mut part1 = 0;
    let mut part2 = 0;
    for (i,l) in input.lines().enumerate() {
        let l: Vec<_> = l.chars().collect(); 
        if l.len() < 3 {
            return Err(anyhow!("line {} length is {}, expected 3", i, l.len()));
        }
        let other = Choice::from_opponent(l[0])?;
        let mine_part1 = Choice::from_self(l[2])?;
        let mine_part2 = other.for_outcome(Outcome::from_char(l[2])?);
        part1 += mine_part1.shape_score() + mine_part1.match_score(&other);
        part2 += mine_part2.shape_score() + mine_part2.match_score(&other);
    }
    Ok(Answer { part1, part2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let answer = solve(include_str!("../../data/example/day02.txt")).unwrap();
        assert_eq!(answer, Answer { part1: 15, part2: 12});
    }
}
