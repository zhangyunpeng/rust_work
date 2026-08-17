pub fn insert_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod insertion_sort {
        use super::*;

        #[test]
        fn test_empty_vec() {
            let mut empty_vec: Vec<String> = vec![];
            insert_sort(&mut empty_vec);
            assert_eq!(empty_vec, Vec::<String>::new());
        }

        #[test]
        fn test_number_vec() {
            let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
            insert_sort(&mut vec);
            assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
        }

        #[test]
        fn test_string_vec() {
            let mut vec = vec![
                String::from("Bob"),
                String::from("David"),
                String::from("Carol"),
                String::from("Alice"),
            ];
            insert_sort(&mut vec);
            assert_eq!(
                vec,
                vec![
                    String::from("Alice"),
                    String::from("Bob"),
                    String::from("Carol"),
                    String::from("David"),
                ]
            );
        }
    }
}
