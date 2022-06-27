use std::{fs::File, io::Read};

#[derive(Debug)]
struct Edge {
    a: usize, // tail
    b: usize, // head
    c: isize, // cost
}

#[derive(Debug)]
struct CostMatrix {
    n: usize,        // number of vertices
    mat: Vec<isize>, // the matrix represented as a flat row-major vector
}

impl CostMatrix {
    fn new(n: usize) -> Self {
        Self {
            n,
            mat: vec![isize::MAX; n * n],
        }
    }

    fn get(&self, i: usize, j: usize) -> isize {
        self.mat[i * self.n + j]
    }

    fn set(&mut self, i: usize, j: usize, val: isize) {
        self.mat[i * self.n + j] = val;
    }
}

// Given an edge list representing a graph it returns the final CostMatrix
fn floyd_warshall(n: usize, edge_list: Vec<Edge>) -> Option<CostMatrix> {
    let mut prev = CostMatrix::new(n);

    for i in 0..n {
        for j in 0..n {
            if i == j {
                prev.set(i, j, 0);
            }
        }
    }

    for e in edge_list {
        prev.set(e.a, e.b, e.c);
    }

    let mut new = CostMatrix::new(n);
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                new.set(
                    i,
                    j,
                    isize::min(
                        prev.get(i, j),
                        prev.get(i, k).saturating_add(prev.get(k, j)),
                    ),
                );
            }
        }
        std::mem::swap(&mut prev, &mut new);
    }

    for i in 0..n {
        for j in 0..n {
            if i == j && prev.get(i, j) < 0 {
                return None;
            }
        }
    }

    return Some(prev);
}

fn minimum_shortest_path(cm: &Option<CostMatrix>) -> Option<Edge> {
    if cm.is_none() {
        return None;
    }

    let cm = cm.as_ref().unwrap();

    let mut res = Edge {
        a: usize::MAX,
        b: usize::MAX,
        c: isize::MAX,
    };

    for i in 0..cm.n {
        for j in 0..cm.n {
            if i == j {
                continue;
            }

            if cm.get(i, j) < res.c {
                res.a = i;
                res.b = j;
                res.c = cm.get(i, j);
            }
        }
    }

    Some(res)
}

fn parse_input(input: &str) -> (usize, Vec<Edge>) {
    let mut lines = input.lines();

    let mut first_line_split = lines.next().unwrap().split_whitespace();
    let n = first_line_split.next().unwrap().parse::<usize>().unwrap();

    let edge_list = lines
        .map(|line| {
            let mut split = line.split_whitespace();
            let a = split.next().unwrap().parse::<usize>().unwrap() - 1;
            let b = split.next().unwrap().parse::<usize>().unwrap() - 1;
            let c = split.next().unwrap().parse::<isize>().unwrap();

            Edge { a, b, c }
        })
        .collect();

    (n, edge_list)
}

fn parse_file(name: &str) -> (usize, Vec<Edge>) {
    let mut file = File::open(format!("data/{}.txt", name)).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    parse_input(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise() {
        let (n1, edge_list1) = parse_file("graph1");
        let (n2, edge_list2) = parse_file("graph2");
        let (n3, edge_list3) = parse_file("graph3");

        dbg!(1, minimum_shortest_path(&floyd_warshall(n1, edge_list1)));
        dbg!(2, minimum_shortest_path(&floyd_warshall(n2, edge_list2)));
        dbg!(3, minimum_shortest_path(&floyd_warshall(n3, edge_list3)));
    }
}
