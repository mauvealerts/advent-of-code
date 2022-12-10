// This is O(input) time, O(alphabet) space.
// Keeps track of duplicate count to avoid scanning

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: Option<usize>,
    part2: Option<usize>,
}

fn main() {
    let d = include_str!("../../data/challenge/day06.txt");
    println!("{:#?}", solve(d))
}

#[derive(Default)]
struct LowerMultiSet {
    counts: [u8; 26],
    dupes: u8,
}

impl LowerMultiSet {
    fn add(&mut self, c: u8) {
        debug_assert!(c.is_ascii_lowercase());
        let cnt = &mut self.counts[(c - b'a') as usize];
        if *cnt == 1 {
            self.dupes += 1
        }
        *cnt += 1
    }

    fn add_all(&mut self, s: &[u8]) {
        for c in s {
            self.add(*c)
        }
    }

    fn remove(&mut self, c: u8) {
        debug_assert!(c.is_ascii_lowercase());
        let cnt = &mut self.counts[(c - b'a') as usize];
        if *cnt > 0 {
            *cnt -= 1
        }
        if *cnt == 1 {
            self.dupes -= 1
        }
    }

    fn all_unique(&self) -> bool {
        self.dupes == 0
    }
}

fn find_distinct(input: &str, win_size: usize) -> Option<usize> {
    let mut set = LowerMultiSet::default();
    for (i, w) in input.as_bytes().windows(win_size).enumerate() {
        if i == 0 {
            set.add_all(w)
        } else {
            set.add(*w.last().unwrap())
        }
        if set.all_unique() {
            return Some(i + win_size);
        }
        set.remove(*w.first().unwrap())
    }
    None
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
