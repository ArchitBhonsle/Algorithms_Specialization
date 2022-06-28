#[derive(Debug, Clone)]
struct Point {
    i: usize,
    x: f64,
    y: f64,
}

fn euclidean_squared(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            let mut split = line.split_whitespace();

            Point {
                i: split.next().unwrap().parse::<usize>().unwrap() - 1,
                x: split.next().unwrap().parse::<f64>().unwrap(),
                y: split.next().unwrap().parse::<f64>().unwrap(),
            }
        })
        .collect()
}

fn tsp(points: &[Point]) -> f64 {
    let mut res = 0.;

    let mut mask = vec![true; points.len()];
    mask[0] = false;
    let mut last = points[0].clone();
    let mut count = 1;

    while count < points.len() {
        let jump_to = points
            .iter()
            .filter(|p| mask[p.i])
            .min_by(|x, y| {
                let dx = euclidean_squared(x, &last);
                let dy = euclidean_squared(y, &last);

                if dx == dy {
                    x.i.cmp(&y.i)
                } else {
                    dx.partial_cmp(&dy).unwrap()
                }
            })
            .unwrap();

        res += euclidean_squared(jump_to, &last);
        last = jump_to.clone();
        mask[last.i] = false;
        count += 1;

        // dbg!(jump_to.i, count, res);
    }

    assert!(mask.iter().all(|x| !x));

    res += euclidean_squared(&last, &points[0]);

    res
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    const T1: &str = r"6
1 2 1
2 4 0
3 2 0
4 0 0
5 4 3
6 0 3";

    #[test]
    fn test() {
        fn helper(input: &str, exp: f64) {
            let points = parse_input(input);
            let res = tsp(&points);
            dbg!(res);

            assert_eq!(res.floor(), exp.floor());
        }

        helper(T1, 15.2361);
    }

    #[test]
    fn exercise() {
        let mut file = File::open("data/nn.txt").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        let points = parse_input(&input);
        let res = tsp(&points);
        dbg!(res);
    }
}
