pub mod primitive;
mod sort;
mod util;
mod benchmark;
mod constant;

#[cfg(test)]
mod tests {
    use crate::primitive::*;
    use crate::util::*;
    use crate::constant::*;
    use std::process::exit;
    use crate::primitive::par_filter_v3;
    use rayon::prelude::*;
    use crate::util::data_generator::random_i32_list_generator;

    const LENGTH: u64 = 1000000;
    #[test]
    fn seq_sample_sort() {
        use crate::sort::seq_sample_sort;
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
        let arr: &Vec<i32> = &vec![1, 2, 1, 1];
        let ret: &mut Vec<i32> = &mut vec_no_init(arr.len());
        let actual_tot: i32 = scan(
            arr,
                ret,
            &|a: &i32, b: &i32| -> i32 { *a + *b },
            &0,
        );

        let expected_arr: &mut Vec<i32> = &mut vec![0, 1, 3, 4];
        let expected_tot: i32 = 5;

        assert_eq!(ret, expected_arr);
        assert_eq!(actual_tot, expected_tot);
    }

    #[test]
    fn par_scan() {
        use crate::primitive::*;
        let arr: &mut Vec<i32> = &mut random_i32_list_generator(LENGTH, -1000, 1001);
        let (actual_arr, actual_tot): (Vec<i32>, i32) = par_scan(
            arr,
            |a: &i32, b: &i32| -> i32 { *a + *b },
            &0,
        );
        let arr2 = &mut arr.clone();
        let actual_tot2: i32 = par_scan_inplace(
            arr2,
            |a: &i32, b: &i32| -> i32 { *a + *b },
            &0,
        );

        let mut expected_arr = vec_no_init(arr.len());
        let expected_tot = scan(&arr, &mut expected_arr, &|a: &i32, b: &i32| -> i32 { *a + *b }, &0);

        assert_eq!(actual_arr, expected_arr);
        assert_eq!(actual_tot, expected_tot);
        assert_eq!(arr2, &mut expected_arr);
        assert_eq!(actual_tot2, expected_tot);
    }

    #[test]
    fn map() {
        use crate::primitive::*;
        let arr: &mut Vec<i32> = &mut random_i32_list_generator(LENGTH, -1000, 1001);
        let actual: Vec<i32> = map(arr, &|_i: usize, a: &i32| -> i32 { if *a <= 2 { 1 } else { 0 } });

        let expected: Vec<i32> = arr.iter().map(|x| { if *x <= 2 {1} else {0}}).collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn par_map() {
        use crate::primitive::*;
        let arr: &mut Vec<i32> = &mut random_i32_list_generator(LENGTH, -1000, 1001);

        let actual: Vec<i32> = par_map_v1(&arr, &|_i: usize, a: &i32| -> i32 { if *a <= 90 { 1 } else { 0 } });
        let actual2: Vec<i32> = par_map_v2(&arr, &|_i: usize, a: &i32| -> i32 { if *a <= 90 { 1 } else { 0 } });
        let actual3: Vec<i32> = par_map_v3(&arr, &|_i: usize, a: &i32| -> i32 { if *a <= 90 { 1 } else { 0 } });
//
        let expected: Vec<i32> = par_map_v5(&arr, &|_i, x| { if *x <= 90 {1} else {0}});

        assert_eq!(actual, expected);
        assert_eq!(actual2, expected);
        assert_eq!(actual3, expected);
    }

    #[test]
    fn filter() {
        use crate::primitive::*;
        let arr: &mut Vec<i32> = &mut random_i32_list_generator(LENGTH, -1000, 1001);
        let actual: Vec<i32> = filter(&arr, |_i: usize, a: &i32| -> bool { *a < 3 });

        let expected: Vec<i32> = par_filter_v3(arr, &|x, a| { *a < 3 });

        assert_eq!(actual, expected);
    }

    #[test]
    fn par_filter() {
        use crate::primitive::*;
        let arr: &mut Vec<i32> = &mut random_i32_list_generator(LENGTH, -1000, 1001);
        let actual: Vec<i32> = par_filter_v1(&arr, &|_i: usize, a: &i32| -> bool { *a < 3 });
        let actual2: Vec<i32> = par_filter_v2(&arr, &|_i: usize, a: &i32| -> bool { *a < 3 });

        let expected: Vec<i32> = par_filter_v3(&arr, &|_i: usize, a: &i32| -> bool { *a < 3 });
        assert_eq!(actual, expected);
        assert_eq!(actual2, expected);
    }

    #[test]
    fn flatten() {
        use crate::primitive::*;
        let arr: &mut Vec<i32> = &mut vec![1, 2, 3, 4];
        let arr2: &mut Vec<i32> = &mut vec![5, 6, 7, 8];
        let arr3: &mut Vec<i32> = &mut vec![9, 10, 11, 12];
        let actual = flatten(&vec![&arr, &arr2, &arr3]);

        let expected: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        assert_eq!(actual, expected);
    }

    #[test]
    fn par_flatten() {
        use crate::primitive::*;
        let arr1: Vec<i32> = random_i32_list_generator(LENGTH/2, -1000, 1001);
        let arr2: Vec<i32> = random_i32_list_generator(LENGTH/2, -1000, 1001);
        let arr3: Vec<i32> = random_i32_list_generator(LENGTH/2, -1000, 1001);
        let arr4: Vec<i32> = random_i32_list_generator(LENGTH/2, -1000, 1001);
        let arr5: Vec<i32> = random_i32_list_generator(LENGTH/2, -1000, 1001);

        let arr6: Vec<i32> = arr1.clone();
        let arr7: Vec<i32> = arr2.clone();
        let arr8: Vec<i32> = arr3.clone();
        let arr9: Vec<i32> = arr4.clone();
        let arr10: Vec<i32> = arr5.clone();

        let actual: Vec<i32> = par_flatten(&vec![arr1, arr2, arr3, arr4, arr5]);

        let expected: Vec<i32> = par_flatten_v2(&vec![arr6, arr7, arr8, arr9, arr10]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn quick_sort() {
        use crate::sort::*;
        let mut arr: &mut Vec<i32> = &mut random_i32_list_generator(LENGTH, -1000, 1001);
        let mut actual = &mut quick_sort(&arr, |a: &i32, b: &i32| -> i32 { *a - *b });

        par_quick_sort_v3(arr, &|a: &i32, b: &i32| -> i32 { *a - *b });

        assert_eq!(actual, arr);
    }

    #[test]
    fn par_quick_sort() {
        use crate::sort::*;
        let mut arr: &mut Vec<i32> = &mut random_i32_list_generator(LENGTH, -1000, 1001);
        let mut arr3: Vec<i32> = arr.clone();
        let mut arr5: Vec<i32> = arr.clone();
        let mut arr7: Vec<i32> = arr.clone();
        let actual = par_quick_sort(&arr, &|a: &i32, b: &i32| -> i32 { *a - *b });
        par_quick_sort_v2(&mut arr3, &|a: &i32, b: &i32| -> i32 { *a - *b });
        par_quick_sort_v3(&mut arr5, &|a: &i32, b: &i32| -> i32 { *a - *b });
        par_sample_sort(&mut arr7, &|a: &i32, b: &i32| -> i32 { *a - *b });

        assert_eq!(actual, arr5);
        assert_eq!(arr3, arr5);
        assert_eq!(arr7, arr5);
    }
}

