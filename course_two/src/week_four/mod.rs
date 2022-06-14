use std::collections::HashSet;

const LOWER_BOUND: isize = -10000;
const UPPER_BOUND: isize = 10000;

fn two_sum(mut numbers: Vec<isize>) -> usize {
    numbers.sort();

    let mut found_targets = HashSet::new();
    let (mut left, mut right) = (0, numbers.len() - 1);
    while left < right {
        let max_sum = numbers[left] + numbers[right];
        if max_sum < LOWER_BOUND {
            left += 1;
            continue;
        }
        if max_sum > UPPER_BOUND {
            right -= 1;
            continue;
        }
        for curr in (left + 1)..=right {
            if numbers[left] == numbers[curr] {
                continue;
            }
            let sum = numbers[left] + numbers[curr];
            if sum > UPPER_BOUND {
                break;
            }
            if sum < LOWER_BOUND {
                continue;
            }

            found_targets.insert(sum);
        }

        left += 1;
    }

    found_targets.len()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn small() {
        dbg!(two_sum(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn exercise() {
        let mut file = File::open("data/2sum.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let numbers = buffer
            .lines()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<isize>>();

        dbg!(two_sum(numbers));
    }
}
