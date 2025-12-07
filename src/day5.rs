use std::ops::RangeInclusive;

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines().into_iter().map(str::trim);
    let fresh_ranges = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let Some((start, end)) = line.split_once("-") else {
                panic!("failed to parse line: {line}");
            };
            RangeInclusive::new(
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .collect_vec();
    lines
        .filter(|line| {
            fresh_ranges
                .iter()
                .find(|r| r.contains(&line.parse::<usize>().unwrap()))
                .is_some()
        })
        .count()
}

trait MergeRange: Sized {
    fn merge_range(&self, other: &Self) -> Option<Self>;
}

impl<T> MergeRange for RangeInclusive<T>
where
    T: Ord + Copy,
{
    fn merge_range(&self, other: &Self) -> Option<Self> {
        if self.contains(other.start()) || other.contains(self.start()) {
            Some(Self::new(
                *self.start().min(other.start()),
                *self.end().max(other.end()),
            ))
        } else {
            None
        }
    }
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let Some((start, end)) = line.split_once("-") else {
                panic!("failed to parse line: {line}");
            };
            RangeInclusive::new(
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .sorted_by_key(|r| *r.start())
        .coalesce(|prev, curr| prev.merge_range(&curr).ok_or((prev, curr)))
        .map(|r| r.end() - r.start() + 1)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 3);
        assert_eq!(solve_part1(include_str!("../input/2025/day5.txt")), 862);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 14);
        assert_eq!(
            solve_part2(include_str!("../input/2025/day5.txt")),
            357907198933892
        );
    }
}
