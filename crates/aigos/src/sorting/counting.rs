pub fn counting_sort(arr: &mut [usize]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();

    let range = max - min + 1;
    let mut count = vec![0; range];

    for &v in arr.iter() {
        count[v - min] += 1;
    }

    let mut pos = 0;
    for (idx, &cnt) in count.iter().enumerate() {
        let val = min + idx;
        for _ in 0..cnt {
            arr[pos] = val;
            pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let mut a = [7, 3, 5, 3, 1, 4];
        counting_sort(&mut a);
        assert_eq!(a, [1, 3, 3, 4, 5, 7]);

        let mut b = [5, 2, 3, 2, 0];
        counting_sort(&mut b);
        assert_eq!(b, [0, 2, 2, 3, 5]);

        let mut c = [1];
        counting_sort(&mut c);
        assert_eq!(c, [1]);
    }
}
