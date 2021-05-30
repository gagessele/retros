pub struct HeapSort;

use super::Sorter;

impl HeapSort {
    fn parent(index: usize) -> usize {
        (index - 1) / 2
    }

    fn left_child(index: usize) -> usize {
        2 * index + 1
    }

    fn sift_down<T: Ord>(slice: &mut [T], start: usize, end: usize) {
        let mut root = start;

        let mut left_child = Self::left_child(root);

        while left_child <= end {
            let mut swap = root;
            let right_child = left_child + 1;
            if slice[swap] < slice[left_child] {
                swap = left_child;
            }

            if left_child + 1 <= end && slice[swap] < slice[right_child] {
                swap = right_child;
            }

            if swap == root {
                return;
            } else {
                slice.swap(root, swap);
                root = swap;
                left_child = Self::left_child(swap);
            }
        }
    }

    fn heapify<T: Ord>(slice: &mut [T]) {
        let len = slice.len() - 1;
        let mut index = Self::parent(len);

        while index != 0 {
            Self::sift_down(slice, index, len);
            index -= 1;
        }
    }
}

impl Sorter for HeapSort {
    fn sort<T: Ord>(slice: &mut [T]) {
        Self::heapify(slice);

        let mut end = slice.len() - 1;
        while end != 0 {
            slice.swap(end, 0);

            end -= 1;

            Self::sift_down(slice, 0, end);
        }
    }
}

#[test]
fn test_heapsort() {
    let mut array = vec![9, 1, 3, 6, 7, 2, 5, 8, 4];
    HeapSort::sort(&mut array);
    assert_eq!(array, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
