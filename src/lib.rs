pub mod primitive;
mod sort;
mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn scan() {
        use crate::primitive::scan::scan;
        let arr: &mut Vec<i32> = &mut vec![1, 2, 1, 1];
        let (actual_arr, actual_tot): (Vec<i32>, i32) = scan(
            arr,
            |a: &i32, b: &i32| -> i32 { *a + *b },
            &0,
        );

        let expected_arr: Vec<i32> = vec![0, 1, 3, 4];
        let expected_tot: i32 = 5;

        println!("actual={:?}, expected={:?}", actual_arr, expected_arr);
        assert_eq!(actual_arr, expected_arr);
        println!("actual={:?}, expected={:?}", actual_tot, expected_tot);
        assert_eq!(actual_tot, expected_tot);
    }

    #[test]
    fn par_scan() {
        use crate::primitive::par_scan::par_scan;
        let arr: &mut Vec<i32> = &mut vec![1, 2, 1, 1, 9];
        let (actual_arr, actual_tot): (Vec<i32>, i32) = par_scan(
            arr,
            |a: &i32, b: &i32| -> i32 { *a + *b },
            &0,
        );

        let expected_arr: Vec<i32> = vec![0, 1, 3, 4, 5];
        let expected_tot: i32 = 14;

        println!("actual={:?}, expected={:?}", actual_arr, expected_arr);
        assert_eq!(actual_arr, expected_arr);
        println!("actual={:?}, expected={:?}", actual_tot, expected_tot);
        assert_eq!(actual_tot, expected_tot);
    }

    #[test]
    fn map() {
        use crate::primitive::map;
        use crate::util::read_csv;
        let arr: &mut Vec<i32> = &mut vec![1, 2, 3, 4];
        let f: &dyn Fn(usize, &i32) -> i32 = &|_i: usize, a: &i32| -> i32 { if *a <= 2 { 1 } else { 0 } };
        let actual: Vec<i32> = map(arr, f);

        let expected: Vec<i32> = vec![1, 1, 0, 0];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn par_map() {

        use crate::primitive::par_map_v1;
        use crate::primitive::par_map_v2;
        use crate::primitive::par_map_v3;
        use crate::primitive::par_map_v4;
        let arr: &Vec<i32> = &vec![61, 81, 50, 59, 7, 31, 11, 36, 93, 15, 36, 72, 96, 34, 2, 32, 83,
                                   24, 81, 76, 22, 60, 9, 54, 72, 13, 90, 75, 47, 7, 7, 17, 68, 90,
                                   86, 32, 54, 67, 50, 69, 93, 89, 30, 47, 99, 73, 18, 74, 49, 77, 53,
                                   40, 70, 65, 35, 53, 19, 73, 52, 14, 93, 66, 71, 87, 72, 90, 12, 12,
                                   81, 75, 79, 18, 63, 46, 40, 92, 31, 94, 64, 94, 8, 1, 4, 44, 5,
                                   57, 66, 67, 9, 75, 9, 49, 61, 68, 11, 25, 39, 90, 86, 48, 91];

        let actual: Vec<i32> = par_map_v1(arr, |_i: usize, a: &i32| -> i32 { if *a <= 90 { 1 } else { 0 } });
        let actual2: Vec<i32> = par_map_v2(arr, |_i: usize, a: &i32| -> i32 { if *a <= 90 { 1 } else { 0 } });
        let actual3: Vec<i32> = par_map_v3(arr, |_i: usize, a: &i32| -> i32 { if *a <= 90 { 1 } else { 0 } });
        let actual4: Vec<i32> = par_map_v4(arr, |_i: usize, a: &i32| -> i32 { if *a <= 90 { 1 } else { 0 } });

        let expected: Vec<i32> = vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1,
                                      1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                                      0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                                      0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0,
                                      1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
        println!("actual2={:?}, expected={:?}", actual2, expected);
        assert_eq!(actual2, expected);
        println!("actual3={:?}, expected={:?}", actual3, expected);
        assert_eq!(actual3, expected);
        println!("actual4={:?}, expected={:?}", actual4, expected);
        assert_eq!(actual4, expected);
    }

    #[test]
    fn filter() {
        use crate::primitive::filter;
        let arr: Vec<i32> = vec![1, 2, 3, 1];
        let actual: Vec<i32> = filter(&arr, |_i: usize, a: &i32| -> bool { *a < 3 });

        let expected: Vec<i32> = vec![1, 2, 1];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn par_filter() {
        use crate::primitive::par_filter_v1;
//        use crate::primitive::par_filter_v2;
        let arr: Vec<i32> = vec![1, 2, 3, 1, 2];
        let actual: Vec<i32> = par_filter_v1(&arr, |_i: usize, a: &i32| -> bool { *a < 3 });
//        let actual2: Vec<i32> = par_filter_v2(&arr, |_i: usize, a: &i32| -> bool { *a < 3 });

        let expected: Vec<i32> = vec![1, 2, 1, 2];
        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
//        println!("actual2={:?}, expected={:?}", actual2, expected);
//        assert_eq!(actual2, expected);
    }

    #[test]
    fn flatten() {
        use crate::primitive::flatten;
        let arr: &mut Vec<i32> = &mut vec![1, 2, 3, 4];
        let arr2: &mut Vec<i32> = &mut vec![5, 6, 7, 8];
        let arr3: &mut Vec<i32> = &mut vec![9, 10, 11, 12];
        let actual = flatten(&vec![&arr, &arr2, &arr3]);

        let expected: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }


    #[test]
    fn quick_sort() {
        use crate::sort::quick_sort;
        let arr: Vec<i32> = vec![1, 7, 16, 0, -4, -7, 2, 3, 64, -1, 9, 1];
        let arr2: Vec<(i32, f32)> = vec![(1, 0.2), (2, 0.1), (-1, 9.0), (2, 0.1), (2, 0.2)];
        let actual = quick_sort(&arr, |a: &i32, b: &i32| -> i32 { *a - *b });
        let actual2 = quick_sort(&arr2, |a: &(i32, f32), b: &(i32, f32)| -> i32 {
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

