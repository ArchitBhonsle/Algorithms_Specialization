type Pivoter = fn(a: &[u32]) -> usize;

fn pivot_first(_a: &[u32]) -> usize {
    0
}
fn pivot_last(a: &[u32]) -> usize {
    a.len() - 1
}
fn pivot_median(a: &[u32]) -> usize {
    let indices = [0, (a.len() - 1) / 2, a.len() - 1];
    let mut ivs = indices
        .iter()
        .map(|&x| (x, a[x]))
        .collect::<Vec<(usize, u32)>>();
    ivs.sort_by_key(|(_i, v)| *v);

    ivs[1].0
}

fn partition(a: &mut [u32], pivot: Pivoter) -> usize {
    let p = pivot(a);
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

fn quicksort(a: &mut [u32], pivot: Pivoter) -> usize {
    let n = a.len();
    if n <= 1 {
        return 0; // we do nothing
    }

    let pivot_idx = partition(a, pivot);
    let (left, pivot_right) = a.split_at_mut(pivot_idx);
    let (_, right) = pivot_right.split_at_mut(1);

    let current_comps = n - 1;

    let left_comps = quicksort(left, pivot);
    let right_comps = quicksort(right, pivot);

    current_comps + left_comps + right_comps
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_partition() {
        fn helper(a: &mut [u32]) {
            [pivot_first, pivot_last, pivot_median]
                .into_iter()
                .for_each(|x| {
                    // dbg!(&a);
                    let p = partition(a, x);
                    // dbg!(&a);

                    assert!(a[..p].iter().all(|&x| x < a[p]));
                    assert!(a[p + 1..].iter().all(|&x| x > a[p]));
                });
        }

        helper(&mut [1, 2, 3, 4, 5]);
        helper(&mut [3, 2, 4, 5, 1]);
        helper(&mut [5, 4, 3, 2, 1]);
        helper(&mut [4, 1, 5, 2, 3]);
    }

    #[test]
    fn test_quicksort() {
        fn helper(a: &mut [u32]) {
            dbg!(&a);
            [pivot_first, pivot_last, pivot_median]
                .into_iter()
                .for_each(|pivot| {
                    dbg!(quicksort(a, pivot));
                    // dbg!(&a);
                    assert!(a.iter().zip(a.iter().skip(1)).all(|(x, y)| x <= y));
                });
        }

        helper(&mut [1, 2, 3, 4, 5]);
        helper(&mut [3, 2, 4, 5, 1]);
        helper(&mut [5, 4, 3, 2, 1]);
        helper(&mut [4, 1, 5, 2, 3]);
    }

    #[test]
    fn exercise() {
        let mut file = File::open("./src/week_three/input.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let a = contents
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<u32>>();

        [pivot_first, pivot_last, pivot_median]
            .into_iter()
            .for_each(|pivot| {
                let mut clone = a.clone();
                dbg!(quicksort(&mut clone, pivot));
                // dbg!(&a);
                assert!(clone.iter().zip(clone.iter().skip(1)).all(|(x, y)| x <= y));
            });
    }
}
