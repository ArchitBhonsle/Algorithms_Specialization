use std::collections::{HashMap, HashSet};

use super::graph::Graph;

fn topological_sort(graph: &Graph) -> HashMap<usize, usize> {
    let mut current_label = graph.adj.keys().count();
    let mut ordering = HashMap::new();
    let mut visited = HashSet::new();

    fn dfs(
        graph: &Graph,
        vertex: usize,
        visited: &mut HashSet<usize>,
        current_label: &mut usize,
        ordering: &mut HashMap<usize, usize>,
    ) {
        visited.insert(vertex);
        for &neighbour in &graph.adj[&vertex] {
            if !visited.contains(&neighbour) {
                dfs(
                    &graph,
                    neighbour,
                    visited,
                    current_label,
                    ordering,
                );
            }
        }

        *current_label -= 1;
        ordering.insert(vertex, *current_label);
    }

    for &vertex in graph.adj.keys() {
        if !visited.contains(&vertex) {
            dfs(
                &graph,
                vertex,
                &mut visited,
                &mut current_label,
                &mut ordering,
            );
        }
    }

    ordering
}

#[cfg(test)]
mod tests {
    use super::*;

    const E1: &str = r"1 2
1 3
2 4
3 4";

    #[test]
    fn topological_sort_test() {
        let graph = Graph::new_from_edge_list(E1);

        let ordering = topological_sort(&graph);
        assert_eq!(ordering[&0], 0);
        assert_eq!(ordering[&3], 3);
    }
}
