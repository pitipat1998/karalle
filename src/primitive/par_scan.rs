use crate::constant::*;
use super::utils::*;
use super::vec::*;
use super::scan::*;
use super::reduce::*;
use std::cmp::min;
use serde::export::fmt::{Display, Debug};

fn par_scan_util<T, U>(seq: &[T], ret: &mut [T], func: &U, offset: &T) -> T
    where T: Sync + Send + Copy + Display + Debug + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> T
{
    let n: usize = seq.len();
    let l: usize = num_blocks(n, BLOCK_SIZE);
    if l <= 2 {
        return scan(seq, ret, func, offset);
    }
    let mut sums: Vec<T> = vec_init(l, &|i: usize, _| {
        let s = i * BLOCK_SIZE;
        let e = min((i+1) * BLOCK_SIZE, seq.len());
        reduce(&seq[s..e], func)
    }, 1);
    let mut tmp = vec_no_init(l);
    let total = scan(&sums, &mut tmp, func, offset);
    sliced_for(seq, ret, BLOCK_SIZE, &|i: usize, s_chunk: &[T], r_chunk: &mut [T]| {
        scan(s_chunk, r_chunk, func, &tmp[i]);
    });
    return total;
}

pub fn par_scan<T, U>(seq: &[T], func: U , offset: &T) -> (Vec<T>, T)
    where T: Sync + Send + Copy + Display + Debug + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> T
{
    let mut ret = vec_no_init(seq.len());
    let tot = par_scan_util(seq, &mut ret, &func, offset);
    (ret, tot)
}

pub fn par_scan_inplace<T, U>(seq: &mut [T], func: U , offset: &T) -> T
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> T
{
    let (ret, tot) = par_scan(&seq, func, offset);
    par_copy(seq, &ret);
    tot

}
