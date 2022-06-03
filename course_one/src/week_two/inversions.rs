use std::{fs::File, io::Read};

fn read_file(name: &str) -> Vec<usize> {
    let mut file = File::open(format!("./src/week_two/{}", name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.lines().map(|x| x.parse().unwrap()).collect()
}

fn merge_and_count_inversions(nums: &mut [usize]) -> usize {
    let n = nums.len();
    let (one, two) = (&nums[..n / 2], &nums[n / 2..]);

    let mut count = 0;
    let mut res = Vec::new();
    res.reserve(n);

    let (mut i, mut j) = (0, 0);
    loop {
        if one[i] < two[j] {
            res.push(one[i]);
            i += 1;
        } else {
            res.push(two[j]);
            j += 1;
            count += n / 2 - i;
        }

        if i >= one.len() || j >= two.len() {
            break;
        }
    }

    while i < one.len() {
        res.push(one[i]);
        i += 1;
    }
    while j < two.len() {
        res.push(two[j]);
        j += 1;
    }

    nums.iter_mut()
        .zip(res.into_iter())
        .for_each(|(n, r)| *n = r);

    count
}

fn count_inversions(nums: &mut [usize]) -> usize {
    let n = nums.len();
    if n == 1 {
        0
    } else {
        let x = count_inversions(&mut nums[..n / 2]);
        let y = count_inversions(&mut nums[n / 2..]);
        let z = merge_and_count_inversions(nums);

        x + y + z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        read_file("integers.txt");
    }

    #[test]
    fn test_count_inversions_small() {
        let mut array = [1, 3, 5, 2, 4, 6];
        assert_eq!(count_inversions(&mut array), 3);

        let mut array = [1, 5, 3, 2, 4];
        assert_eq!(count_inversions(&mut array), 4);

        let mut array = [5, 4, 3, 2, 1];
        assert_eq!(count_inversions(&mut array), 10);

        let mut array = [1, 6, 3, 2, 4, 5];
        assert_eq!(count_inversions(&mut array), 5);

        let mut array = [9, 12, 3, 1, 6, 8, 2, 5, 14, 13, 11, 7, 10, 4, 0];
        assert_eq!(count_inversions(&mut array), 56);

        let mut array = [
            37, 7, 2, 14, 35, 47, 10, 24, 44, 17, 34, 11, 16, 48, 1, 39, 6, 33, 43, 26, 40, 4, 28,
            5, 38, 41, 42, 12, 13, 21, 29, 18, 3, 19, 0, 32, 46, 27, 31, 25, 15, 36, 20, 8, 9, 49,
            22, 23, 30, 45,
        ];
        assert_eq!(count_inversions(&mut array), 590);

        let mut array = [
            4, 80, 70, 23, 9, 60, 68, 27, 66, 78, 12, 40, 52, 53, 44, 8, 49, 28, 18, 46, 21, 39,
            51, 7, 87, 99, 69, 62, 84, 6, 79, 67, 14, 98, 83, 0, 96, 5, 82, 10, 26, 48, 3, 2, 15,
            92, 11, 55, 63, 97, 43, 45, 81, 42, 95, 20, 25, 74, 24, 72, 91, 35, 86, 19, 75, 58, 71,
            47, 76, 59, 64, 93, 17, 50, 56, 94, 90, 89, 32, 37, 34, 65, 1, 73, 41, 36, 57, 77, 30,
            22, 13, 29, 38, 16, 88, 61, 31, 85, 33, 54,
        ];
        assert_eq!(count_inversions(&mut array), 2372);
    }

    #[test]
    fn test_count_inversions() {
        let mut vec = read_file("inversions_input.txt");
        dbg!(count_inversions(&mut vec));
        assert!(vec.windows(2).all(|w| w[0] <= w[1]));
    }
}
