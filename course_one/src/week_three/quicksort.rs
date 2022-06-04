use rand::prelude::*;

fn choose_pivot<T: Ord>(a: &[T]) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..a.len())
}

fn partition<T: Ord>(a: &mut [T]) -> usize {
    let p = choose_pivot(a); // index of the pivot element
    a.swap(0, p); // bring pivot to the first place
    let p = 0; // reassign pivot index to the corrected value

    let mut i = 1; // boundary between less-than-index and greater-than-index number
    let mut flag = false; // check if we've seen any greater-than-pivot elements

    for j in 1..a.len() {
        // the boundary between seen and unseen elements
        if a[j] > a[p] {
            // if the element is greater than the pivot
            if !flag {
                // if the flag is not set, set it
                flag = true;
            }
            continue; // and do nothing else
        }
        // if the element is lesser than the pivot
        if flag {
            // if we've seen any elements greater than the pivot
            a.swap(i, j); // swap the current element with the element at the lesser-greater boundary
        }
        i += 1; // move the lesser-greater boundary by 1
    }

    a.swap(i - 1, p); // bring the pivot element to the lesser-greater than boundary
    i - 1 // return the corrected index of the pivot
}

fn quicksort<T: Ord>(a: &mut [T]) {
    if a.len() <= 1 {
        // in the base case of quicksort
        return; // we do nothing
    }

    let pivot_idx = partition(a); // partition the array
    let (left, pivot_right) = a.split_at_mut(pivot_idx); // split off the two sides of the array
    let (_, right) = pivot_right.split_at_mut(1); // remove the pivo from the right side

    quicksort(left); // recursively quicksort of the left side
    quicksort(right); // recursively quicksort of the right side
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        fn helper(a: &mut [u32]) {
            // dbg!(&a);
            let p = partition(a);
            // dbg!(&a);

            assert!(a[..p].iter().all(|&x| x < a[p]));
            assert!(a[p + 1..].iter().all(|&x| x > a[p]));
        }

        helper(&mut [1, 2, 3, 4, 5]);
        helper(&mut [3, 2, 4, 5, 1]);
        helper(&mut [5, 4, 3, 2, 1]);
        helper(&mut [4, 1, 5, 2, 3]);
    }

    #[test]
    fn test_quicksort() {
        fn helper(a: &mut [u32]) {
            quicksort(a);
            // dbg!(&a);

            assert!(a.iter().zip(a.iter().skip(1)).all(|(x, y)| x <= y));
        }

        helper(&mut [1, 2, 3, 4, 5]);
        helper(&mut [3, 2, 4, 5, 1]);
        helper(&mut [5, 4, 3, 2, 1]);
        helper(&mut [4, 1, 5, 2, 3]);
    }
}
