// This is technically O(n) assuming alphabet is constant
// Need nightly for const generic windows()
#![feature(array_windows)]

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    part1: Option<usize>,
    part2: Option<usize>,
}

fn main() {
    let d = include_str!("../../data/challenge/day06.txt");
    println!("{:#?}", solve(d))
}

struct LowerMultiSet {
    counts: [u8; 26],
}

impl LowerMultiSet {
    fn new() -> Self {
        Self { counts: [0; 26] }
    }

    fn count_mut(&mut self, c: u8) -> &mut u8 {
        debug_assert!(c.is_ascii_lowercase());
        unsafe { self.counts.get_unchecked_mut((c - b'a') as usize) }
    }

    fn add(&mut self, c: u8) {
        *self.count_mut(c) += 1
    }

    fn add_all(&mut self, s: &[u8]) {
        for c in s {
            self.add(*c)
        }
    }

    fn remove(&mut self, c: u8) {
        let cnt = self.count_mut(c);
        if *cnt > 0 {
            *cnt -= 1
        }
    }

    fn len(&mut self) -> usize {
        self.counts.iter().filter(|cnt| cnt > &&0).count()
    }
}

fn find_distinct<const N: usize>(input: &str) -> Option<usize> {
    let mut set = LowerMultiSet::new();
    for (i, w) in input.as_bytes().array_windows::<N>().enumerate() {
        if i == 0 {
            set.add_all(w)
        } else {
            set.add(*w.last().unwrap())
        }
        if set.len() == N {
            return Some(i + N);
        }
        set.remove(*w.first().unwrap())
    }
    None
}

fn part1(input: &str) -> Option<usize> {
    find_distinct::<4>(input)
}

fn part2(input: &str) -> Option<usize> {
    find_distinct::<14>(input)
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
