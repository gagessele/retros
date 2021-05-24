pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice);
}

mod bubblesort;
