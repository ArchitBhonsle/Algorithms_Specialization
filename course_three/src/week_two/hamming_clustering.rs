/// We want the maximum spacing to be atleast 3.
/// This means that all the nodes with spacing less than 3 should be
/// in the same cluster.
///
/// To cluster the nodes with distance:
///
/// - 0: We just eliminate duplicate nodes.
/// - 1: We create all possible values at distance 1 using bit operations
///      and if they are in the graph we add them to the same cluster.
/// - 2: We create all possible values at distance 2 using bit operations
///      and if they are in the graph we add them to the same cluster.
use std::collections::HashMap;

use super::union_find::UnionFind;

#[derive(Clone, Debug, PartialEq)]
struct Graph {
    bits: usize,
    nodes: HashMap<usize, usize>,
}

impl Graph {
    fn new_from_str(string: &str) -> Self {
        let mut lines = string.lines();

        let mut first_line_split = lines.next().unwrap().split_whitespace();
        let _ = first_line_split.next().unwrap();
        let bits = first_line_split.next().unwrap().parse().unwrap();

        let mut nodes: HashMap<usize, usize> = HashMap::new();

        lines.for_each(|line| {
            let mut node = 0;
            for bit in line.split_whitespace() {
                node <<= 1;
                if bit == "1" {
                    node += 1;
                }
            }

            if !nodes.contains_key(&node) {
                nodes.insert(node, nodes.len());
            }
        });

        Self { bits, nodes }
    }
}

fn solve(graph: &Graph) -> usize {
    let mut uf = UnionFind::new(graph.nodes.len());

    // Cluster nodes with the distance of 1
    // This has already been done as we have eliminated duplicates

    // Cluster nodes with the distance of 1
    for &node in graph.nodes.keys() {
        // To create all the variations we change each bit of the node
        for i in 0..graph.bits {
            let variation = node ^ (1 << i);
            // If this variation is present in the graph we should cluster
            // it with node
            if graph.nodes.contains_key(&variation) {
                uf.union(graph.nodes[&node], graph.nodes[&variation]);
            }
        }
    }

    // Cluster nodes with the distance of 2
    for &node in graph.nodes.keys() {
        for i in 0..graph.bits {
            for j in (i + 1)..graph.bits {
                let variation = (node ^ (1 << i)) ^ (1 << j);
                // If this variation is present in the graph we should cluster
                // it with node
                if graph.nodes.contains_key(&variation) {
                    uf.union(graph.nodes[&node], graph.nodes[&variation]);
                }
            }
        }
    }

    uf.n_clusters()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};

    #[test]
    fn test_parsing() {
        let graph = Graph::new_from_str(
            r"10 4
0 0 0 1
0 0 0 1
0 0 0 1
0 0 1 0
0 0 1 1
0 1 0 0
0 1 0 1
0 1 1 0
0 1 1 1
1 0 0 0",
        );

        assert_eq!(
            &graph.nodes,
            &HashMap::from([
                (1, 0),
                (2, 1),
                (3, 2),
                (4, 3),
                (5, 4),
                (6, 5),
                (7, 6),
                (8, 7),
            ])
        );
    }

    #[test]
    fn test_1() {
        let graph = Graph::new_from_str(
            r"5 5
0 0 0 0 0
1 1 1 1 1
0 0 0 0 1
1 1 1 0 0
0 0 0 1 0",
        );

        assert_eq!(solve(&graph), 2);
    }

    #[test]
    fn test_2() {
        let graph = Graph::new_from_str(
            r"11 24
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1
1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1
0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0
0 1 1 1 1 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0
0 1 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0
0 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 1 1 1 1 1 1 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 1 1 1 1 1 1 1 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 1 1 1 1 1 0 0 0",
        );

        assert_eq!(solve(&graph), 6);
    }

    #[test]
    fn exercise() {
        let mut file = File::open("data/clustering_big.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let graph = Graph::new_from_str(&buffer);
        dbg!(solve(&graph));
    }
}
