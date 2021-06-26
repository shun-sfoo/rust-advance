//选择排序（Selection sort）是一种简单直观的排序算法。
//它的工作原理如下。首先在未排序序列中找到最小（大）元素，
//存放到排序序列的起始位置，然后，再从剩余未排序元素中继续寻找最小（大）元素，
//然后放到已排序序列的末尾。以此类推，直到所有元素均排序完毕。
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min = i;
        for j in i + 1..arr.len() {
            if arr[min] > arr[j] {
                min = j;
            }
        }
        arr.swap(min, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection() {
        let mut arr = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], arr);
    }
}
