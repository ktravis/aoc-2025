use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::input_lines;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    x: u32,
    y: u32,
    z: u32,
}

impl From<(u32, u32, u32)> for Pos {
    fn from((x, y, z): (u32, u32, u32)) -> Self {
        Self { x, y, z }
    }
}

impl Pos {
    pub fn distance(&self, other: Self) -> u32 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

#[aoc_generator(day8)]
pub fn generate(input: &str) -> Vec<Pos> {
    let lines = input_lines(input);
    lines
        .map(|line| {
            line.splitn(3, ",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32, u32)>()
                .unwrap()
                .into()
        })
        .sorted_by_key(|p: &Pos| p.x + p.y + p.z)
        .collect_vec()
}

struct PairIter<'a> {
    positions: &'a [Pos],
    i: usize,
    skip: &'a mut HashSet<(Pos, Pos)>,
}

impl<'a> Iterator for PairIter<'a> {
    type Item = (Pos, Pos);

    fn next(&mut self) -> Option<Self::Item> {
        let mut j = self.i + 1;
        while self.i < self.positions.len() && j < self.positions.len() {
            let a = self.positions[self.i].clone();
            let b = self.positions[j].clone();
            if !self.skip.contains(&(a, b)) {
                println!("{} {:?} {j} {:?}", self.i, a, b);
                self.i += 1;
                return Some((a, b));
            }
            println!("already checked {:?} {:?}", a, b);
            j += 1;
            if j >= self.positions.len() {
                self.i += 1;
                j = self.i + 1;
            }
        }
        None
    }
}

fn iter_pairs<'a>(positions: &'a [Pos], skip: &'a mut HashSet<(Pos, Pos)>) -> PairIter<'a> {
    PairIter {
        positions,
        skip,
        i: 0,
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(positions: &Vec<Pos>) -> usize {
    let mut network = 0;
    let mut connected_networks: HashSet<(u32, u32)> = HashSet::new();
    let mut connections: HashMap<Pos, u32> = HashMap::new();
    let mut network_sizes: HashMap<u32, usize> = HashMap::new();
    let mut checked: HashSet<(Pos, Pos)> = HashSet::new();

    while checked.len() < 1000 && checked.len() < positions.len() - 1 {
        let (a, b) = iter_pairs(&positions[..], &mut checked)
            .fuse()
            .min_by_key(|(a, b)| a.distance(b.clone()))
            .unwrap();
        println!("MIN    {a:?} {b:?} = {}", a.distance(b));
        checked.insert((a, b));
        match (connections.get(&a), connections.get(&b)) {
            (None, None) => {
                // both nodes are solo, connect them
                connections.insert(a, network);
                connections.insert(b, network);
                *network_sizes.entry(network).or_default() += 2;
                network += 1;
            }
            (None, Some(n)) => {
                // add one node to network
                let x = *n;
                connections.insert(a, x);
                *network_sizes.entry(x).or_default() += 2;
            }
            (Some(n), None) => {
                // same
                let x = *n;
                connections.insert(b, x);
                *network_sizes.entry(x).or_default() += 2;
            }
            (Some(n1), Some(n2)) => {
                // connect the two networks
                connected_networks.insert((*n1.min(n2), *n1.max(n2)));
            }
        }
    }
    println!("{:?}", network_sizes);
    println!("{:?}", network_sizes);
    for (n1, n2) in &connected_networks {
        *network_sizes.get_mut(n1).unwrap() += network_sizes[n2].saturating_sub(1);
        *network_sizes.get_mut(n2).unwrap() = 0;
    }
    println!("{:?}", network_sizes);
    network_sizes
        .iter()
        .map(|(_, size)| *size)
        .k_largest(3)
        .product()
}

#[aoc(day8, part2)]
pub fn solve_part2(positions: &Vec<Pos>) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn test_part1() {
        println!("testing");
        assert_eq!(solve_part1(&generate(TEST_INPUT)), 4277556);
        // assert_eq!(
        //     solve_part1(&generate(include_str!("../input/2025/day8.txt"))),
        //     6891729672676
        // );
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&generate(TEST_INPUT)), 4277556);
        assert_eq!(
            solve_part2(&generate(include_str!("../input/2025/day8.txt"))),
            9770311947567
        );
    }
}
