extern crate rayon;
use rayon::prelude::*;
use std::fmt::Debug;

const THRESHOLD: usize = 100;

fn par_map_recurse<T: Sync + Send, U: Sync + Send, W: (FnMut(usize, &T) -> U) + Sync + Send>(seq: &mut Vec<T>, func: &mut W, ret: &mut Vec<U>, start: usize, stop: usize) {
    let n = stop-start;
    println!("start={}, stop={}", start, stop);
    if n <= THRESHOLD {
        for i in start..stop {
            ret[i] = func(i, &seq[i]);
        }
    } else {
        let half: usize = (stop - start) / 2;
        par_map_recurse(seq, func, ret,start, half);
        par_map_recurse(seq, func, ret, half + 1, stop);
    }
}

pub fn par_map<T: Sync + Send + Copy, U: Sync + Send + Copy + Debug, W: (FnMut(usize, &T) -> U) + Sync + Send>(seq: &mut Vec<T>, func: &mut W) -> Vec<U> {
    let mut ret: Vec<U> = Vec::with_capacity(seq.len());
    unsafe { ret.set_len(seq.len()) }
    par_map_recurse(seq, func, &mut ret,0, seq.len());
    ret
}