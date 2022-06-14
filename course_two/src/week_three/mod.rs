use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Return the sum of the running medians mod 10000
// If length is odd, the median is (length + 1)/2 th element
// If length is even, the median is length/2 th element
fn solve(sequence: &[usize]) -> usize {
    let mut sum = 0;
    let mut lower_heap: BinaryHeap<usize> = BinaryHeap::new();
    let mut higher_heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new(); // Wrap the items in std::cmp::Reverse

    for &x in sequence {
        // lower heap will either have the same number of elements as the higher heap
        // or one more

        // if the lower heap is empty, the higher heap has to be empty too
        if lower_heap.is_empty() {
            lower_heap.push(x);
        } else {
            // the lower heap cannot be empty here

            if lower_heap.len() == higher_heap.len() {
                // lower heap has the same number of elements as the higher heap
                // we should try to insert the element in the lower heap
                // but first we check if the higher heap has a smaller element

                if !higher_heap.is_empty() && *higher_heap.peek().unwrap() > Reverse(x) {
                    let Reverse(from_high) = higher_heap.pop().unwrap();
                    lower_heap.push(from_high);
                    higher_heap.push(Reverse(x));
                } else {
                    lower_heap.push(x);
                }
            } else {
                // lower heap has one more element than the higher heap
                // so we should insert and elememt in the higher heap
                // but first we check ig the lower heap has a higher elememt

                if *lower_heap.peek().unwrap() > x {
                    let from_low = lower_heap.pop().unwrap();
                    lower_heap.push(x);
                    higher_heap.push(Reverse(from_low));
                } else {
                    higher_heap.push(Reverse(x));
                }
            }
        }

        // dbg!(&lower_heap);
        // dbg!(&higher_heap);

        // the median will be the highest element in the lower heap
        let median = lower_heap.peek().unwrap();
        sum = (sum + median) % 10000;
    }

    sum
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn small() {
        let sequence = [5, 3, 2, 1, 4];
        assert_eq!(solve(&sequence), 16);
    }

    #[test]
    fn exercise() {
        let mut file = std::fs::File::open("data/median.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let sequence = buffer.lines().map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
        dbg!(solve(&sequence));
    }
}
