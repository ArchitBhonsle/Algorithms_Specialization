use std::{cmp, collections::BinaryHeap};

#[derive(Debug, Clone)]
/// Every node in the huffman tree is either
///     - a leaf node representing an alphabet
///     - an intermediate node representing a sub-tree (a meta-node)
enum Node {
    Leaf { i: usize, w: usize },
    Meta(Box<(Node, Node, usize)>),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.w().cmp(&other.w())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.w() == other.w()
    }
}

impl Node {
    fn w(&self) -> usize {
        match self {
            &Node::Leaf { w, .. } => w,
            Node::Meta(n) => n.2,
        }
    }
    fn new_leaf(i: usize, w: usize) -> Node {
        Node::Leaf { i, w }
    }

    fn new_meta(a: Node, b: Node) -> Node {
        let w = a.w() + b.w();
        Node::Meta(Box::new((a, b, w)))
    }

    fn new_from_weights(freqs: &Weights) -> Node {
        let mut nodes: BinaryHeap<_> = freqs
            .iter()
            .enumerate()
            .map(|(i, &w)| cmp::Reverse(Node::Leaf { i, w }))
            .collect::<Vec<_>>()
            .into();

        while nodes.len() > 1 {
            let a = nodes.pop().unwrap();
            let b = nodes.pop().unwrap();

            nodes.push(cmp::Reverse(Node::new_meta(a.0, b.0)));
        }

        nodes.pop().unwrap().0
    }

    fn max_depth(&self) -> usize {
        fn recurse(n: &Node, d: usize) -> usize {
            match n {
                Node::Leaf { .. } => d,
                Node::Meta(x) => usize::max(recurse(&x.0, d + 1), recurse(&x.1, d + 1)),
            }
        }

        recurse(self, 0)
    }

    fn min_depth(&self) -> usize {
        fn recurse(n: &Node, d: usize) -> usize {
            match n {
                Node::Leaf { .. } => d,
                Node::Meta(x) => usize::min(recurse(&x.0, d + 1), recurse(&x.1, d + 1)),
            }
        }

        recurse(self, 0)
    }
}

type Weights = Vec<usize>;

fn parse_input(input: &str) -> Weights {
    input
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn basic() {
        let weights = vec![3, 2, 6, 8, 2, 6];
        let tree = Node::new_from_weights(&weights);

        assert_eq!(tree.min_depth(), 2);
        assert_eq!(tree.max_depth(), 4);
    }

    #[test]
    fn test_1() {
        let input = "10
37
59
43
27
30
96
96
71
8
76";
        let tree = Node::new_from_weights(&parse_input(input));

        assert_eq!(tree.min_depth(), 2);
        assert_eq!(tree.max_depth(), 5);
    }

    #[test]
    fn test_2() {
        let input = "15
895
121
188
953
378
849
153
579
144
727
589
301
442
327
930";
        let tree = Node::new_from_weights(&parse_input(input));

        assert_eq!(tree.min_depth(), 3);
        assert_eq!(tree.max_depth(), 6);
    }

    #[test]
    fn exercise() {
        let mut file = File::open("data/huffman.txt").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        let tree = Node::new_from_weights(&parse_input(&input));

        dbg!(tree.min_depth());
        dbg!(tree.max_depth());
    }
}
