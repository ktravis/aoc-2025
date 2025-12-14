use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use aoc_runner_derive::aoc;
use itertools::Itertools;

use crate::utils::input_lines;

#[derive(Debug, Default, Clone)]
struct Node {
    name: String,
    outs: Vec<Arc<Mutex<Node>>>,
    paths_to: HashMap<String, usize>,
}

impl Node {
    fn new(name: String) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            name,
            ..Default::default()
        }))
    }

    fn paths_to_node(self: &mut Node, name: &str) -> usize {
        if let Some(n) = self.paths_to.get(name) {
            return *n;
        }
        let paths = self
            .outs
            .iter()
            .map(|x| {
                let mut n = x.lock().unwrap();
                if n.name == name {
                    1
                } else {
                    n.paths_to_node(name)
                }
            })
            .sum();
        self.paths_to.insert(name.to_string(), paths);
        paths
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> usize {
    let out = Node::new("out".into());
    let mut nodes = HashMap::new();
    nodes.insert("out".into(), out.clone());
    let mut conns = HashMap::new();
    for line in input_lines(input) {
        let (name, outs) = line.split_once(":").unwrap();
        let name = name.to_string();
        nodes.insert(name.clone(), Node::new(name.clone()));
        conns.insert(
            name,
            outs.split(" ")
                .map(str::to_string)
                .filter(|s| !s.is_empty())
                .collect_vec(),
        );
    }
    for (name, outs) in conns {
        let x = outs
            .into_iter()
            .map(|s| nodes.get(&s).unwrap().clone())
            .collect_vec();
        nodes.entry(name).and_modify(|n| {
            n.lock().unwrap().outs = x;
        });
    }
    nodes
        .get("you")
        .unwrap()
        .lock()
        .unwrap()
        .paths_to_node("out")
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> usize {
    let out = Node::new("out".into());
    let mut nodes = HashMap::new();
    nodes.insert("out".into(), out.clone());
    let mut conns = HashMap::new();
    for line in input_lines(input) {
        let (name, outs) = line.split_once(":").unwrap();
        let name = name.to_string();
        nodes.insert(name.clone(), Node::new(name.clone()));
        conns.insert(
            name,
            outs.split(" ")
                .map(str::to_string)
                .filter(|s| !s.is_empty())
                .collect_vec(),
        );
    }
    for (name, outs) in conns {
        let x = outs
            .into_iter()
            .map(|s| nodes.get(&s).unwrap().clone())
            .collect_vec();
        nodes.entry(name).and_modify(|n| {
            n.lock().unwrap().outs = x;
        });
    }
    // what is  a e s t h e t i c s
    let dac_to_out = nodes
        .get("dac")
        .unwrap()
        .lock()
        .unwrap()
        .paths_to_node("out");
    let fft_to_out = nodes
        .get("fft")
        .unwrap()
        .lock()
        .unwrap()
        .paths_to_node("out");
    let dac_to_fft = nodes
        .get("dac")
        .unwrap()
        .lock()
        .unwrap()
        .paths_to_node("fft");
    let fft_to_dac = nodes
        .get("fft")
        .unwrap()
        .lock()
        .unwrap()
        .paths_to_node("dac");
    let svr_to_fft = nodes
        .get("svr")
        .unwrap()
        .lock()
        .unwrap()
        .paths_to_node("fft");
    let svr_to_dac = nodes
        .get("svr")
        .unwrap()
        .lock()
        .unwrap()
        .paths_to_node("dac");
    svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 5);
        assert_eq!(solve_part1(include_str!("../input/2025/day11.txt")), 523);
    }

    const TEST_INPUT2: &'static str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT2), 2);
        assert_eq!(
            solve_part2(include_str!("../input/2025/day11.txt")),
            517315308154944
        );
    }
}
