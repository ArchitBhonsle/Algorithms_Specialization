use std::collections::HashMap;

use super::union_find::UnionFind;

#[derive(Debug, Clone)]
struct Graph {
    n: usize,
    edge_list: Vec<Edge>,
}

#[derive(Debug, Clone)]
struct Edge {
    a: usize,
    b: usize,
    c: usize,
}

fn parse_input(string: &str) -> Graph {
    let mut lines = string.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let mut edge_list = lines
        .map(|line| {
            let mut split = line.split_whitespace();
            let a = split.next().unwrap().parse::<usize>().unwrap() - 1;
            let b = split.next().unwrap().parse::<usize>().unwrap() - 1;
            let c = split.next().unwrap().parse::<usize>().unwrap();

            Edge { a: usize::min(a, b), b: usize::max(a, b), c }
        })
        .collect::<Vec<Edge>>();

    edge_list.sort_by_key(|x| x.c);

    Graph { n, edge_list }
}

fn kruskals_algorithm(graph: &Graph) -> usize {
    let mut uf = UnionFind::new(graph.n);

    let mut iter = graph.edge_list.iter();
    let mut cost = 0;
    while let Some(&Edge { a, b, c }) = iter.next() {
        if uf.find(a) == uf.find(b) {
            // cycle
            continue;
        }

        cost += c;
        uf.union(a, b);
        dbg!((a, b, c));
    }

    cost
}

/// Returns the mapping of every vertex and it's corresponding cluster
fn clustering(graph: &Graph, k: usize) -> Vec<usize> {
    let mut uf = UnionFind::new(graph.n);

    let mut iter = graph.edge_list.iter();
    while uf.n_clusters() > k {
        let &Edge { a, b, c: _ } = iter.next().unwrap();

        if uf.find(a) == uf.find(b) {
            continue;
        }

        uf.union(a, b);
    }

    uf.get_leaders()
}

fn calculate_spacing(graph: &Graph, leaders: &Vec<usize>) -> usize {
    let lookup: HashMap<(usize, usize), usize> =
        graph.edge_list.iter().map(|e| ((e.a, e.b), e.c)).collect();

    let mut res = usize::MAX;
    for i in 0..leaders.len() {
        for j in (i+1)..leaders.len() {
            if leaders[i] == leaders[j] {
                continue;
            }

            res = res.min(lookup[&(i, j)]);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};

    #[test]
    fn small() {
        let small: &str = "4
1 2 1
1 3 4
1 4 3
2 4 2
3 4 5";
        let graph = parse_input(small);

        assert_eq!(kruskals_algorithm(&graph), 7);
    }

    #[test]
    fn exercise() {
        let mut file = File::open("data/clustering.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let graph = parse_input(&buffer);
        let leaders = clustering(&graph, 4);

        let spacing = calculate_spacing(&graph, &leaders);
        dbg!(spacing);
    }
}
