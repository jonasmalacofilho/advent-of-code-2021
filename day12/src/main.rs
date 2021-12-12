use std::collections::HashMap;

use indoc::indoc;
use petgraph::graph::{NodeIndex, UnGraph};
use smol_str::SmolStr;

const INPUT: &str = indoc! {"
    OU-xt
    hq-xt
    br-HP
    WD-xt
    end-br
    start-OU
    hq-br
    MH-hq
    MH-start
    xt-br
    end-WD
    hq-start
    MH-br
    qw-OU
    hm-WD
    br-WD
    OU-hq
    xt-MH
    qw-MH
    WD-qw
    end-qw
    qw-xt
"};

fn main() {
    println!("--- Day 12: Passage Pathing ---");

    let network = parse(INPUT);
    dbg!(count_paths(&network, Mode::SmallOnce));
    dbg!(count_paths(&network, Mode::SmallTwiceOnce));
}

#[derive(Debug, PartialEq, Eq)]
enum CaveSize {
    Small,
    Big,
}

impl From<&str> for CaveSize {
    fn from(name: &str) -> Self {
        let first = name.chars().next().expect("cave name is empty string");
        if first.is_lowercase() {
            CaveSize::Small
        } else if first.is_uppercase() {
            CaveSize::Big
        } else {
            panic!("cannot derive cave size from name: {}", name);
        }
    }
}

type CaveGraph = UnGraph<(CaveSize, SmolStr), ()>;

#[derive(Debug)]
struct CaveNetwork {
    graph: CaveGraph,
    start: NodeIndex,
    end: NodeIndex,
}

fn parse(s: &str) -> CaveNetwork {
    let mut nodes = HashMap::new();
    let mut graph = CaveGraph::new_undirected();

    for line in s.lines() {
        let (from, to) = line.split_once('-').expect("invalid edge format");

        let n1 = *nodes
            .entry(from.to_string())
            .or_insert_with(|| graph.add_node((from.into(), from.into())));
        let n2 = *nodes
            .entry(to.to_string())
            .or_insert_with(|| graph.add_node((to.into(), to.into())));

        graph.add_edge(n1, n2, ());
    }

    CaveNetwork {
        graph,
        start: nodes["start"],
        end: nodes["end"],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    SmallOnce,
    SmallTwiceOnce,
}

fn count_paths(network: &CaveNetwork, mode: Mode) -> usize {
    // Paths are found using a depth-first search.  A recursive implementation is simpler to reason
    // about (due to having less noise caused by a manually managed stack) and all graphs to be
    // analyzed are rather small

    let mut paths = vec![]; // wasteful, but useful for debugging

    assert_eq!(network.graph[network.start].1, "start");
    assert_eq!(network.graph[network.end].1, "end");

    fn visit(
        graph: &CaveGraph,
        cur: NodeIndex,
        mut mode: Mode,
        mut visited: Vec<NodeIndex>,
        paths: &mut Vec<Vec<NodeIndex>>,
    ) {
        if graph[cur].0 == CaveSize::Small && visited.contains(&cur) {
            // Small caves can only appear a limited number of times; this is also what prevents
            // infinite loops, since otherwise the graph would have unbounded cycles
            match mode {
                Mode::SmallTwiceOnce if graph[cur].1 != "start" => {
                    // Allow this cave appear twice in the path, but not later ones
                    mode = Mode::SmallOnce;
                }
                _ => {
                    // This path is invalid, discard it
                    return;
                }
            };
        }

        visited.push(cur);

        if graph[cur].1 == "end" {
            // Found a new path to "end"
            paths.push(visited);
            return;
        }

        for neighbor in graph.neighbors(cur) {
            visit(graph, neighbor, mode, visited.clone(), paths);
        }
    }

    visit(&network.graph, network.start, mode, vec![], &mut paths);

    // fn fmt_path(graph: &CaveGraph, path: &[NodeIndex]) -> String {
    //     itertools::Itertools::intersperse(path.iter().map(|&n| graph[n].1.as_str()), "-").collect()
    // }
    // dbg!(paths.iter().map(|p| fmt_path(&network.graph, p)).collect::<Vec<_>>());

    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TINY_SAMPLE: &str = indoc! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "};

    const LARGER_SAMPLE: &str = indoc! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "};

    const LARGEST_SAMPLE: &str = indoc! {"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    "};

    #[test]
    fn paths_in_tiny_example() {
        let network = parse(TINY_SAMPLE);
        assert_eq!(count_paths(&network, Mode::SmallOnce), 10);
        assert_eq!(count_paths(&network, Mode::SmallTwiceOnce), 36);
    }

    #[test]
    fn paths_in_larger_example() {
        let network = parse(LARGER_SAMPLE);
        assert_eq!(count_paths(&network, Mode::SmallOnce), 19);
        assert_eq!(count_paths(&network, Mode::SmallTwiceOnce), 103);
    }

    #[test]
    fn paths_in_largest_example() {
        let network = parse(LARGEST_SAMPLE);
        assert_eq!(count_paths(&network, Mode::SmallOnce), 226);
        assert_eq!(count_paths(&network, Mode::SmallTwiceOnce), 3509);
    }

    #[test]
    fn does_not_regress() {
        let network = parse(INPUT);
        assert_eq!(count_paths(&network, Mode::SmallOnce), 3495);
        assert_eq!(count_paths(&network, Mode::SmallTwiceOnce), 94849);
    }
}
