fn bubble_sort(arr: &mut Vec<i32>) {
    let mut n = arr.len();
    while n > 0 {
        let (mut i, mut max_ptr) = (1, 0);
        while i < n {
            if arr[i - 1] > arr[i] {
                arr.swap(i, i - 1);
                max_ptr = i;
            }
            i += 1;
        }
        n = max_ptr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut a = vec![1, 4, 5, 3, 2];
        bubble_sort(&mut a);
        assert_eq!(vec![1, 2, 3, 4, 5], a);
    }
}
