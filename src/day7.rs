use aoc_runner_derive::aoc;
use itertools::Itertools;

use crate::utils::input_lines;

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut lines = input_lines(input);
    let start = lines.by_ref().next().unwrap();
    let mut beams = start.chars().map(|c| c == 'S').collect_vec();
    lines
        .map(|line| {
            assert_eq!(line.len(), start.len());
            line.chars()
                .enumerate()
                .map(|(i, c)| {
                    if c == '^' && beams[i] {
                        beams[i - 1] = true;
                        beams[i] = false;
                        beams[i + 1] = true;
                        1
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut lines = input_lines(input);
    let start = lines.by_ref().next().unwrap();
    lines
        .fold(
            start.chars().map(|c| (c == 'S') as usize).collect_vec(),
            |mut acc, line| {
                assert_eq!(line.len(), start.len());
                line.chars().enumerate().for_each(|(i, c)| {
                    if c == '^' {
                        let n = acc[i];
                        acc[i - 1] += n;
                        acc[i] = 0;
                        acc[i + 1] += n;
                    }
                });
                acc
            },
        )
        .iter()
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 21);
        assert_eq!(solve_part1(include_str!("../input/2025/day7.txt")), 1590);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 40);
        assert_eq!(
            solve_part2(include_str!("../input/2025/day7.txt")),
            20571740188555
        );
    }
}
