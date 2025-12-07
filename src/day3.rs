use aoc_runner_derive::aoc;
use itertools::Itertools;

use crate::utils::input_lines;

pub fn max_bank_joltage<const C: usize>(bank: &str) -> usize {
    assert!(bank.len() >= C);
    let mut start = 0;
    let digits: [usize; C] = std::array::from_fn(|d: usize| {
        let d = d + 1;
        let (i, m) = bank
            .chars()
            .enumerate()
            .take(bank.len() - C + d) // drop the last (count - d)
            .skip(start) // offset from last max digit
            .max_set_by_key(|(_, c)| *c)
            .into_iter()
            .nth(0)
            .unwrap();
        start = i + 1;
        m.to_digit(10).expect("char is a digit") as usize
    });
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| d * 10usize.pow(i as u32))
        .sum()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    input_lines(input).map(|s| max_bank_joltage::<2>(s)).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    input_lines(input).map(|s| max_bank_joltage::<12>(s)).sum()
}

#[cfg(test)]
mod test {
    use crate::day3::max_bank_joltage;

    const TEST_INPUT: &'static str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
    #[test]
    fn test_max_bank_joltage_part1() {
        let expected = [98, 89, 78, 92];
        for (i, line) in TEST_INPUT.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            println!("{line} -> {}", expected[i]);
            assert_eq!(max_bank_joltage::<2>(line), expected[i]);
        }

        assert_eq!(max_bank_joltage::<2>("991"), 99);
    }
    #[test]
    fn test_max_bank_joltage_part2() {
        let expected = [987654321111, 811111111119, 434234234278, 888911112111];
        for (i, line) in TEST_INPUT.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            println!("{line} -> {}", expected[i]);
            assert_eq!(max_bank_joltage::<12>(line), expected[i]);
        }
    }
}
