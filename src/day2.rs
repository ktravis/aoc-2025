use aoc_runner_derive::aoc;
use itertools::Itertools;

fn is_id_valid(id: &str) -> bool {
    if id.len() == 0 {}
    match id.len() {
        0 => true,
        n if n % 2 == 1 => true,
        n => {
            let (left, right) = id.split_at(n / 2);
            left != right
        }
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut sum = 0;
    //     let test_ranges_input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    // 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    // 824824821-824824827,2121212118-2121212124"#;
    input
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .for_each(|s| {
            let (start, end) = s
                .split_once("-")
                .unwrap_or_else(|| panic!("invalid range: '{s}'"));
            for n in start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap() {
                if !is_id_valid(&format!("{n}")) {
                    sum += n;
                }
            }
        });
    sum
}

#[cfg(test)]
mod test {
    use crate::day2::is_id_valid_part2;

    use super::is_id_valid;

    #[test]
    fn test_is_id_valid() {
        assert!(!is_id_valid("1111"));
        assert!(is_id_valid("1121"));
        assert!(!is_id_valid("1212"));
        assert!(is_id_valid("11111"));
        assert!(is_id_valid("12121"));
    }

    #[test]
    fn test_is_id_valid_part2() {
        assert!(!is_id_valid_part2(824824824));
        assert!(!is_id_valid_part2(1111));
        assert!(is_id_valid_part2(1121));
        assert!(!is_id_valid_part2(1212));
        assert!(!is_id_valid_part2(11111));
        assert!(is_id_valid_part2(12121));
    }
}

pub fn is_id_valid_part2(id: usize) -> bool {
    let num_digits = id.ilog10() + 1;
    for pattern_length in (1..=num_digits / 2).rev() {
        if num_digits % pattern_length != 0 {
            continue;
        }
        if (1..=num_digits / pattern_length)
            .map(|rep| {
                let scale = 10usize.pow(pattern_length);
                let offset = scale.pow(rep - 1);
                (id / offset) % scale
            })
            .all_equal()
        {
            return false;
        }
    }
    true
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut sum = 0;
    // let test_ranges_input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    // 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    // 824824821-824824827,2121212118-2121212124"#;
    input
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .for_each(|s| {
            let (start, end) = s
                .split_once("-")
                .unwrap_or_else(|| panic!("invalid range: '{s}'"));
            for n in start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap() {
                if !is_id_valid_part2(n) {
                    sum += n;
                }
            }
        });
    sum
}
