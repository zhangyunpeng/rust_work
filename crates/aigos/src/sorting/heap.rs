use std::fmt::Debug;

fn heap_sort<T: PartialOrd + Debug>(arr: &mut [T]) {
    let n = arr.len();
    if n < 2 {
        return;
    }

    for i in (0..n / 2).rev() {
        sift_down(arr, i, n);
    }

    for end in (1..n).rev() {
        arr.swap(0, end);
        sift_down(arr, 0, end);
    }
}

fn sift_down<T: PartialOrd + Debug>(arr: &mut [T], mut root: usize, heap_size: usize) {
    loop {
        let mut largest = root;
        let left = 2 * root + 1;
        let right = 2 * root + 2;

        if left < heap_size && arr[left] > arr[largest] {
            largest = left;
        }
        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        if largest == root {
            break;
        }

        arr.swap(root, largest);
        root = largest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_heap() {
        let mut a = [4, 10, 3, 5, 1];
        println!("{:?}", a);
        heap_sort(&mut a);
        println!("{:?}", a);
        // assert_eq!(a, [1, 3, 4, 5, 10]);

        // let mut b = [5,4,3,2,1];
        // heap_sort(&mut b);
        // assert_eq!(b, [1,2,3,4,5]);
        //
        // let mut c = [1];
        // heap_sort(&mut c);
        // assert_eq!(c, [1]);
        //
        // let mut d = [2,2,1,3,2];
        // heap_sort(&mut d);
        // assert_eq!(d, [1,2,2,2,3]);
    }
}
