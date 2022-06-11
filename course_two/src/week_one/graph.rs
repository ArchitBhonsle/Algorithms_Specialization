use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    pub adj: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new_from_edge_list(edge_list: &str) -> Self {
        let mut adj = HashMap::new();

        edge_list.lines().for_each(|l| {
            let mut split = l.split_whitespace();
            let a = split
                .next()
                .expect("An edge should contain two vertices")
                .parse::<usize>()
                .unwrap()
                - 1;
            let b = split
                .next()
                .expect("An edge should contain two vertices")
                .parse::<usize>()
                .unwrap()
                - 1;

            let ae = adj.entry(a).or_insert(Vec::new());
            ae.push(b);

            adj.entry(b).or_insert(Vec::new());
        });

        Self { adj }
    }

    pub fn new_from_edge_list_undirected(edge_list: &str) -> Self {
        let mut adj = HashMap::new();

        edge_list.lines().for_each(|l| {
            let mut split = l.split_whitespace();
            let a = split
                .next()
                .expect("An edge should contain two vertices")
                .parse::<usize>()
                .unwrap()
                - 1;
            let b = split
                .next()
                .expect("An edge should contain two vertices")
                .parse::<usize>()
                .unwrap()
                - 1;

            let ae = adj.entry(a).or_insert(Vec::new());
            ae.push(b);

            let be = adj.entry(b).or_insert(Vec::new());
            be.push(a);
        });

        Self { adj }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_edge_list_test() {
        let edge_list = r"1 2
2 1";
        assert_eq!(Graph::new_from_edge_list(edge_list), Graph {
            adj: HashMap::from([
                (0, vec![1]),
                (1, vec![0]),
            ])
        });
    }

    #[test]
    fn new_from_edge_list_undirected_test() {
        let edge_list = r"1 2";

        assert_eq!(Graph::new_from_edge_list_undirected(edge_list), Graph {
            adj: HashMap::from([
                (0, vec![1]),
                (1, vec![0]),
            ])
        });
    }
}
