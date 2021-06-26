struct SegmentTree<T> {
    data: Vec<T>,
    tree: Vec<T>,
}

trait Merge<T> {
    fn merge(a: T, b: T);
}

impl<T> SegmentTree<T> {
    fn left_child(index: usize) -> u32 {
        index as u32 * 2 + 1
    }

    fn right_child(index: usize) -> u32 {
        index as u32 * 2 + 2
    }
}
