use std::collections::BTreeSet;

#[derive(Debug)]
struct Graph {
    adj: Vec<Vec<(usize, usize)>>,
    n: usize,
    m: usize,
}

fn read_graph(data: &str) -> Graph {
    let mut n = 0;
    let mut m = 0;
    data.lines().for_each(|x| {
        let mut iter = x.split_whitespace();
        let index = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        n = n.max(index);

        iter.for_each(|pair| {
            let vertex = pair.split(',').next().unwrap().parse::<usize>().unwrap() - 1;
            n = n.max(vertex);
            m += 1;
        });
    });
    n += 1;

    let mut adj = vec![Vec::new(); n];

    data.lines().for_each(|x| {
        let mut iter = x.split_whitespace();
        let index = iter.next().unwrap().parse::<usize>().unwrap() - 1;

        adj[index].extend(
            iter.map(|pair| {
                let mut pair_iter = pair.split(',');
                let vertex = pair_iter.next().unwrap().parse::<usize>().unwrap() - 1;
                let distance = pair_iter.next().unwrap().parse::<usize>().unwrap();

                (vertex, distance)
            })
        );
    });

    Graph { adj, n, m }
}

fn dijkstras_algorithm(graph: &Graph, start: usize) -> Vec<usize> {
    let mut distances = vec![usize::MAX; graph.n];
    distances[start] = 0;

    let mut queue: BTreeSet<(usize, usize)> = BTreeSet::new(); // (distance, vertex)
    queue.insert((0, start));

    while queue.len() != 0 {
        let (_, current_vertex) = queue.pop_first().unwrap();

        for &(neighbour_vertex, neighbour_distance) in graph.adj[current_vertex].iter() {
            if distances[current_vertex] + neighbour_distance < distances[neighbour_vertex] {
                queue.remove(&(distances[neighbour_vertex], neighbour_vertex));

                distances[neighbour_vertex] = distances[current_vertex] + neighbour_distance;

                queue.insert((distances[neighbour_vertex], neighbour_vertex));
            }
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn small_test() {
        let graph = read_graph(
            r"1 2,1 3,4
2 3,2 4,6
3 4,3",
        );
        dbg!(&graph);
        dbg!(dijkstras_algorithm(&graph, 0));
    }

    #[test]
    fn exercise() {
        let mut file = std::fs::File::open("./data/dijkstra_data.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let graph = read_graph(&buffer);
        dbg!(&graph);

        let distances = dijkstras_algorithm(&graph, 0);
        dbg!(&distances);
        let res = [7, 37, 59, 82, 99, 115, 133, 165, 188, 197]
            .into_iter()
            .map(|x| distances[x - 1])
            .collect::<Vec<usize>>();

        dbg!(&res);
    }
}
