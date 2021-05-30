pub trait Sorter {
    fn sort<T: Ord>(slice: &mut [T]);
}

mod bubblesort;
mod heapsort;
mod quicksort;
