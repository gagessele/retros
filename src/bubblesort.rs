use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
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
    super::sort::<_, BubbleSort>(&mut array);
    assert_eq!(array, &[1, 2, 3, 4, 5]);
}
