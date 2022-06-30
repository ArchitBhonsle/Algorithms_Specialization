use std::collections::{HashMap, HashSet};

struct ImplicationGraph {
    n: usize,                        // number of vertices
    adj: HashMap<isize, Vec<isize>>, // adjacency list
}

impl ImplicationGraph {
    fn new_from_input(input: &str) -> Self {
        let mut lines = input.lines();

        let n = lines.next().unwrap().parse::<isize>().unwrap();
        let mut adj: HashMap<isize, Vec<isize>> = HashMap::new();

        for i in -n..=n {
            if i == 0 {
                continue;
            }
            adj.insert(i, Vec::new());
        }

        lines.for_each(|line| {
            let mut split = line.split_whitespace();
            let a = split.next().unwrap().parse::<isize>().unwrap();
            let b = split.next().unwrap().parse::<isize>().unwrap();

            // This represents a disjunction, a v b

            // If !a then b
            adj.get_mut(&-a).unwrap().push(b);
            // If !b then a
            adj.get_mut(&-b).unwrap().push(a);
        });

        ImplicationGraph { n: n as usize, adj }
    }

    fn reversed(&self) -> ImplicationGraph {
        let mut adj = HashMap::new();

        for (vertex, neighbours) in self.adj.iter() {
            for neighbour in neighbours.iter() {
                let neighbour_list = adj.entry(*neighbour).or_insert(Vec::new());
                neighbour_list.push(*vertex);

                adj.entry(*vertex).or_insert(Vec::new());
            }
        }

        ImplicationGraph { n: self.n, adj }
    }

    // Copied from course two
    fn get_scc_leaders(&self) -> HashMap<isize, isize> {
        let reverse = self.reversed();

        let mut current_time = 0;
        let mut current_leader = None;

        let mut visited = HashSet::new();
        let mut leaders: HashMap<isize, isize> = HashMap::new();
        let mut ordering = self.adj.keys().map(|x| *x).collect();

        fn dfs_loop(
            graph: &ImplicationGraph,
            visited: &mut HashSet<isize>,
            current_leader: &mut Option<isize>,
            leaders: &mut HashMap<isize, isize>,
            current_time: &mut usize,
            ordering: &mut Vec<isize>,
        ) {
            let loop_order = ordering.clone();

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
            graph: &ImplicationGraph,
            current_vertex: isize,
            visited: &mut HashSet<isize>,
            current_leader: &mut Option<isize>,
            leaders: &mut HashMap<isize, isize>,
            current_time: &mut usize,
            ordering: &mut Vec<isize>,
        ) {
            visited.insert(current_vertex);
            leaders.insert(current_vertex, current_leader.unwrap());

            if graph.adj.contains_key(&current_vertex) {
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
            }

            ordering[*current_time] = current_vertex;
            *current_time += 1
        }

        dfs_loop(
            &reverse,
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
            self,
            &mut visited,
            &mut current_leader,
            &mut leaders,
            &mut current_time,
            &mut ordering,
        );

        leaders
    }

    fn is_feasible(&self) -> bool {
        let leaders = self.get_scc_leaders();
        let n = self.n as isize;

        (-n..=n)
            .filter(|&x| x != 0)
            .all(|x| leaders[&x] != leaders[&-x])
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn small() {
        let t1 = r"2
1 2
2 -1
-1 -2";

        let t2 = r"2
1 2
-1 2
1 -2
-1 -2";

        fn helper(input: &str, exp: bool) {
            let g = ImplicationGraph::new_from_input(&input);

            assert_eq!(g.is_feasible(), exp);
        }

        helper(t1, true);
        helper(t2, false);
    }

    #[test]
    fn exercise() {
        let files = [
            "data/2sat1.txt",
            "data/2sat2.txt",
            "data/2sat3.txt",
            "data/2sat4.txt",
            "data/2sat5.txt",
            "data/2sat6.txt",
        ];

        for file in files {
            let mut file = File::open(file).unwrap();
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();

            let g = ImplicationGraph::new_from_input(&input);
            assert_eq!(g.adj.len(), g.n * 2);
            assert_eq!(g.adj.values().map(|x| x.len()).sum::<usize>(), g.n * 2);

            dbg!(g.is_feasible());
        }
    }
}
