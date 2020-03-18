extern crate rayon;

use super::map;
use super::scan;
use crate::primitive::vec_no_init;

pub fn filter<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
    where T: Copy,
          U: Fn(usize, &T) -> bool
{
    let mapped: Vec<i32> = map(seq, &|i: usize, elt: &T| -> i32 { if func(i, elt) {1} else {0}});
    let (x, tot): (Vec<i32>, i32) = scan(&mapped,
                                         &|elt1: &i32, elt2: &i32| -> i32 { *elt1 + *elt2 },
                                         &0);
    let mut ret = vec_no_init(tot as usize);

    for i in 0..mapped.len() {
        if mapped[i] == 1 {
            ret[x[i] as usize] = seq[i];
        }
    }
    ret
}