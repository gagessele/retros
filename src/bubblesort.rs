pub struct BubbleSort;

use super::Sorter;

impl Sorter for BubbleSort {
    fn sort<T: Ord>(slice: &mut [T]) {
        let mut len = slice.len();
        let mut index = len;
        while index > 1 {
            for i in 1..len {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    index = i;
                }
            }
            len = index;
        }
    }
}

#[test]
fn test_bubblesort() {
    let mut array = vec![5, 4, 3, 2, 1];
    BubbleSort::sort(&mut array);
    assert_eq!(array, vec![1, 2, 3, 4, 5]);
}
