pub mod part1 {
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

    #[allow(dead_code)]
    pub fn run() {
        let mut sum = 0;
        let test_ranges_input = include_str!("../inputs/day2.txt");
        //     let test_ranges_input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        // 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        // 824824821-824824827,2121212118-2121212124"#;
        test_ranges_input
            .split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .for_each(|s| {
                let (start, end) = s
                    .split_once("-")
                    .unwrap_or_else(|| panic!("invalid range: '{s}'"));
                print!("range {start}-{end}: ");
                for n in start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap() {
                    if !is_id_valid(&format!("{n}")) {
                        print!(" {n}");
                        sum += n;
                    }
                }
                println!("");
            });
        println!("invalid sum {sum}");
    }

    #[cfg(test)]
    mod test {
        use super::is_id_valid;

        #[test]
        fn test_is_id_valid() {
            assert!(!is_id_valid("1111"));
            assert!(is_id_valid("1121"));
            assert!(!is_id_valid("1212"));
            assert!(is_id_valid("11111"));
            assert!(is_id_valid("12121"));
        }
    }
}

pub mod part2 {
    use itertools::Itertools;

    fn is_id_valid(id: usize) -> bool {
        let num_digits = id.ilog10() + 1;
        for pattern_length in 1..=num_digits / 2 {
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

    #[allow(dead_code)]
    pub fn run() {
        let mut sum = 0;
        let test_ranges_input = include_str!("../inputs/day2.txt");
        // let test_ranges_input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        // 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        // 824824821-824824827,2121212118-2121212124"#;
        test_ranges_input
            .split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .for_each(|s| {
                let (start, end) = s
                    .split_once("-")
                    .unwrap_or_else(|| panic!("invalid range: '{s}'"));
                print!("range {start}-{end}: ");
                for n in start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap() {
                    if !is_id_valid(n) {
                        print!(" {n}");
                        sum += n;
                    }
                }
                println!("");
            });
        println!("invalid sum {sum}");
    }

    #[cfg(test)]
    mod test {
        use crate::day2::part2::is_id_valid;

        #[test]
        fn test_digits() {
            assert_eq!(20usize.ilog10(), 1);
            assert_eq!(2usize.ilog10(), 0);
            assert_eq!(10usize.ilog10(), 1);
            assert_eq!(99usize.ilog10(), 1);
            assert_eq!(100usize.ilog10(), 2);
        }

        #[test]
        fn test_is_int_id_valid() {
            assert!(!is_id_valid(824824824));
            assert!(!is_id_valid(1111));
            assert!(is_id_valid(1121));
            assert!(!is_id_valid(1212));
            assert!(!is_id_valid(11111));
            assert!(is_id_valid(12121));
        }
    }
}
