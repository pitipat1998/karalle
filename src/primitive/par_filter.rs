extern crate rayon;

use rayon::prelude::*;

use super::par_map;
use super::par_scan;

pub fn par_filter<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(usize, &T) -> bool
{
    let mapped: Vec<i32> = par_map(seq, &|i: usize, elt: &T| -> i32 { if func(i, elt) {1} else {0}});
    let (x, tot): (Vec<i32>, i32) = par_scan(&mapped,
                                         &|elt1: &i32, elt2: &i32| -> i32 { *elt1 + *elt2 },
                                         &0);
    let mut ret = Vec::with_capacity(tot as usize);
    unsafe { ret.set_len(tot as usize) }

    rayon::scope(|s| {
        for i in 0..mapped.len() {
            if mapped[i] == 1 {
                ret[x[i] as usize] = seq[i];
            }
        }
    });
    ret
}