pub trait Sorter {
    fn sort<T: Ord>(slice: &mut [T]);
}

pub fn sort<T: Ord, S: Sorter>(slice: &mut [T]) {
    S::sort(slice);
}

mod bubblesort;
