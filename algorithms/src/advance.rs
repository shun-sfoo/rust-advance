use rand::prelude::*;
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    fn merge<T: Ord + Clone>(arr: &mut [T], start: usize, end: usize) {
        if start >= end {
            return;
        }

        let mid = (start + end) / 2;
        merge(arr, start, mid);
        merge(arr, mid + 1, end);

        let mut aux: Vec<T> = Vec::new();
        for i in start..=end {
            aux.push(arr[i].clone())
        }

        let mut i = start;
        let mut j = mid + 1;

        for k in start..=end {
            if i > mid {
                arr[k] = aux[j - start].clone();
                j += 1;
            } else if j > end {
                arr[k] = aux[i - start].clone();
                i += 1;
            } else if aux[i - start] < aux[j - start] {
                arr[k] = aux[i - start].clone();
                i += 1;
            } else {
                arr[k] = aux[j - start].clone();
                j += 1;
            }
        }
    }

    let (start, end) = (0, arr.len() - 1);
    merge(arr, start, end);
}

pub fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
    fn quick<T: Ord + Clone>(arr: &mut [T], start: i32, end: i32) {
        if start >= end {
            return;
        }

        let p = partition(arr, start as usize, end as usize);
        quick(arr, start, p - 1);
        quick(arr, p + 1, end);
    }

    fn partition<T: Ord + Clone>(arr: &mut [T], start: usize, end: usize) -> i32 {
        // let mut rng = thread_rng();
        // arr.swap(start, start + rng.gen_range(0..(end - start + 1)));

        let (mut i, mut j) = (start + 1, end);
        let e = arr[start].clone();

        loop {
            while i <= end && arr[i] < e {
                i += 1;
            }

            while j >= start + 1 && arr[j] > e {
                j -= 1;
            }

            if i > j {
                break;
            }

            arr.swap(i, j);
            i += 1;
            j -= 1;
        }
        arr.swap(start, j);
        j as i32
    }

    quick(arr, 0, (arr.len() - 1) as i32);
}

pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    fn shift_down<T: Ord>(arr: &mut [T], mut root: usize) {
        let last = arr.len() - 1;
        loop {
            let left = root * 2 + 1;

            if left > last {
                break;
            }

            let right = left + 1;
            let max = if right <= last && arr[right] > arr[left] {
                right
            } else {
                left
            };

            if arr[max] > arr[root] {
                arr.swap(max, root);
            }
            root = max;
        }
    }

    fn heapify<T: Ord>(arr: &mut [T]) {
        let last_parent = (arr.len() - 2) / 2;
        for i in (0..last_parent).rev() {
            shift_down(arr, i);
        }
    }

    if arr.len() <= 1 {
        return;
    }

    heapify(arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        shift_down(&mut arr[..end], 0);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::prelude::*;
    use std::time::Instant;

    #[test]
    fn merge() {
        let mut arr = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], arr);
    }

    #[test]
    fn quick() {
        let mut arr = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], arr);
    }

    #[test]
    fn heap() {
        let mut arr = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        heap_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], arr);
    }

    #[test]
    fn test_speed() {
        const N: usize = 20000;
        let mut arr: [i32; N] = [0; N];
        let mut rng = thread_rng();
        for i in 0..N {
            arr[i] = rng.gen_range(0..(N as i32));
        }

        let mut arr1 = arr.clone();
        let mut arr2 = arr.clone();

        let start = Instant::now();
        merge_sort(&mut arr);
        println!("merge {} ms", start.elapsed().as_millis());
        assert!(arr.is_sorted());

        let start = Instant::now();
        quick_sort(&mut arr1);
        println!("quick {} ms", start.elapsed().as_millis());
        assert!(arr1.is_sorted());

        let start = Instant::now();
        heap_sort(&mut arr2);
        println!("heap {} ms", start.elapsed().as_millis());
        assert!(arr2.is_sorted());
    }
}
