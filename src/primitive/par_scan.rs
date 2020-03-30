use crate::constant::*;
use super::utils::*;
use super::vec::*;
use super::scan::*;
use super::reduce::*;
use std::cmp::min;

fn par_scan_util<T, U>(seq: &mut [T], ret: &mut [T], func: &U, offset: &T) -> T
    where T: Sync + Send + Copy  ,
          U: Sync + Send + Fn(&T, &T) -> T
{
    let n: usize = seq.len();
    let l: usize = num_blocks(n, BLOCK_SIZE);
    if l <= 2 {
        return scan(seq, ret, func, offset);
    }
    let sums: &mut [T] = &mut vec_init(l, &|i: usize| {
        let s = i * BLOCK_SIZE;
        let e = min((i+1) * BLOCK_SIZE, seq.len());
        reduce(&seq[s..e], func)
    }, 1);
//    let (sums1, sums2) = no_split(sums);
    let mut tmp = vec_no_init(l);
    let total = scan(&sums, &mut tmp, func, offset);
    double_sliced_for(seq, ret, seq.len(), BLOCK_SIZE, &|ss, sr, i: usize, s, e| {
        scan(&ss[s..e], &mut sr[s..e], func, &tmp[i]);
    });
    return total;
}

pub fn par_scan<T, U>(seq: &mut [T], func: U , offset: &T) -> (Vec<T>, T)
    where T: Sync + Send + Copy  ,
          U: Sync + Send + Fn(&T, &T) -> T
{
    let mut ret = vec_no_init(seq.len());
    let tot = par_scan_util(seq, &mut ret, &func, offset);
    (ret, tot)
}

pub fn par_scan_inplace<T, U>(seq: &mut [T], func: U , offset: &T) -> T
    where T: Sync + Send + Copy ,
          U: Sync + Send + Fn(&T, &T) -> T
{
    let (seq1, seq2) = no_split(seq);
    par_scan_util(seq1, seq2, &func, offset)
}
