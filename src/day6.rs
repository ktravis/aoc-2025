use aoc_runner_derive::aoc;
use itertools::Itertools;

use crate::utils::input_lines;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut lines = input_lines(input);
    let mut first = lines
        .by_ref()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| {
            let n = s.parse::<usize>().unwrap();
            (n, n)
        })
        .collect_vec();

    let mut total = 0;
    lines.for_each(|line| {
        line.split_whitespace().enumerate().for_each(|(i, s)| {
            let a = &mut first[i];
            match s {
                "+" => total += a.0,
                "*" => total += a.1,
                _ => {
                    let n = s.parse::<usize>().unwrap();
                    a.0 += n;
                    a.1 *= n;
                }
            }
        })
    });
    total
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut lines = input
        .lines()
        .map(|line| line.trim_end_matches("\n"))
        .peekable();
    let mut nums = vec![None::<usize>; lines.peek().unwrap().len()];
    for line in lines {
        if line.contains("+") || line.contains("*") {
            let mut total = 0;
            let mut partial = vec![];
            for (c, num) in line.chars().rev().zip(nums) {
                if let Some(n) = num {
                    partial.push(n);
                }
                match c {
                    '+' => total += partial.drain(..).sum::<usize>(),
                    '*' => total += partial.drain(..).product::<usize>(),
                    _ => {}
                }
            }
            return total;
        }
        for (i, c) in line.chars().rev().enumerate() {
            if c >= '0' && c <= '9' {
                let n = c.to_digit(10).unwrap() as usize;
                nums[i] = Some(nums[i].unwrap_or(0) * 10 + n);
            }
        }
    }
    panic!("found no operator line")
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 4277556);
        assert_eq!(
            solve_part1(include_str!("../input/2025/day6.txt")),
            6891729672676
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 3263827);
        assert_eq!(
            solve_part2(include_str!("../input/2025/day6.txt")),
            9770311947567
        );
    }
}
