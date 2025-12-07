use aoc_runner_derive::aoc;
use itertools::Itertools;

use crate::utils::input_lines;

struct Grid {
    pub(crate) width: usize,
    pub(crate) cells: Vec<usize>,
}

impl Grid {
    pub fn incr_space(&mut self, x: i32, y: i32) {
        if x < 0 || x as usize >= self.width || y < 0 {
            return;
        }
        let index = (x + self.width as i32 * y) as usize;
        if index >= self.cells.len() {
            self.cells.extend_from_slice(&vec![0; self.width]);
        }
        self.cells[index] += 1;
    }
    pub fn decr_space(&mut self, x: i32, y: i32) {
        if x < 0 || x as usize >= self.width || y < 0 {
            return;
        }
        let index = (x + self.width as i32 * y) as usize;
        if index >= self.cells.len() {
            self.cells.extend_from_slice(&vec![0; self.width]);
        }
        self.cells[index] = self.cells[index].saturating_sub(1);
    }

    pub fn incr_surrounding_spaces(&mut self, x: usize, y: usize) {
        let x = x as i32;
        let y = y as i32;
        for dx in x - 1..=x + 1 {
            for dy in y - 1..=y + 1 {
                if dx == x && dy == y {
                    continue;
                }
                self.incr_space(dx, dy);
            }
        }
    }

    pub fn decr_surrounding_spaces(&mut self, x: usize, y: usize) {
        let x = x as i32;
        let y = y as i32;
        for dx in x - 1..=x + 1 {
            for dy in y - 1..=y + 1 {
                if dx == x && dy == y {
                    continue;
                }
                self.decr_space(dx, dy);
            }
        }
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut lines = input_lines(input).peekable();
    let mut grid = Grid {
        width: lines
            .peek()
            .expect("input contains at least one line")
            .len(),
        cells: vec![],
    };
    let occupied = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => false,
                    '@' => {
                        grid.incr_surrounding_spaces(x, y);
                        true
                    }
                    _ => unreachable!("invalid character in input"),
                })
                .collect_vec()
        })
        .collect_vec();
    occupied
        .into_iter()
        .zip(grid.cells.into_iter())
        .filter_map(|(pres, surr)| (pres && surr < 4).then_some(1))
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut lines = input_lines(input).peekable();
    let mut grid = Grid {
        width: lines
            .peek()
            .expect("input contains at least one line")
            .len(),
        cells: vec![],
    };
    let mut occupied = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => false,
                    '@' => {
                        grid.incr_surrounding_spaces(x, y);
                        true
                    }
                    _ => unreachable!("invalid character in input"),
                })
                .collect_vec()
        })
        .collect_vec();
    let mut total = 0;
    loop {
        let mut removed = 0;
        for i in 0..occupied.len() {
            if occupied[i] && grid.cells[i] < 4 {
                removed += 1;
                occupied[i] = false;
                grid.decr_surrounding_spaces(i % grid.width, i / grid.width);
            }
        }
        total += removed;
        if removed == 0 {
            break;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 43);
    }
}
