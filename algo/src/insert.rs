pub fn insert_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        if arr[i - 1] > arr[i] && i > 0 {
            arr.swap(i - 1, i);
            i -= 1;
        }
    }
}
