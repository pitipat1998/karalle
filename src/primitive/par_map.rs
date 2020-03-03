extern crate rayon;
use rayon::prelude::*;

const THRESHOLD: usize = 100;

fn par_map_recurse<T: Sync, U: Sync + Send>(seq: &mut Vec<T>, func: &dyn Fn(usize, &T) -> U, ret: &mut Vec<U>, start: usize, stop: usize) {
    let n = stop-start;
    if n <= THRESHOLD {
        (start..stop).into_par_iter().for_each(|i| {
            ret[i] = func(i, &seq[i])
        })
    } else {
        let half: usize = (stop - start) / 2;
        par_map_recurse(seq, func, ret,start, half);
        par_map_recurse(seq,func,ret, half + 1, stop);
    }
}

pub fn par_map<T: Sync, U: Sync + Send>(seq: &mut Vec<T>, func: &dyn Fn(usize, &T) -> U) -> Vec<U> {
    let mut ret: Vec<U> = Vec::new();
    unsafe { ret.set_len(seq.len()) }
    par_map_recurse(seq, func, &mut ret,0, seq.len());
    ret
}