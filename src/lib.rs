mod iter;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::iter::scan;
        let arr: &mut Vec<usize> = &mut vec![1,2,1,1];
        let f: &dyn Fn(usize, usize) -> usize = &|a: usize, b: usize| -> usize { a + b };
        let s: usize = 0;
        let (pref, tot): (Vec<usize>, usize) = scan(arr, f, s);
        println!("pref={:?}, tot={:?}", pref, tot);
        assert_eq!(1 + 1, 2)
    }
}

