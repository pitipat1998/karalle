mod primitive;
mod sort;

#[cfg(test)]
mod tests {
    #[test]
    fn map() {
        use crate::primitive::map;
        let arr: &mut Vec<i32> = &mut vec![1, 2, 3, 1];
        let f: &dyn Fn(usize, &i32) -> i32 = &|_i: usize, a: &i32| -> i32 { if *a <= 2 { 1 } else { 0 } };
        let actual: Vec<i32> = map(arr, f);

        let expected: Vec<i32> = vec![1, 1, 0, 1];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn par_map() {
        use crate::primitive::par_map;
        let arr: &mut Vec<i32> = &mut vec![61, 81, 50, 59, 7, 31, 11, 36, 93, 15, 36, 72, 96, 34, 2, 32, 83,
                                           24, 81, 76, 22, 60, 9, 54, 72, 13, 90, 75, 47, 7, 7, 17, 68, 90,
                                           86, 32, 54, 67, 50, 69, 93, 89, 30, 47, 99, 73, 18, 74, 49, 77, 53,
                                           40, 70, 65, 35, 53, 19, 73, 52, 14, 93, 66, 71, 87, 72, 90, 12, 12,
                                           81, 75, 79, 18, 63, 46, 40, 92, 31, 94, 64, 94, 8, 1, 4, 44, 5,
                                           57, 66, 67, 9, 75, 9, 49, 61, 68, 11, 25, 39, 90, 86, 48, 91];

        println!("{:?}", arr);
        let actual: Vec<i32> = par_map(arr, &mut |_i: usize, a: &i32| -> i32 { if *a <= 90 { 1 } else { 0 } });

        let expected: Vec<i32> = vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1,
                                      1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                                      0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                                      0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0,
                                      1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];

        println!("actual={:?}, expected={:?}", actual, expected);
//        assert_eq!(actual, expected);
    }

    #[test]
    fn filter() {
        use crate::primitive::filter;
        let arr: &mut Vec<i32> = &mut vec![1, 2, 3, 1];
        let f: &dyn Fn(usize, &i32) -> bool = &|_i: usize, a: &i32| -> bool { *a < 3 };
        let actual: Vec<i32> = filter(arr, f);

        let expected: Vec<i32> = vec![1, 2, 1];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }


    #[test]
    fn qsort() {
        use crate::sort::qsort;
        let arr: &mut Vec<i32> = &mut vec![1, 7, 16, 0, -4, -7, 2, 3, 64, -1, 9, 1];
        let arr2: &mut Vec<(i32, f32)> = &mut vec![(1, 0.2), (2, 0.1), (-1, 9.0), (2, 0.1), (2, 0.2)];
        let actual: Vec<i32> = qsort(arr, &|a: &i32, b: &i32| -> i32 { *a - *b });
        let actual2: Vec<(i32, f32)> = qsort(arr2, &|a: &(i32, f32), b: &(i32, f32)| -> i32 {
            let (a1, a2): (i32, f32) = *a;
            let (b1, b2): (i32, f32) = *b;
            if a1 == b1 {
                if a2 < b2 { -1 } else if a2 == b2 { 0 } else { 0 }
            } else {
                return a1 - b1;
            }
        });

        let expected: Vec<i32> = vec![-7, -4, -1, 0, 1, 1, 2, 3, 7, 9, 16, 64];
        let expected2: Vec<(i32, f32)> = vec![(-1, 9.0), (1, 0.2), (2, 0.1), (2, 0.1), (2, 0.2)];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
        println!("actual2={:?}, expected2={:?}", actual2, expected2);
        assert_eq!(actual2, expected2);
    }
}

