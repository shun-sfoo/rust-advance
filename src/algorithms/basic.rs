//! 基础算法

/**
 * # 选择排序
 *
 * 选择排序（Selection sort）是一种简单直观的排序算法。
 * 它的工作原理如下。首先在未排序序列中找到最小（大）元素，
 * 存放到排序序列的起始位置，然后，再从剩余未排序元素中继续寻找最小（大）元素，
 * 然后放到已排序序列的末尾。以此类推，直到所有元素均排序完毕。
 *
 */
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut min = i;
        for j in i + 1..len {
            if arr[min] > arr[j] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}

/**
 * # 插入排序
 *
 * 插入排序（英语：Insertion Sort）是一种简单直观的排序算法。
 * 它的工作原理是通过构建有序序列，对于未排序数据，在已排序序列中从后向前扫描，
 * 找到相应位置并插入。插入排序在实现上，通常采用in-place排序
 * 即只需用到 O ( 1 ) {\displaystyle O(1)} {\displaystyle O(1)}的额外空间的排序），
 * 因而在从后向前扫描过程中，需要反复把已排序元素逐步向后挪位，为最新元素提供插入空间。
 */
pub fn insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    for i in 1..len {
        let elem = arr[i].clone();
        let mut j = i;
        while j > 0 && arr[j - 1] > elem {
            arr.swap(j, j - 1);
            j -= 1;
        }
        arr[j] = elem;
    }
}

/**
 * # 希尔排序
 * 希尔排序（Shellsort），也称递减增量排序算法，是插入排序的一种更高效的改进版本。
 * 希尔排序是非稳定排序算法。
 */
pub fn shell_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    let mut h = len / 2;

    while h >= 1 {
        for i in (h..len).step_by(h) {
            let elem = arr[i].clone();
            let mut j = i;
            while j >= h && arr[j - h] > elem {
                arr.swap(j, j - h);
                j -= h;
            }
            arr[j] = elem;
        }
        h /= 2;
    }
}

/**
 * # 冒泡排序
 *
 * 冒泡排序（英语：Bubble Sort）又称为泡式排序，是一种简单的排序算法。
 * 它重复地走访过要排序的数列，一次比较两个元素，如果他们的顺序错误就把他们交换过来。
 * 走访数列的工作是重复地进行直到没有再需要交换，也就是说该数列已经排序完成。
 * 这个算法的名字由来是因为越小的元素会经由交换慢慢“浮”到数列的顶端。
*/

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let mut len = arr.len();
    while len > 0 {
        let (mut i, mut max) = (1, 0);
        while i < len {
            if arr[i - 1] > arr[i] {
                arr.swap(i, i - 1);
                max = i;
            }
            i += 1;
        }
        len = max;
    }
}

#[test]
fn test_insertion() {
    let mut arr = vec![1, 3, 2, 6, 4, 5];
    insertion_sort(&mut arr);
    let result = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(result, arr);
}

#[test]
fn test_selection() {
    let mut arr = vec![1, 3, 2, 6, 4, 5];
    selection_sort(&mut arr);
    let result = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(result, arr);
}

#[test]
fn test_shell() {
    let mut arr = vec![1, 3, 2, 6, 4, 5];
    shell_sort(&mut arr);
    let result = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(result, arr);
}

#[test]
fn test_bubble() {
    let mut arr = vec![1, 3, 2, 6, 4, 5];
    bubble_sort(&mut arr);
    let result = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(result, arr);
}
