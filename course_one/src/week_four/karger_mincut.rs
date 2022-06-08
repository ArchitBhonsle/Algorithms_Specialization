use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    pub adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new_from_str(inp: &str) -> Graph {
        let mut adj = Vec::new();
        inp.lines().for_each(|line| {
            let mut iter = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap() - 1);

            let index = iter
                .next()
                .expect("Every line should have at least one element");

            assert_eq!(adj.len(), index as usize);

            adj.push(iter.collect::<Vec<usize>>());
        });
        Graph { adj }
    }

    pub fn get_edge_vertices(&self, i: usize) -> (usize, usize) {
        let (mut trav, mut sum) = (0, 0);
        while sum + self.adj[trav].len() <= i {
            sum += self.adj[trav].len();
            trav += 1;
        }

        let a = trav;
        let b = self.adj[a][i - sum];

        (a, b)
    }

    pub fn contract_edge(&mut self, i: usize) {
        let (a, b) = self.get_edge_vertices(i);
        let (a, b) = (usize::min(a, b), usize::max(a, b));
        // dbg!((a, b), &self.adj);

        let b_list = self.adj.remove(b);
        self.adj[a].extend(b_list);
        self.adj[a].retain(|&x| x != a && x != b);

        self.adj.iter_mut().for_each(|l| {
            l.iter_mut().for_each(|x| {
                if *x == b {
                    *x = a;
                } else if *x > b {
                    *x -= 1;
                }
            })
        });
    }

    pub fn kargers_min_cut(&mut self) -> usize {
        let mut rng = rand::thread_rng();
        // dbg!();
        while self.adj.len() > 2 {
            let m = self.adj.iter().map(|x| x.len()).sum();
            let rand = rng.gen_range(0..m);
            // dbg!(rand);

            self.contract_edge(rand);
        }

        // dbg!(&self);

        dbg!(self.adj.iter().flatten().count())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    const E1: &str = r"1 2 3 4
2 1 3 4
3 1 2 4
4 1 2 3
";

    #[test]
    fn test_new_from_str() {
        let parsed = Graph::new_from_str(E1);
        let exp = Graph {
            adj: vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]],
        };

        assert_eq!(parsed, exp);
    }

    #[test]
    fn test_get_edge() {
        let graph = Graph::new_from_str(E1);

        let helper = |i: usize, exp: (usize, usize)| {
            assert_eq!(graph.get_edge_vertices(i), exp);
        };

        helper(0, (0, 1));
        helper(5, (1, 3));
        helper(6, (2, 0));
        helper(9, (3, 0));
        helper(11, (3, 2));
    }

    const E2: &str = r"1 2 3 4
2 3 4
3 4";

    #[test]
    fn test_contraction() {
        let mut graph = Graph::new_from_str(E2);
        graph.contract_edge(0);
        dbg!(&graph);

        let exp = Graph {
            adj: vec![vec![1, 2, 1, 2], vec![2]],
        };

        assert_eq!(graph, exp);
    }

    const E3: &str = r"1 2 3 4
2 3 4
3 4 5
4 6
5 6 7 8
6 7 8
7 8
8";

    #[test]
    fn test_kargers_min_cut() {
        let graph = Graph::new_from_str(E3);

        let res = (0..graph.adj.len().pow(2))
            .map(|_| graph.clone().kargers_min_cut())
            .min()
            .unwrap();

        assert_eq!(res, 2);
    }

    #[test]
    fn exercise() {
        let mut file = File::open("./src/week_four/mincut_input.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let graph = Graph::new_from_str(&buffer);

        let mut res = usize::MAX;
        for i in 0..graph.adj.len().pow(2) {
            let curr = graph.clone().kargers_min_cut();
            res = usize::min(res, curr);

            if i % 10 == 0 {
                dbg!((i, curr, res));
            }
        }
        // let res = (0..graph.adj.len().pow(2))
        //     .map(|_| graph.clone().kargers_min_cut())
        //     .min()
        //     .unwrap();

        dbg!(res);
    }
}
