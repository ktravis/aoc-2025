use anyhow::{Result, anyhow, bail};

fn parse_line(line: &str) -> Result<i32> {
    if line.len() < 2 {
        bail!("line was too short: '{}'", line)
    }
    let x: u32 = line[1..].parse().map_err(|e| {
        anyhow!(
            "failed to parse line as integer ('{}', full line '{line}'): {e}",
            &line[1..]
        )
    })?;
    // parse as unsigned, then add the sign based on prefix
    let i = x as i32;
    match line.chars().nth(0).unwrap_or_default() {
        'L' => Ok(-i),
        'R' => Ok(i),
        c @ _ => bail!("unexpected line prefix: {}", c),
    }
}

pub mod part1 {
    use crate::{day1::parse_line, utils::input_lines};

    #[allow(dead_code)]
    pub fn run() -> usize {
        const INPUT: &'static str = include_str!("../inputs/day1.txt");
        let mut pos = 50;
        let mut zero_count = 0;
        input_lines(INPUT).for_each(|line| {
            let l = parse_line(line).expect("failed to parse line");
            pos = (pos + l) % 100;
            if pos == 0 {
                zero_count += 1;
            }
        });
        println!("zeros: {zero_count}");
        zero_count
    }
}

pub mod part2 {
    use crate::day1::parse_line;
    use crate::utils::input_lines;

    pub fn count_crossings(start: i32, delta: i32) -> usize {
        let next = start + delta;
        let mut crossings = (next / 100).abs() as usize;
        if next <= 0 && start != 0 {
            // if we went from > 0 to < 0, that's one crossing also
            crossings += 1;
        }
        crossings
    }

    #[allow(dead_code)]
    pub fn run() -> usize {
        const INPUT: &'static str = include_str!("../inputs/day1.txt");
        let mut pos = 50;
        let mut zero_count = 0;
        input_lines(INPUT).for_each(|line| {
            let l = parse_line(line).expect("failed to parse line");
            if l == 0 {
                return;
            }
            let next = pos + l;
            let n = count_crossings(pos, l);
            if n != 0 {
                zero_count += n;
            }
            pos = next % 100;
            if pos < 0 {
                pos += 100;
            }
        });
        println!("zeros: {zero_count}");
        zero_count
    }

    #[cfg(test)]
    mod test {
        use super::count_crossings;

        #[test]
        fn test_count_crossings() {
            assert_eq!(count_crossings(50, 1000), 10);
            assert_eq!(count_crossings(50, -65), 1);
            assert_eq!(count_crossings(0, -65), 0);
            assert_eq!(count_crossings(1, -1), 1);
            assert_eq!(count_crossings(10, -1), 0);
            assert_eq!(count_crossings(0, -100), 1);
            assert_eq!(count_crossings(0, -150), 1);
            assert_eq!(count_crossings(10, -150), 2);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day1::parse_line;

    #[test]
    fn test_parse_line() {
        assert!(matches!(parse_line(""), Err(_)));
        assert!(matches!(parse_line("L"), Err(_)));
        assert!(matches!(parse_line("x100"), Err(_)));
        assert!(matches!(parse_line("L1"), Ok(-1)));
        assert!(matches!(parse_line("L0"), Ok(0)));
        assert!(matches!(parse_line("R0"), Ok(0)));
        assert!(matches!(parse_line("L-1"), Err(_)));
        assert!(matches!(parse_line("R1100"), Ok(1100)));
        assert!(matches!(parse_line("R1 10 0"), Err(_)));
    }
}
