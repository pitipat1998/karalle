extern crate rayon;

use crate::primitive::*;
use crate::constant::*;
use rayon::prelude::*;


// TODO: more versions of filter
#[allow(dead_code)]
pub fn non_inplace_par_filter<T, U>(seq: &Vec<T>, func: &U) -> Vec<T>
    where T: Sync + Send + Copy ,
          U: Sync + Send + Fn(usize, &T) -> bool
{
    let mapped: Vec<i32> = rayon_par_map(seq, &|i: usize, elt: &T| -> i32 { if func(i, elt) {1} else {0}});
    par_filter_util_v1(seq, &mapped, &func)
}

pub fn par_filter<T, U>(seq: &Vec<T>, func: &U) -> Vec<T>
    where T: Sync + Send + Copy ,
          U: Sync + Send + (Fn(usize, &T) -> bool)
{
    let n = seq.len();
    let l = num_blocks(n, BLOCK_SIZE);
    let sums: &mut [usize] = &mut vec_no_init(l);
    let fl: &mut [bool] = &mut vec_no_init(n);
    double_sliced_for(fl, sums, n, BLOCK_SIZE, &|ff, ss, i,  s,  e| {
        let mut r: usize = 0;
        for  j in s..e {
            let b = func(j, &seq[j]);
            ff[j] = b;
            r += if b {1} else {0};
        }
        ss[i] = r;
    });
    let m = par_scan_inplace(sums, |a, b| { *a + *b }, &0);
    let mut ret: Vec<T> = vec_no_init(m);
    single_sliced_for(&mut ret, n, BLOCK_SIZE, &|rr, i,  s, e| {
        let ss = &seq[s..e];
        let ff = &fl[s..e];
        let end = if i == l-1 {m} else {sums[i+1]};
        let tmp_rr = &mut rr[sums[i]..end];
        let mut k: usize = 0;
        for j in 0..ss.len() {
            if ff[j] {
                tmp_rr[k] = ss[j];
                k += 1;
            }
        }
    });
    ret
}

#[allow(dead_code)]
pub fn rayon_par_filter<T, U>(seq: &[T], func: &U) -> Vec<T>
    where T: Sync + Send + Copy ,
          U: Sync + Send + (Fn(usize, &T) -> bool)
{
    seq.par_iter().filter_map(|x| if func(1, x) {Some(*x)} else {None}).collect()
}


fn par_filter_util_v1<T, U>(seq: &[T], mapped: &[i32], func: &U) -> Vec<T>
    where T: Sync + Send + Copy ,
          U: Sync + Send + Fn(usize, &T) -> bool
{
    if seq.len() <= GRANULARITY {
        let mut ret = Vec::new();
        for i in 0..seq.len() {
            if mapped[i] == 1 {
                ret.push(seq[i]);
            }
        }
        ret
    } else {
        let half: usize = seq.len()/2;
        let (seq_l, seq_r) = seq.split_at(half);
        let (mapped_l, mapped_r) = mapped.split_at(half);
        let mut l: Vec<T> = vec![];
        let mut r: Vec<T> = vec![];

        rayon::join(
            || { l = par_filter_util_v1(seq_l, mapped_l, func); },
            || { r = par_filter_util_v1(seq_r, mapped_r, func); }
        );
        l.extend(r);
        l
    }
}

