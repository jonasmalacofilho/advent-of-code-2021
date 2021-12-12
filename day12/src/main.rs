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
    dbg!(count_paths(&network));
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

fn count_paths(network: &CaveNetwork) -> usize {
    // Paths are found using a depth-first search.  A recursive implementation is simpler to reason
    // about (due to having less noise caused by a manually managed stack) and all graphs to be
    // analyzed are rather small

    let mut paths = vec![]; // wasteful, but useful for debugging

    fn visit(
        graph: &CaveGraph,
        cur: NodeIndex,
        end: NodeIndex,
        mut visited: Vec<NodeIndex>,
        paths: &mut Vec<Vec<NodeIndex>>,
    ) {
        if cur == end {
            // Found a new path to "end"
            paths.push(visited);
            return;
        } else if graph[cur].0 == CaveSize::Small && visited.contains(&cur) {
            // Small caves can only appear once; this is also responsible for preventing infinitive
            // loops, otherwise the graph would have unbounded cycles
            return;
        } else {
            visited.push(cur);
        }

        for neighbor in graph.neighbors(cur) {
            visit(graph, neighbor, end, visited.clone(), paths);
        }
    }

    visit(
        &network.graph,
        network.start,
        network.end,
        vec![],
        &mut paths,
    );

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
        assert_eq!(count_paths(&network), 10);
    }

    #[test]
    fn paths_in_larger_example() {
        let network = parse(LARGER_SAMPLE);
        assert_eq!(count_paths(&network), 19);
    }

    #[test]
    fn paths_in_largest_example() {
        let network = parse(LARGEST_SAMPLE);
        assert_eq!(count_paths(&network), 226);
    }

    #[test]
    fn does_not_regress() {
        let network = parse(INPUT);
        assert_eq!(count_paths(&network), 3495);
    }
}
