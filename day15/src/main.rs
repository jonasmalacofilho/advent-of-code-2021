use std::{collections::HashMap, num::NonZeroUsize};

use petgraph::{
    algo::dijkstra,
    graph::{DiGraph, NodeIndex},
    Graph,
};

const SAMPLE: &str = include_str!("../sample.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Hello, world!");

    for (name, data) in [("sample", SAMPLE), ("input", INPUT)] {
        for repeat in [1, 5] {
            let (g, start, dest) = build_graph(data, NonZeroUsize::new(repeat).unwrap());
            let res = dijkstra(&g, start, Some(dest), |e| *e.weight());
            dbg!(name, repeat, res[&dest]);
        }
    }
}

fn build_graph(s: &str, repeat: NonZeroUsize) -> (DiGraph<(), i32>, NodeIndex, NodeIndex) {
    let mut g = Graph::new();
    let mut nodes = HashMap::new();
    let repeat = repeat.get();

    let nsq = s.lines().next().unwrap().len();

    for (i, line) in s.lines().enumerate() {
        for (j, _) in line.char_indices() {
            let risk: i32 = line[j..j + 1].parse().expect("invalid position risk");
            for di in 0..repeat {
                for dj in 0..repeat {
                    let idx = g.add_node(());
                    let risk = risk + di as i32 + dj as i32;
                    nodes.insert(
                        (i + di * nsq, j + dj * nsq),
                        (idx, (risk - 1) % 9 + 1),
                    );
                }
            }
        }
    }

    let nsq = nsq * repeat;
    assert_eq!(nodes.len(), nsq * nsq);

    for i in 0..nsq {
        for j in 0..nsq {
            let this = nodes[&(i, j)];

            let bottom = if i < nsq - 1 { Some((i + 1, j)) } else { None };
            let right = if j < nsq - 1 { Some((i, j + 1)) } else { None };

            for other in [bottom, right].into_iter().flatten() {
                let other = nodes[&other];
                g.update_edge(this.0, other.0, other.1);
                g.update_edge(other.0, this.0, this.1);
            }
        }
    }

    (g, nodes[&(0, 0)].0, nodes[&(nsq - 1, nsq - 1)].0)
}
