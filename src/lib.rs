mod iter;
#[cfg(test)]
mod tests {
    #[test]
    fn plus_scan() {
        use crate::iter::scan;
        let arr: &mut Vec<usize> = &mut vec![1,2,1,1];
        let f: &dyn Fn(usize, usize) -> usize = &|a: usize, b: usize| -> usize { a + b };
        let s: usize = 0;
        let (actual_arr, actual_tot): (Vec<usize>, usize) = scan(arr, f, s);

        let expected_arr: Vec<usize> = vec![0, 1, 3, 4];
        let expected_tot: usize = 5;

        println!("actual={:?}, expected={:?}", actual_arr, expected_arr);
        assert_eq!(actual_arr, expected_arr);
        println!("actual={:?}, expected={:?}", actual_tot, expected_tot);
        assert_eq!(actual_tot, expected_tot);
    }

    #[test]
    fn map() {
        use crate::iter::map;
        let arr: &mut Vec<usize> = &mut vec![1,2,3,1];
        let f: &dyn Fn(usize) -> usize = &|a: usize| -> usize { if a <= 2  {1} else {0} };
        let actual: Vec<usize> = map(arr, f);

        let expected: Vec<usize> = vec![1, 1, 0, 1];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn filter() {
        use crate::iter::filter;
        let arr: &mut Vec<usize> = &mut vec![1,2,3,1];
        let f: &dyn Fn(usize) -> bool = &|a: usize| -> bool {a < 3 };
        let actual: Vec<usize> = filter(arr, f);

        let expected: Vec<usize> = vec![1, 2, 1];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }
}

