/*
   鸡尾酒排序
   鸡尾酒排序双向扫描：先左→右把大数冒泡到尾部，再右→左把小数冒泡到头部，来回摆动。
*/

fn cocktail_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mut left: usize = 0;
    let mut right: usize = len - 1;

    while left < right {
        let mut swapped = false;
        for i in left..right {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        right -= 1;
        if !swapped {
            break;
        }

        let mut swapped = false;
        for i in (left..right).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        left += 1;
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cocktail() {
        let mut a = [2, 3, 4, 5, 1];
        cocktail_sort(&mut a);
        assert_eq!(a, [1, 2, 3, 4, 5]);

        let mut b = [5, 1, 4, 2, 8];
        cocktail_sort(&mut b);
        assert_eq!(b, [1, 2, 4, 5, 8]);

        let mut c = [1];
        cocktail_sort(&mut c);
        assert_eq!(c, [1]);

        let mut d = [1, 2, 3, 4];
        cocktail_sort(&mut d);
        assert_eq!(d, [1, 2, 3, 4]);
    }
}
