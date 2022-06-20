use std::collections::HashMap;

fn knapsack(v: &[usize], w: &[usize], c: usize) -> usize {
    assert_eq!(v.len(), w.len());

    let mut dp = vec![vec![0; c + 1]; v.len() + 1];

    for i in 1..=v.len() {
        let (cw, cv) = (w[i - 1], v[i - 1]); // the weights and values are off by one
        for x in 0..=c {
            if w[i - 1] > x {
                dp[i][x] = dp[i - 1][x];
            } else {
                dp[i][x] = usize::max(
                    dp[i - 1][x],           // without x
                    dp[i - 1][x - cw] + cv, // with x
                );
            }
        }
    }

    dp[v.len()][c]
}

fn knapsack_top_down(v: &[usize], w: &[usize], c: usize) -> usize {
    assert_eq!(v.len(), w.len());

    let mut cache = HashMap::new();

    fn recurse(i: usize, x: usize, v: &[usize], w: &[usize], cache: &mut HashMap<(usize, usize), usize>) -> usize {
        if i == 0 {
            return 0;
        }

        if cache.contains_key(&(i, x)) {
            return cache[&(i, x)];
        }

        let without_x = recurse(i - 1, x, v, w, cache);
        if w[i - 1] > x {
            cache.insert((i, x), without_x);
            return without_x;
        } else {
            let with_x = recurse(i - 1, x - w[i - 1], v, w, cache) + v[i - 1];
            let result = usize::max(without_x, with_x);

            cache.insert((i, x), result);
            return result;
        }
    }

    recurse(v.len(), c, v, w, &mut cache)
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>, usize) {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let c = first_line
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let n = first_line
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let (mut v, mut w) = (Vec::with_capacity(n), Vec::with_capacity(n));
    for line in lines {
        let mut split = line.split_whitespace();
        let vi = split.next().unwrap().parse::<usize>().unwrap();
        let wi = split.next().unwrap().parse::<usize>().unwrap();

        v.push(vi);
        w.push(wi);
    }

    (v, w, c)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn small() {
        let v = vec![3, 2, 4, 4];
        let w = vec![4, 3, 2, 3];
        let c = 6;

        let res = knapsack(&v, &w, c);
        let res_top_down = knapsack_top_down(&v, &w, c);

        assert_eq!(res, 8);
        assert_eq!(res_top_down, 8);
    }

    #[test]
    fn exercise_1() {
        let mut input = String::new();
        File::open("data/knapsack.txt")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();

        let (v, w, c) = parse_input(&input);
        let res = knapsack(&v, &w, c);

        dbg!(res);
    }

    #[test]
    fn exercise_2() {
        let mut input = String::new();
        File::open("data/knapsack_big.txt")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();

        let (v, w, c) = parse_input(&input);
        let res = knapsack_top_down(&v, &w, c);

        dbg!(res);
    }
}
