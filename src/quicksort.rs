pub struct QuickSort;

use super::Sorter;

impl QuickSort {
    fn partition<T: Ord>(slice: &mut [T], lo: usize, hi: usize) -> usize {
        let mut i: usize = lo;

        for j in lo..hi {
            if slice[j] < slice[hi] {
                slice.swap(i, j);
                i += 1;
            }
        }

        slice.swap(i, hi);
        i
    }

    fn quicksort<T: Ord>(slice: &mut [T], lo: usize, hi: usize) {
        if lo >= hi {
            return;
        }

        let pivot: usize = Self::partition(slice, lo, hi);
        Self::quicksort(slice, lo, pivot - 1);
        Self::quicksort(slice, pivot + 1, hi);
    }
}

impl Sorter for QuickSort {
    fn sort<T: Ord>(slice: &mut [T]) {
        Self::quicksort(slice, 0, slice.len() - 1);
    }
}

#[test]
fn test_quicksort() {
    let mut array = vec![9, 1, 3, 6, 7, 2, 5, 8, 4];
    QuickSort::sort(&mut array);
    assert_eq!(array, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
