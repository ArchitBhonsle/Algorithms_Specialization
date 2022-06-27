use std::fmt;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone)]
struct Set(usize);

impl Set {
    fn is_set(&self, n: usize) -> bool {
        self.0 & (1 << n) != 0
    }

    fn is_reset(&self, n: usize) -> bool {
        self.0 & (1 << n) == 0
    }

    fn set(&mut self, n: usize) {
        self.0 |= 1 << n;
    }

    fn reset(&mut self, n: usize) {
        self.0 &= !(1 << n);
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Set({:b})", self.0)
    }
}

// Generate a vector of vector of sets of n items that contain the first item
// grouped by the number of items in them
fn generate_setss(n: usize) -> Vec<Vec<Set>> {
    let mut res = vec![vec![]; n + 1];

    for s in 0..=(1 << n) {
        let set = Set(s);
        if set.is_set(0) {
            res[set.0.count_ones() as usize].push(set);
        }
    }

    res
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            let mut split = line.split_whitespace();

            Point {
                x: split.next().unwrap().parse::<f32>().unwrap(),
                y: split.next().unwrap().parse::<f32>().unwrap(),
            }
        })
        .collect()
}

fn distance(a: &Point, b: &Point) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn generate_distances(points: &[Point]) -> Vec<f32> {
    let n = points.len();
    let mut distances = vec![0.; n * n];

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let d = distance(&points[i], &points[j]);
            distances[i * n + j] = d;
            distances[j * n + i] = d;
        }
    }

    distances
}

fn tsp(points: &[Point]) -> f32 {
    let n = points.len();

    let distances = generate_distances(&points);
    // dbg!(&distances);
    println!("distances generated");

    let setss = generate_setss(n);
    // dbg!(&setss);
    println!("setss generated");

    // let mut cache: HashMap<(Set, usize), f64> = HashMap::new();
    let mut cache = vec![vec![6.9; n]; 1 << n];

    for s in 0..(1 << n) {
        if s == 1 {
            cache[s][0] = 0.;
        } else {
            cache[s][0] = f32::INFINITY;
        }
    }

    // dbg!(&cache);
    println!("cache initialised");

    for m in 2..=n {
        println!("on {m}");
        for set in setss[m].iter() {
            for j in 0..n {
                if j == 0 || set.is_reset(j) {
                    continue;
                }

                let mut min = f32::INFINITY;

                for k in 0..n {
                    if k == j || set.is_reset(k) {
                        continue;
                    }

                    let mut smaller_set = set.clone();
                    smaller_set.reset(j);

                    min = min.min(cache[smaller_set.0][k] + distances[k * n + j]);
                }
                cache[set.0][j] = min;
            }
        }
    }

    println!("cache filled");

    // dbg!(&cache);

    let mut res = f32::INFINITY;

    for j in 1..n {
        res = res.min(cache[setss[n][0].0][j] + distances[j]);
    }

    res
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_set() {
        let mut x = Set(8);
        assert_eq!(x.0, 0b1000);
        assert_eq!(x.is_set(3), true);
        assert_eq!(x.is_reset(0), true);

        x.reset(3);
        x.set(0);
        assert_eq!(x.is_set(0), true);
        assert_eq!(x.is_reset(3), true);
    }

    const T1: &str = r"3
0 0
0 3
3 3";

    const T2: &str = r"8
0 2.05
3.414213562373095 3.4642135623730947
0.5857864376269049 0.6357864376269047
0.5857864376269049 3.4642135623730947
2 0
4.05 2.05
2 4.10
3.414213562373095 0.6357864376269047";

    const T3: &str = r"4
0 0
4 3
4 0
0 3";

    #[test]
    fn test_tsp() {
        fn helper(res: f32, exp: f32) {
            assert!((res - exp).abs() < 0.1)
        }

        helper(tsp(&parse_input(T1)), 10.24);
        helper(tsp(&parse_input(T2)), 12.36);
        helper(tsp(&parse_input(T3)), 14.00);
    }

    #[test]
    fn exercise() {
        let mut file = File::open("data/tsp.txt").unwrap();
        let mut input = String::new();

        file.read_to_string(&mut input).unwrap();

        let points = parse_input(&input);
        // dbg!(&points);

        dbg!(tsp(&points));
    }
}

