pub mod primitive;
mod sort;
mod util;
mod benchmark;

#[cfg(test)]
mod tests {
    #[test]
    fn seq_sample_sort() {
        use crate::primitive::seq_sample_sort;
        let v: &mut Vec<i32> = &mut vec![683, 70, 196, 312, 980, 206, 366, 802, 455, 90, 79, 527, 530,
                                         587, 803, 738, 84, 907, 394, 390, 941, 644, 757, 235, 192, 317,
                                         830, 728, 865, 376, 155, 368, 586, 443, 881, 575, 456, 129, 626,
                                         526, 862, 939, 42, 141, 346, 871, 94, 195, 971, 923, 985, 559,
                                         76, 748, 383, 968, 874, 465, 521, 580, 169, 879, 828, 691, 725,
                                         373, 15, 638, 269, 570, 844, 866, 254, 748, 779, 305, 112, 159,
                                         607, 849, 887, 787, 300, 252, 356, 887, 81, 64, 92, 897, 65,
                                         449, 358, 749, 906, 618, 319, 492, 57, 632];
        let vr = seq_sample_sort::<i32>(v.as_mut_slice(), 5, 3);
        let expected = vec![
            15, 42, 57, 64, 65, 70, 76, 79, 81, 84, 90, 92, 94, 112, 129, 141, 155,
            159, 169, 192, 195, 196, 206, 235, 252, 254, 269, 300, 305, 312, 317,
            319, 346, 356, 358, 366, 368, 373, 376, 383, 390, 394, 443, 449, 455, 456,
            465, 492, 521, 526, 527, 530, 559, 570, 575, 580, 586, 587, 607, 618,
            626, 632, 638, 644, 683, 691, 725, 728, 738, 748, 748, 749, 757, 779,
            787, 802, 803, 828, 830, 844, 849, 862, 865, 866, 871, 874, 879, 881,
            887, 887, 897, 906, 907, 923, 939, 941, 968, 971, 980, 985];
        assert_eq!(vr, expected);
    }

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
        use crate::primitive::par_filter_v2;
        let arr: Vec<i32> = vec![1, 2, 3, 1, 2];
        let actual: Vec<i32> = par_filter_v1(&arr, |_i: usize, a: &i32| -> bool { *a < 3 });
        let actual2: Vec<i32> = par_filter_v2(&arr, |_i: usize, a: &i32| -> bool { *a < 3 });

        let expected: Vec<i32> = vec![1, 2, 1, 2];
        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
        println!("actual2={:?}, expected={:?}", actual2, expected);
        assert_eq!(actual2, expected);
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
    fn par_flatten() {
        use crate::primitive::par_flatten;
        let arr: &mut Vec<i32> = &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let arr2: &mut Vec<i32> = &mut vec![15, 16];
        let arr3: &mut Vec<i32> = &mut vec![1, 2];
        let arr4: &mut Vec<i32> = &mut vec![1, 2, 3, 4, 5, 6, 7, 8];
        let arr5: &mut Vec<i32> = &mut vec![1, 2, 10, 20];
        let actual = par_flatten(&vec![&arr, &arr2, &arr3, &arr4, &arr5]);

        let expected: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 10, 20];

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

    #[test]
    fn par_quick_sort() {
        use crate::sort::par_quick_sort;
        let arr: Vec<i32> = vec![1, 7, 16, 0, -4, -7, 2, 3, 64, -1, 9, 1];
        let arr2: Vec<(i32, f32)> = vec![(1, 0.2), (2, 0.1), (-1, 9.0), (2, 0.1), (2, 0.2)];
        let actual = par_quick_sort(&arr, |a: &i32, b: &i32| -> i32 { *a - *b });
        let actual2 = par_quick_sort(&arr2, |a: &(i32, f32), b: &(i32, f32)| -> i32 {
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

