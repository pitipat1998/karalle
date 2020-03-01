mod plus_scan;
use self::plus_scan::scan;

mod map;
pub use self::map::map;

mod filter;
pub use self::filter::filter;

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
}
