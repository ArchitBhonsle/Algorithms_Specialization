use std::collections::{HashMap, HashSet};

use super::graph::Graph;

fn reverse(graph: &Graph) -> Graph {
    let mut adj = HashMap::new();

    for (vertex, neighbours) in graph.adj.iter() {
        for neighbour in neighbours.iter() {
            let neighbour_list = adj.entry(*neighbour).or_insert(Vec::new());
            neighbour_list.push(*vertex);

            adj.entry(*vertex).or_insert(Vec::new());
        }
    }

    Graph { adj }
}

fn kosaraju(graph: &Graph) -> Vec<usize> {
    let reverse = reverse(graph);

    let mut current_time = 0;
    let mut current_leader = None;

    let mut visited = HashSet::new();
    let n = graph.adj.keys().count();
    let mut leaders = vec![0; n];
    let mut ordering = vec![0; n];

    fn dfs_loop(
        graph: &Graph,
        n: usize,
        first_phase: bool,
        visited: &mut HashSet<usize>,
        current_leader: &mut Option<usize>,
        leaders: &mut Vec<usize>,
        current_time: &mut usize,
        ordering: &mut Vec<usize>,
    ) {
        let loop_order = if first_phase {
            (0..n).collect()
        } else {
            ordering.clone()
        };
        for &i in loop_order.iter().rev() {
            if !visited.contains(&i) {
                *current_leader = Some(i);
                dfs(
                    graph,
                    i,
                    visited,
                    current_leader,
                    leaders,
                    current_time,
                    ordering,
                );
            }
        }
    }

    fn dfs(
        graph: &Graph,
        current_vertex: usize,
        visited: &mut HashSet<usize>,
        current_leader: &mut Option<usize>,
        leaders: &mut Vec<usize>,
        current_time: &mut usize,
        ordering: &mut Vec<usize>,
    ) {
        visited.insert(current_vertex);
        leaders[current_vertex] = current_leader.unwrap();

        for &neighbour in graph.adj[&current_vertex].iter() {
            if !visited.contains(&neighbour) {
                dfs(
                    graph,
                    neighbour,
                    visited,
                    current_leader,
                    leaders,
                    current_time,
                    ordering,
                );
            }
        }

        ordering[*current_time] = current_vertex;
        *current_time += 1
    }

    dfs_loop(
        &reverse,
        n,
        true,
        &mut visited,
        &mut current_leader,
        &mut leaders,
        &mut current_time,
        &mut ordering,
    );

    // we need to reset certain variables since they are shared between the two phases
    visited = HashSet::new();
    current_time = 0;

    dfs_loop(
        graph,
        n,
        false,
        &mut visited,
        &mut current_leader,
        &mut leaders,
        &mut current_time,
        &mut ordering,
    );

    leaders
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::week_one::graph::Graph;

    use super::*;

    const GRAPH: &str = r"1 2
2 3
2 4
3 1
3 5
3 6
4 9
4 10
5 6
6 7
6 8
6 9
7 5
8 7
8 11
9 10
10 11
11 9";

    #[test]
    fn basic_test() {
        let graph = Graph::new_from_edge_list(GRAPH);

        kosaraju(&graph);
    }

    #[test]
    fn exercise() {
        let builder = std::thread::Builder::new()
            .name("reductor".into())
            .stack_size(8 * 1024 * 1024); // 1MB of stack space

        let handler = builder
            .spawn(|| {
                let mut file = File::open("./src/week_one/scc.txt").unwrap();
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).unwrap();

                let graph = Graph::new_from_edge_list(&buffer);

                let ordering = kosaraju(&graph);
                let mut size_counts = vec![0; graph.adj.keys().count()];
                for &leader in ordering.iter() {
                    size_counts[leader] += 1;
                }
                size_counts.sort_by(|a, b| b.partial_cmp(a).unwrap());
                println!("{:?}", &size_counts[..5]);
            })
            .unwrap();

        handler.join().unwrap();
    }
}
