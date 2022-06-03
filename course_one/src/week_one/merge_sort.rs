fn merge<T: Ord + Copy>(nums: &mut [T]) {
    let n = nums.len();
    let (left, right) = (&nums[..n / 2], &nums[n / 2..]);

    let mut res = Vec::with_capacity(n);

    let (mut i, mut j) = (0, 0);
    loop {
        if left[i] < right[j] {
            res.push(left[i]);
            i += 1;
        } else {
            res.push(right[j]);
            j += 1;
        }

        if i >= left.len() || j >= right.len() {
            break;
        }
    }

    while i < left.len() {
        res.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        res.push(right[j]);
        j += 1;
    }

    nums.copy_from_slice(&res);
}

fn merge_sort<T: Ord + Copy>(nums: &mut [T]) {
    let n = nums.len();
    if n != 1 {
        merge_sort(&mut nums[..n / 2]);
        merge_sort(&mut nums[n / 2..]);
        merge(nums);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge_test() {
        let mut nums = vec![
            1, 3, 5, // left
            2, 4, 6, 7, // right
        ];
        merge(&mut nums);

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], nums);
    }

    #[test]
    fn merge_sort_test() {
        let mut nums = vec![2, 5, 3, 1, 4];
        merge_sort(&mut nums);

        assert_eq!(vec![1, 2, 3, 4, 5], nums);
    }
}
