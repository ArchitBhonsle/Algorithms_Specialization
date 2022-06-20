use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Desc {
    Leader(usize),
    Size(usize),
}

#[derive(Clone, Debug)]
pub struct UnionFind {
    map: Vec<Desc>,
    clusters: HashMap<usize, Vec<usize>>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            map: vec![Desc::Size(1); size],
            clusters: (0..size).map(|x| (x, vec![x])).collect(),
        }
    }

    pub fn find(&self, u: usize) -> usize {
        match self.map[u] {
            Desc::Leader(x) => x,
            Desc::Size(_) => u,
        }
    }

    pub fn union(&mut self, u: usize, v: usize) {
        let u_leader = self.find(u);
        let v_leader = self.find(v);

        if u_leader == v_leader {
            return;
        }

        let u_size = self.clusters[&u_leader].len();
        let v_size = self.clusters[&v_leader].len();

        // a is the larger cluster
        let (a_leader, b_leader, a_size, b_size) = if u_size >= v_size {
            (u_leader, v_leader, u_leader, v_size)
        } else {
            (v_leader, u_leader, v_leader, u_size)
        };

        // We merge b into a

        // First we update a's size
        self.map[a_leader] = Desc::Size(a_size + b_size);

        // Make the cluster b an empty vector
        let b_cluster = self.clusters.remove(&b_leader).unwrap();

        self.clusters.get_mut(&a_leader).unwrap().reserve(b_size);
        for vertex in b_cluster {
            // Change the mapping of every vertex in b's clusters to have a's leader
            self.map[vertex] = Desc::Leader(a_leader);

            // Insert every vertex in b's clusters into a's clusters
            self.clusters.get_mut(&a_leader).unwrap().push(vertex);
        }
    }

    pub fn n_clusters(&self) -> usize {
        self.clusters.len()
    }

    pub fn get_leaders(&self) -> Vec<usize> {
        self.map
            .iter()
            .enumerate()
            .map(|(i, d)| match d {
                &Desc::Leader(x) => x,
                &Desc::Size(_) => i,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut uf = UnionFind::new(4);
        dbg!(&uf);

        assert_eq!(uf.find(1), 1);

        uf.union(0, 1);
        assert!(uf.find(0) == 0 && uf.find(1) == 0 && uf.clusters[&0].len() == 2);

        uf.union(2, 1);
        assert!(uf.find(2) == 0 && uf.clusters[&0].len() == 3);

        uf.union(2, 3);
        assert!(uf.find(3) == 0 && uf.find(1) == 0 && uf.clusters[&0].len() == 4);

        dbg!(&uf);
    }
}
