use std::{collections::BTreeSet, isize};

#[derive(Clone, Debug, PartialEq)]
struct Graph {
    n: usize,                    // number of nodes
    m: usize,                    // number of edges
    a: Vec<Vec<(usize, isize)>>, // adjacency list
}

impl Graph {
    fn new_from_str(string: &str) -> Self {
        let mut lines = string.lines();
        let first = lines.next().unwrap();

        let mut first_split = first.split_whitespace();
        let n = first_split.next().unwrap().parse::<usize>().unwrap();
        let m = first_split.next().unwrap().parse::<usize>().unwrap();

        let mut a = vec![Vec::new(); n];
        for line in lines {
            let mut split = line.split_whitespace();
            let u = split.next().unwrap().parse::<usize>().unwrap() - 1;
            let v = split.next().unwrap().parse::<usize>().unwrap() - 1;
            let c = split.next().unwrap().parse::<isize>().unwrap();

            a[u].push((v, c));
            a[v].push((u, c));
        }

        Self { n, m, a }
    }

    fn prims_algorithm(&self, start: usize) -> isize {
        let mut current_keys = vec![isize::MAX; self.n];
        current_keys[0] = isize::MIN;
        let mut heap: BTreeSet<(isize, usize)> = BTreeSet::new(); // (key, vertex)

        for &(edge_to, edge_cost) in self.a[start].iter() {
            if current_keys[edge_to] > edge_cost {
                heap.remove(&(current_keys[edge_to], edge_to));
                current_keys[edge_to] = edge_cost;
                heap.insert((current_keys[edge_to], edge_to));
            }
        }

        let mut cost = 0;
        while !heap.is_empty() {
            let (k, v) = heap.pop_first().unwrap();
            // dbg!((k, v), &current_keys);
            cost += k;
            current_keys[v] = isize::MIN;

            for &(w, c) in self.a[v].iter() {
                if current_keys[w] > c {
                    heap.remove(&(current_keys[w], w));
                    current_keys[w] = c;
                    heap.insert((current_keys[w], w));
                }
            }
        }

        cost
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    const SMALL: &str = "4 5
1 2 1
1 3 4
1 4 3
2 4 2
3 4 5";

    #[test]
    fn small() {
        let graph = Graph::new_from_str(SMALL);

        assert_eq!(graph.prims_algorithm(0), 7);
    }

    #[test]
    fn exercise() {
        let mut file = File::open("data/edges.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let graph = Graph::new_from_str(&buffer);
        // dbg!(&graph);
        dbg!("graph parsed");
        dbg!(graph.prims_algorithm(0));
    }
}
