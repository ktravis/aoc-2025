use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::input_lines;

type Present = [[bool; 3]; 3];

#[derive(Default)]
struct Region {
    width: usize,
    height: usize,
    needs: Vec<usize>,
}

#[derive(Default)]
struct Problem {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

fn parse_present<'a, 'b>(iter: &'a mut impl Iterator<Item = &'b str>) -> Present {
    let mut p = Present::default();
    for (i, s) in iter.enumerate() {
        if s.is_empty() {
            break;
        }
        assert!(i < 3);
        s.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                p[i][j] = true;
            }
        });
    }
    p
}

#[aoc_generator(day12)]
pub fn generate(input: &str) -> Problem {
    let mut problem = Problem::default();
    let mut iter = input_lines(input);
    loop {
        let Some(line) = iter.next().map(str::to_string) else {
            return problem;
        };
        let (pre, post) = line.split_once(":").unwrap();
        if pre.contains("x") {
            let (w, h) = pre.split_once("x").unwrap();
            let width = w.parse::<usize>().unwrap();
            let height = h.parse::<usize>().unwrap();
            problem.regions.push(Region {
                width,
                height,
                needs: post
                    .split(" ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec(),
            });
        } else {
            let present_index = pre.parse::<usize>().unwrap();
            assert_eq!(present_index, problem.presents.len());
            problem.presents.push(parse_present(&mut iter));
        }
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(problem: &Problem) -> usize {
    0
}

#[aoc(day12, part2)]
pub fn solve_part2(problem: &Problem) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&generate(TEST_INPUT)), 50);
        assert_eq!(
            solve_part1(&generate(include_str!("../input/2025/day12.txt"))),
            4769758290
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&generate(TEST_INPUT)), 24);
        assert_eq!(
            solve_part2(&generate(include_str!("../input/2025/day12.txt"))),
            9770311947567
        );
    }
}
