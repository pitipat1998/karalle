extern crate rayon;

use rayon::prelude::*;

use super::map;
use super::scan;

pub fn filter<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
    where T: Copy,
          U: Fn(usize, &T) -> bool
{
    let mut mapped: Vec<i32> = map(seq, &|i: usize, elt: &T| -> i32 { if func(i, elt) {1} else {0}});
    let (x, tot): (Vec<i32>, i32) = scan(&mapped,
                                         &|elt1: &i32, elt2: &i32| -> i32 { *elt1 + *elt2 },
                                         &0);
    let mut ret = Vec::with_capacity(tot as usize);
    unsafe { ret.set_len(tot as usize) }

    for i in 0..mapped.len() {
        if mapped[i] == 1 {
            ret[x[i] as usize] = seq[i];
        }
    }
    ret
}