use std::{collections::HashMap, num::NonZeroUsize};

use petgraph::{
    algo::dijkstra,
    graph::{DiGraph, NodeIndex},
    Graph,
};

// SAFETY: values are non-zero.
const TILE_ONCE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };
const TILE_TIMES_FIVE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(5) };

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("--- Day 15: Chiton ---");

    dbg!(shortest_path_to_dest(INPUT, TILE_ONCE));
    dbg!(shortest_path_to_dest(INPUT, TILE_TIMES_FIVE));
}

fn shortest_path_to_dest(input: &str, repeat: NonZeroUsize) -> i32 {
    let (g, start, dest) = build_graph(input, repeat);

    // Using a library for the main algorithm required by the challenge feels like cheating a bit,
    // but it gives me some experience with petgraph (and I already implemented my share of these
    // graph algorithms).  That said, writing a solution tailored for these lattice graphs, in
    // particular one that doesn't require building a graph and can mostly work directly on top of
    // the input "map" matrix, would also be interesting.
    let res = dijkstra(&g, start, Some(dest), |e| *e.weight());

    res[&dest]
}

fn build_graph(s: &str, tile_multiplier: NonZeroUsize) -> (DiGraph<(), i32>, NodeIndex, NodeIndex) {
    let mut g = Graph::new();
    let mut nodes = HashMap::new();
    let tile_multiplier = tile_multiplier.get();

    let nsq = s.lines().next().unwrap().len();

    for (i, line) in s.lines().enumerate() {
        for (j, _) in line.char_indices() {
            let risk: i32 = line[j..j + 1].parse().expect("invalid position risk");
            for di in 0..tile_multiplier {
                for dj in 0..tile_multiplier {
                    let idx = g.add_node(());
                    let risk = risk + di as i32 + dj as i32;
                    nodes.insert((i + di * nsq, j + dj * nsq), (idx, (risk - 1) % 9 + 1));
                }
            }
        }
    }

    let nsq = nsq * tile_multiplier;
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "};

    #[test]
    fn shortest_path_on_sample() {
        assert_eq!(shortest_path_to_dest(SAMPLE, TILE_ONCE), 40);
    }

    #[test]
    fn shortest_path_on_sample_times_five() {
        assert_eq!(shortest_path_to_dest(SAMPLE, TILE_TIMES_FIVE), 315);
    }

    #[test]
    fn does_not_regress_on_part1() {
        assert_eq!(shortest_path_to_dest(INPUT, TILE_ONCE), 702);
    }

    #[test]
    fn does_not_regress_on_part2() {
        assert_eq!(shortest_path_to_dest(INPUT, TILE_TIMES_FIVE), 2955);
    }
}
