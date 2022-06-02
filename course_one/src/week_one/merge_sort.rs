fn merge(x: &[i32], y: &[i32]) -> Vec<i32> {
    let n = x.len() + y.len();
    let mut result = Vec::new();
    result.reserve_exact(n);

    let (mut i, mut j) = (0, 0);
    while i < x.len() && j < y.len() {
        if x[i] < y[j] {
            result.push(x[i]);
            i += 1;
        } else {
            result.push(y[j]);
            j += 1;
        }
    }

    while i < x.len() {
        result.push(x[i]);
        i += 1
    }

    while j < y.len() {
        result.push(y[j]);
        j += 1
    }

    result
}

fn merge_sort(a: &[i32]) -> Vec<i32> {
    let n = a.len();
    if n == 1 {
        return a.into();
    }

    let (x, y) = (&a[..n / 2], &a[n / 2..]);
    let x_sorted = merge_sort(x);
    let y_sorted = merge_sort(y);

    merge(&x_sorted, &y_sorted)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge_test() {
        let x = vec![1, 3, 5, 7];
        let y = vec![2, 4, 6];

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], merge(&x, &y));
    }

    #[test]
    fn merge_sort_test() {
        let x = vec![2, 5, 3, 1, 4];
        assert_eq!(vec![1, 2, 3, 4, 5], merge_sort(&x));
    }
}
