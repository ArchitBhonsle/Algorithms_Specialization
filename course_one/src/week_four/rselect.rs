use rand::prelude::*;

fn choose_pivot(a: &[u64]) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..a.len())
}

fn partition(a: &mut [u64]) -> usize {
    let p = choose_pivot(a);
    a.swap(0, p);
    let p = 0;

    let mut i = 1;
    let mut flag = false;

    for j in 1..a.len() {
        if a[j] > a[p] {
            if !flag {
                flag = true;
            }
            continue;
        }
        if flag {
            a.swap(i, j);
        }
        i += 1;
    }

    a.swap(i - 1, p);
    i - 1
}

fn rselect(a: &mut [u64], i: usize) -> u64 {
    if a.len() == 1 {
        return a[0];
    }

    let p = partition(a); // pivot index
    dbg!((&a, i, p));

    if i == p {
        return a[p];
    } else if i < p {
        return rselect(&mut a[..p], i);
    } else {
        return rselect(&mut a[p + 1..], i - p - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rselect() {
        fn helper(a: &[u64], i: usize) {
            let mut x = Vec::from(a.clone());
            x.sort();
            let exp = x[i];

            let mut y = Vec::from(a.clone());
            let res = rselect(&mut y, i);

            dbg!(&a, i, &y);
            assert_eq!(exp, res);
        }

        helper(&[1, 2, 3, 4, 5], 0);
        helper(&[3, 2, 4, 5, 1], 1);
        helper(&[5, 4, 3, 2, 1], 2);
        helper(&[4, 1, 5, 2, 3], 3);
        helper(&[4, 2, 3, 1, 5], 4);
    }
}
