use std::collections::{HashMap, HashSet, VecDeque};

use super::graph::Graph;

fn shortest_paths(graph: &Graph, start: usize) -> HashMap<usize, usize> {
    let mut distances = HashMap::new();
    distances.insert(start, 0);
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut queue = VecDeque::from([start]);
    while queue.len() != 0 {
        let curr = queue.pop_front().unwrap();

        for &neighbour in &graph.adj[&curr] {
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                queue.push_back(neighbour);

                distances.insert(neighbour, distances[&curr] + 1);
            }
        }
    }

    dbg!(visited);

    distances
}

fn connected_components(graph: &Graph) -> Vec<Vec<usize>> {
    let mut components = Vec::new();
    let mut visited = HashSet::new();

    fn bfs(graph: &Graph, start: usize, component: &mut Vec<usize>, visited: &mut HashSet<usize>) {
        let mut queue = VecDeque::from([start]);
        visited.insert(start);
        component.push(start);

        while queue.len() != 0 {
            let curr = queue.pop_front().unwrap();

            for &neighbour in &graph.adj[&curr] {
                if !visited.contains(&neighbour) {
                    visited.insert(neighbour);
                    queue.push_back(neighbour);

                    component.push(neighbour);
                }
            }
        }
    }

    for &vertex in graph.adj.keys() {
        if !visited.contains(&vertex) {
            let mut component = Vec::new();
            bfs(graph, vertex, &mut component, &mut visited);
            components.push(component);
        }
    }

    components
}

#[cfg(test)]
mod tests {
    use super::*;

    const E1: &str = r"1 2
1 3
2 4
3 4
3 5
4 5
4 6
5 6";

    #[test]
    fn shortest_paths_test() {
        let graph = Graph::new_from_edge_list_undirected(E1);
        dbg!(&graph);
        dbg!(shortest_paths(&graph, 0));
    }

    const E2: &str = r"1 3
1 5
3 5
5 7
5 9
2 4
6 8
6 10
8 10";

    #[test]
    fn undirected_connectivity_test() {
        let graph = Graph::new_from_edge_list_undirected(E2);
        dbg!(&graph);

        dbg!(connected_components(&graph));
    }
}
