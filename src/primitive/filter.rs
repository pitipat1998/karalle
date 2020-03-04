extern crate rayon;

use rayon::prelude::*;

use super::map;
use super::scan;

pub fn filter<T: Copy>(seq: &mut Vec<T>, func: &dyn Fn(usize, &T) -> bool) -> Vec<T> {
    let mut mapped: Vec<i32> = map(seq, &|i: usize, elt: &T| -> i32 { if func(i, elt) {1} else {0}});
    let (_pref, _tot): (Vec<i32>, i32) = scan(&mut mapped,
                                              &|elt1: &i32, elt2: &i32| -> i32 { *elt1 + *elt2 },
                                              &0);
    let mut ret: Vec<T> = Vec::with_capacity(seq.capacity());
//    unsafe {ret.set_len(n)};
//    let n : usize = seq.len();

    for (i, elt) in mapped.iter().enumerate() {
        if *elt == 1 {
            ret.push(seq[i])
        }
    }
    ret
}