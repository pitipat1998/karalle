mod primitive;
mod sort;
#[cfg(test)]
mod tests {
    #[test]
    fn map() {
        use crate::primitive::map;
        let arr: &mut Vec<i32> = &mut vec![1,2,3,1];
        let f: &dyn Fn(usize, &i32) -> i32 = &|_i:usize, a: &i32| -> i32 { if *a <= 2  {1} else {0} };
        let actual: Vec<i32> = map(arr, f);

        let expected: Vec<i32> = vec![1, 1, 0, 1];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn filter() {
        use crate::primitive::filter;
        let arr: &mut Vec<i32> = &mut vec![1,2,3,1];
        let f: &dyn Fn(usize, &i32) -> bool = &|_i:usize, a: &i32| -> bool {*a < 3 };
        let actual: Vec<i32> = filter(arr, f);

        let expected: Vec<i32> = vec![1, 2, 1];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }


    #[test]
    fn qsort() {
        use crate::sort::qsort;
        let arr: &mut Vec<i32> = &mut vec![1,7,16,0,-4,-7,2,3,64,-1,9,1];
        let arr2: &mut Vec<(i32, f32)> = &mut vec![(1, 0.2), (2, 0.1), (-1, 9.0), (2, 0.1), (2, 0.2)];
        let actual: Vec<i32> = qsort(arr, &|a: &i32, b: &i32| -> i32 { *a-*b });
        let actual2: Vec<(i32, f32)> = qsort(arr2, &|a: &(i32, f32), b: &(i32, f32)| -> i32 {
            let (a1, a2): (i32, f32) = *a;
            let (b1, b2): (i32, f32) = *b;
            if a1 == b1 {
                if a2 < b2 {-1}
                else if a2 == b2 {0}
                else {0}
            }
            else {
                return a1 - b1;
            }
        });

        let expected: Vec<i32> = vec![-7,-4,-1,0,1,1,2,3,7,9,16,64];
        let expected2: Vec<(i32, f32)> = vec![(-1, 9.0), (1, 0.2), (2, 0.1), (2, 0.1), (2, 0.2)];

        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
        println!("actual2={:?}, expected2={:?}", actual2, expected2);
        assert_eq!(actual2, expected2);
    }
}

