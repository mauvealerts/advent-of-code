// Need nightly for const generic windows()
#![feature(array_windows)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::{BTreeSet, HashSet};

pub fn find_distinct_bitset(input: &str, win_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(win_size)
        .position(|w| {
            w.iter()
                .fold(0_u32, |s, c| s | (1 << (c - b'a')))
                .count_ones() as usize
                == win_size
        })
        .map(|idx| idx + win_size)
}

pub fn find_distinct_slow_terse_hash(input: &str, win_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(win_size)
        .position(|w| win_size == w.iter().collect::<HashSet<_>>().len())
        .map(|i| i + win_size)
}

pub fn find_distinct_slow_terse_btree(input: &str, win_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(win_size)
        .position(|w| win_size == w.iter().collect::<BTreeSet<_>>().len())
        .map(|i| i + win_size)
}

pub fn find_distinct_slow(input: &str, win_size: usize) -> Option<usize> {
    if let Some(idx) = input
        .chars()
        .collect::<Vec<_>>()
        .windows(win_size)
        .position(|w| {
            for i in 0..w.len() {
                if w[..i].iter().chain(w[i + 1..].iter()).any(|c| c == &w[i]) {
                    return false;
                }
            }
            true
        })
    {
        Some(idx + win_size)
    } else {
        None
    }
}

pub fn find_distinct_fewer_cmp(input: &str, win_size: usize) -> Option<usize> {
    let input: Vec<_> = input.chars().collect();
    let mut idx = 0;
    'outer: while idx <= input.len() - win_size {
        let win = &input[idx..idx + win_size];
        for (i, c) in win.iter().enumerate().rev() {
            if win[i + 1..].iter().any(|d| d == c) {
                idx = idx + i + 1;
                continue 'outer;
            }
        }
        return Some(idx + win_size);
    }
    None
}

pub fn find_distinct_linear_bounds(input: &str, win_size: usize) -> Option<usize> {
    let mut set = LowerMultiSetBounds::new();
    for (i, w) in input.as_bytes().windows(win_size).enumerate() {
        if i == 0 {
            set.add_all(w)
        } else {
            set.add(*w.last().unwrap())
        }
        if set.len() == win_size {
            return Some(i + win_size);
        }
        set.remove(*w.first().unwrap())
    }
    None
}

pub fn find_distinct_linear_unchecked(input: &str, win_size: usize) -> Option<usize> {
    let mut set = LowerMultiSetUnchecked::new();
    for (i, w) in input.as_bytes().windows(win_size).enumerate() {
        if i == 0 {
            set.add_all(w)
        } else {
            set.add(*w.last().unwrap())
        }
        if set.len() == win_size {
            return Some(i + win_size);
        }
        set.remove(*w.first().unwrap())
    }
    None
}

pub fn find_distinct_linear_const_bounds<const N: usize>(input: &str) -> Option<usize> {
    let mut set = LowerMultiSetBounds::new();
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

pub fn find_distinct_linear_const_unchecked<const N: usize>(input: &str) -> Option<usize> {
    let mut set = LowerMultiSetUnchecked::new();
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

struct LowerMultiSetBounds {
    counts: [u8; 26],
}

impl LowerMultiSetBounds {
    fn new() -> Self {
        Self { counts: [0; 26] }
    }

    fn add(&mut self, c: u8) {
        debug_assert!(c.is_ascii_lowercase());
        self.counts[(c - b'a') as usize] += 1
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
    }

    fn len(&mut self) -> usize {
        self.counts.iter().filter(|cnt| cnt > &&0).count()
    }
}

struct LowerMultiSetUnchecked {
    counts: [u8; 26],
}

impl LowerMultiSetUnchecked {
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

fn bench_stuff(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_distinct");
    let data = include_str!("../data/challenge/day06.txt");
    for wsize in [4, 14].iter() {
        group.bench_with_input(BenchmarkId::new("bitset", wsize), wsize, |b, i| {
            b.iter(|| find_distinct_bitset(black_box(data), *i))
        });
        group.bench_with_input(BenchmarkId::new("slow terse hash", wsize), wsize, |b, i| {
            b.iter(|| find_distinct_slow_terse_hash(black_box(data), *i))
        });
        group.bench_with_input(
            BenchmarkId::new("slow terse btree", wsize),
            wsize,
            |b, i| b.iter(|| find_distinct_slow_terse_btree(black_box(data), *i)),
        );
        group.bench_with_input(BenchmarkId::new("slow", wsize), wsize, |b, i| {
            b.iter(|| find_distinct_slow(black_box(data), *i))
        });
        group.bench_with_input(BenchmarkId::new("fewer cmp", wsize), wsize, |b, i| {
            b.iter(|| find_distinct_fewer_cmp(black_box(data), *i))
        });
        group.bench_with_input(BenchmarkId::new("linear bounds", wsize), wsize, |b, i| {
            b.iter(|| find_distinct_linear_bounds(black_box(data), *i))
        });
        group.bench_with_input(
            BenchmarkId::new("linear unchecked", wsize),
            wsize,
            |b, i| b.iter(|| find_distinct_linear_unchecked(black_box(data), *i)),
        );
    }
    group.bench_function(BenchmarkId::new("const linear bounds", 4), |b| {
        b.iter(|| find_distinct_linear_const_bounds::<4>(black_box(data)))
    });
    group.bench_function(BenchmarkId::new("const linear bounds", 14), |b| {
        b.iter(|| find_distinct_linear_const_bounds::<14>(black_box(data)))
    });
    group.bench_function(BenchmarkId::new("const linear unchecked", 4), |b| {
        b.iter(|| find_distinct_linear_const_unchecked::<4>(black_box(data)))
    });
    group.bench_function(BenchmarkId::new("const linear unchecked", 14), |b| {
        b.iter(|| find_distinct_linear_const_unchecked::<14>(black_box(data)))
    });
}

criterion_group!(benches, bench_stuff);
criterion_main!(benches);
