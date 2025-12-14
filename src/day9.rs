use aoc_runner_derive::aoc;
use itertools::Itertools;

use crate::utils::input_lines;

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> usize {
    input_lines(input)
        .filter_map(|line| {
            line.split_once(',')
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        })
        .permutations(2)
        .map(|v| (v[0].0.abs_diff(v[1].0) + 1) * (v[0].1.abs_diff(v[1].1) + 1))
        .max()
        .unwrap()
}

type Pair = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
}

impl Direction {
    fn vertical(self) -> bool {
        self == Direction::PosY || self == Direction::NegY
    }

    fn horizontal(self) -> bool {
        !self.vertical()
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    start: Pair,
    end: Pair,
    dir: Direction,
}

impl Edge {
    fn new(start: Pair, end: Pair) -> Self {
        let dir = if start.0 < end.0 {
            assert_eq!(start.1, end.1);
            Direction::PosX
        } else if start.0 > end.0 {
            assert_eq!(start.1, end.1);
            Direction::NegX
        } else if start.1 < end.1 {
            assert_eq!(start.0, end.0);
            Direction::PosY
        } else if start.1 > end.1 {
            assert_eq!(start.0, end.0);
            Direction::NegY
        } else {
            panic!("{start:?} {end:?}")
        };

        Edge { start, end, dir }
    }
}

struct Polygon {
    edges: Vec<Edge>,
}

fn edges_intersect(e1: Edge, e2: Edge) -> bool {
    if e1.start == e2.start || e1.start == e2.end || e1.end == e2.start || e1.end == e2.end {
        return false;
    }
    if e1.dir.horizontal() {
        assert!(e2.dir.vertical());
        let min_y = e2.start.1.min(e2.end.1);
        let max_y = e2.start.1.max(e2.end.1);
        assert_eq!(e1.start.1, e1.end.1);
        if e1.start.1 < min_y || e1.start.1 > max_y {
            return false;
        }
        let min_x = e1.start.0.min(e1.end.0);
        let max_x = e1.start.0.max(e1.end.0);
        assert_eq!(e2.start.0, e2.end.0);
        return e1.start.0 != e2.start.0
            && e1.end.0 != e2.start.0
            && e1.start.1 != e2.start.1
            && e1.end.1 != e2.end.1
            && min_x <= e2.start.0
            && max_x >= e2.start.0;
    } else {
        assert!(e2.dir.horizontal());
        let min_x = e2.start.0.min(e2.end.0);
        let max_x = e2.start.0.max(e2.end.0);
        assert_eq!(e1.start.0, e1.end.0);
        if e1.start.0 < min_x || e1.start.0 > max_x {
            return false;
        }
        let min_y = e1.start.1.min(e1.end.1);
        let max_y = e1.start.1.max(e1.end.1);
        assert_eq!(e2.start.1, e2.end.1);
        return e1.start.1 != e2.start.1
            && e1.end.1 != e2.start.1
            && e1.start.0 != e2.start.0
            && e1.end.0 != e2.end.0
            && min_y <= e2.start.1
            && max_y >= e2.start.1;
    }
}

impl Polygon {
    pub fn edge_intersections(&self, e: Edge) -> usize {
        self.edges
            .iter()
            .cloned()
            .filter(|edge| e.dir.vertical() != edge.dir.vertical() && edges_intersect(e, *edge))
            .count()
    }
    pub fn rect_is_inside(&self, c1: Pair, c2: Pair) -> bool {
        // assume points are not colinear
        assert_ne!(c1.0, c2.0);
        assert_ne!(c1.1, c2.1);
        let left_x = c1.0.min(c2.0);
        let right_x = c1.0.max(c2.0);
        let bottom_y = c1.1.min(c2.1);
        let top_y = c1.1.max(c2.1);
        let top_edge = Edge::new((left_x, top_y), (right_x, top_y));
        let right_edge = Edge::new((right_x, top_y), (right_x, bottom_y));
        let bottom_edge = Edge::new((right_x, bottom_y), (left_x, bottom_y));
        let left_edge = Edge::new((left_x, bottom_y), (left_x, top_y));

        (self.edge_intersections(top_edge))
            + (self.edge_intersections(right_edge))
            + (self.edge_intersections(bottom_edge))
            + (self.edge_intersections(left_edge))
            == 0
    }
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> usize {
    let points = input_lines(input)
        .filter_map(|line| {
            line.split_once(',')
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        })
        .collect_vec();
    let edges = points
        .iter()
        .cloned()
        .circular_tuple_windows()
        .filter(|(a, b)| a != b)
        .map(|(a, b)| Edge::new(a, b))
        .collect_vec();
    let poly = Polygon { edges };

    points
        .into_iter()
        .permutations(2)
        .filter(|v| v[0].0 != v[1].0 && v[0].1 != v[1].1 && poly.rect_is_inside(v[0], v[1]))
        .map(|v| (v[0].0.abs_diff(v[1].0) + 1) * (v[0].1.abs_diff(v[1].1) + 1))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 50);
        assert_eq!(
            solve_part1(include_str!("../input/2025/day9.txt")),
            4769758290
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 24);
        assert_eq!(
            solve_part2(include_str!("../input/2025/day9.txt")),
            1588990708,
        );
    }

    #[test]
    fn test_edges_intersect() {
        assert!(!edges_intersect(
            Edge::new((0, 0), (4, 0)),
            Edge::new((0, 1), (0, 0)),
        ));
        assert!(!edges_intersect(
            Edge::new((0, 1), (0, 0)),
            Edge::new((0, 0), (4, 0)),
        ));
        assert!(edges_intersect(
            Edge::new((0, 0), (4, 0)),
            Edge::new((2, 1), (2, 0)),
        ));
        assert!(edges_intersect(
            Edge::new((2, 1), (2, 0)),
            Edge::new((0, 0), (4, 0)),
        ));
        assert!(edges_intersect(
            Edge::new((2, 2), (2, 0)),
            Edge::new((0, 1), (4, 1)),
        ));
        assert!(edges_intersect(
            Edge::new((2, 2), (2, 0)),
            Edge::new((4, 1), (1, 1)),
        ));
        assert!(edges_intersect(
            Edge::new((2, 0), (2, 3)),
            Edge::new((2, 1), (0, 1)),
        ));
        assert!(edges_intersect(
            Edge::new((2, 1), (0, 1)),
            Edge::new((2, 0), (2, 3)),
        ));
        assert!(edges_intersect(
            Edge::new((2, 0), (2, 3)),
            Edge::new((3, 1), (0, 1)),
        ));
        assert!(!edges_intersect(
            Edge::new((2, 0), (2, 3)),
            Edge::new((3, 4), (0, 4))
        ));
        assert!(!edges_intersect(
            Edge::new((2, 0), (2, 3)),
            Edge::new((2, 3), (1, 3))
        ));
        assert!(!edges_intersect(
            Edge::new((2, 0), (2, 3)),
            Edge::new((2, 3), (5, 3))
        ));
        assert!(!edges_intersect(
            Edge::new((2, 3), (5, 3)),
            Edge::new((2, 0), (2, 3))
        ));
        assert!(edges_intersect(
            Edge::new((2, 3), (5, 3)),
            Edge::new((2, 0), (2, 4))
        ));
    }

    #[test]
    fn test_rect_is_inside() {
        let points = input_lines(TEST_INPUT)
            .filter_map(|line| {
                line.split_once(',')
                    .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            })
            .collect_vec();
        let edges = points
            .iter()
            .cloned()
            .circular_tuple_windows()
            .filter(|(a, b)| a != b)
            .map(|(a, b)| Edge::new(a, b))
            .collect_vec();
        let poly = Polygon { edges };
        assert!(poly.rect_is_inside((7, 2), (11, 5)));
    }
}
