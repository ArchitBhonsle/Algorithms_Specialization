/// Returns an array of booleans indicating whether a given vertex
/// is included or not in the maximum weight independent set.
fn mwis(w: &[usize]) -> Vec<bool> {
    let n = w.len();

    let mut dp = Vec::with_capacity(n + 1);
    dp.push(0);
    dp.push(w[0]);

    for i in 2..(n + 1) {
        dp.push(usize::max(dp[i - 1], dp[i - 2] + w[i - 1]));
    }

    // reconstruction
    let mut res = vec![false; n];
    let mut i = n;

    while i >= 1 {
        if dp[i - 1] > if i == 1 { 0 } else { dp[i - 2] } + w[i - 1] {
            i -= 1;
            continue;
        }

        res[i - 1] = true;
        i -= if i == 1 { 1 } else { 2 };
    }

    res
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().skip(1).map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn small() {
        let weights = [1, 4, 5, 4];
        assert_eq!(mwis(&weights), vec![false, true, false, true]);
    }

    #[test]
    fn exercise() {
        let mut input = String::new();

        File::open("data/mwis.txt")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();

        let weights = parse_input(&input);
        let mwis = mwis(&weights);

        let sol = [1, 2, 3, 4, 17, 117, 517, 997]
            .into_iter()
            .map(|x| if mwis[x - 1] { '1' } else { '0' })
            .collect::<String>();
        dbg!(&sol);
    }
}
