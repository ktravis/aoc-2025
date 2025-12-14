use std::collections::HashSet;

use aoc_runner_derive::aoc;
use itertools::Itertools;

use crate::utils::input_lines;

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    // input_lines(input).map(|line| {
    //     let parts = line.split(" ");
    //     let light_line = parts.next().unwrap().trim_matches(&['[', ']']);
    //     let lights = light_line
    //         .chars()
    //         .enumerate()
    //         .map(|(i, c)| match c {
    //             '.' => 0,
    //             '#' => 1 << i - 1,
    //         })
    //         .sum();
    //     let buttons = parts.collect_vec();
    //     let _joltage = buttons.pop();
    //     let buttons = buttons
    //         .into_iter()
    //         .map(|b| {
    //             b.trim_matches(&['(', ')'])
    //                 .split(",")
    //                 .map(|s| 1 << s.parse::<usize>().unwrap())
    //                 .sum()
    //         })
    //         .collect_vec();
    //     for i in 0..light_line.len() {
    //         if lights & 1 << i != 0 {
    //
    //         }
    //     }
    // })
    0
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 50);
        assert_eq!(
            solve_part1(include_str!("../input/2025/day10.txt")),
            4769758290
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 24);
        assert_eq!(
            solve_part2(include_str!("../input/2025/day10.txt")),
            9770311947567
        );
    }
}
