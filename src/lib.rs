mod primitive;
mod sort;
#[cfg(test)]
mod tests {
    #[test]
    fn plus_scan() {
        use crate::primitive::scan;
        let arr: &mut Vec<i32> = &mut vec![1,2,1,1];
        let f: &dyn Fn(i32, i32) -> i32 = &|a: i32, b: i32| -> i32 { a + b };
        let s: i32 = 0;
        let (actual_arr, actual_tot): (Vec<i32>, i32) = scan(arr, f, s);

        let expected_arr: Vec<i32> = vec![0, 1, 3, 4];
        let expected_tot: i32 = 5;

        println!("actual={:?}, expected={:?}", actual_arr, expected_arr);
        assert_eq!(actual_arr, expected_arr);
        println!("actual={:?}, expected={:?}", actual_tot, expected_tot);
        assert_eq!(actual_tot, expected_tot);
    }

    #[test]
    fn map() {
        use crate::primitive::map;
        let arr: &mut Vec<i32> = &mut vec![1,2,3,1];
        let f: &dyn Fn(i32) -> i32 = &|a: i32| -> i32 { if a <= 2  {1} else {0} };
        let actual: Vec<i32> = map(arr, f);

        let expected: Vec<i32> = vec![1, 1, 0, 1];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn filter() {
        use crate::primitive::filter;
        let arr: &mut Vec<i32> = &mut vec![1,2,3,1];
        let f: &dyn Fn(i32) -> bool = &|a: i32| -> bool {a < 3 };
        let actual: Vec<i32> = filter(arr, f);

        let expected: Vec<i32> = vec![1, 2, 1];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }


    #[test]
    fn qsort() {
        use crate::sort::qsort;
        let arr: &mut Vec<i32> = &mut vec![1,7,16,0,-4,-7,2,3,64,-1,9,1];
        let actual: Vec<i32> = qsort(arr, &|a: i32, b: i32| -> i32 { a-b });

        let expected: Vec<i32> = vec![-7,-4,-1,0,1,1,2,3,7,9,16,64];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }
}

