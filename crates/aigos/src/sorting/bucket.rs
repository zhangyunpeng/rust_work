/*
   桶排序
   桶索引计算规则： index = ((val - min) / (max - min)) * (bucketCount - 1)
*/

fn bucket_sort(arr: &mut [usize]) {
    if arr.len() <= 1 {
        return;
    }

    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();

    let bucket_count = arr.len();
    let mut buckets = vec![vec![]; bucket_count];

    let range = (max - min) as f64;
    for &val in arr.iter() {
        let idx = if range == 0.0 {
            bucket_count - 1
        } else {
            (((val - min) as f64 / range) * (bucket_count - 1) as f64) as usize
        };
        buckets[idx].push(val);
    }

    for bucket in buckets.iter_mut() {
        bucket.sort_unstable();
    }

    let mut pos = 0;
    for bucket in buckets {
        for v in bucket {
            arr[pos] = v;
            pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bucket() {
        let mut a = [7, 3, 15, 2, 9, 6];
        bucket_sort(&mut a);
        assert_eq!(a, [2, 3, 6, 7, 9, 15]);

        let mut b = [1];
        bucket_sort(&mut b);
        assert_eq!(b, [1]);

        let mut c = [5, 5, 5, 5];
        bucket_sort(&mut c);
        assert_eq!(c, [5, 5, 5, 5]);
    }
}
