// Is it AoC without a bitset somewhere?

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: Option<usize>,
    part2: Option<usize>,
}

fn main() {
    let d = include_str!("../../data/challenge/day06.txt");
    println!("{:#?}", solve(d))
}

fn find_distinct(input: &str, win_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(win_size)
        .position(|w| {
            w.iter()
                .fold(0_u32, |s, c| s | (1 << (c - 'a' as u8)))
                .count_ones() as usize
                == win_size
        })
        .map(|idx| idx + win_size)
}

fn part1(input: &str) -> Option<usize> {
    find_distinct(input, 4)
}

fn part2(input: &str) -> Option<usize> {
    find_distinct(input, 14)
}

fn solve(input: &str) -> Answer {
    let part1 = part1(input);
    let part2 = part2(input);

    Answer { part1, part2 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        for (inp, (part1, part2)) in include_str!("../../data/example/day06.txt").lines().zip([
            (7, 19),
            (5, 23),
            (6, 23),
            (10, 29),
            (11, 26),
        ]) {
            let answer = solve(inp);
            assert_eq!(
                answer,
                Answer {
                    part1: Some(part1),
                    part2: Some(part2)
                },
                "input: {inp:?}"
            );
        }
    }

    #[test]
    fn challenge() {
        let got = solve(include_str!("../../data/challenge/day06.txt"));
        assert_eq!(
            got,
            Answer {
                part1: Some(1275),
                part2: Some(3605)
            }
        )
    }
}
