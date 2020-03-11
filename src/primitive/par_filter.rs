extern crate rayon;

use crate::primitive::par_map::par_map_v3;
use crate::primitive::par_scan::par_scan;
use serde::export::fmt::Debug;

const THRESHOLD: usize = 1;

// TODO: more versions of filter
#[allow(dead_code)]
pub fn par_filter_v1<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(usize, &T) -> bool
{
    let mapped: Vec<i32> = par_map_v3(seq, &|i: usize, elt: &T| -> i32 { if func(i, elt) {1} else {0}});
    par_filter_util_v1(seq, &mapped, &func)
}

pub fn par_filter_v2<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(usize, &T) -> bool
{
    let mapped: Vec<usize> = par_map_v3(seq, &|i: usize, elt: &T| -> usize { if func(i, elt) {1} else {0}});
    let (x, tot): (Vec<usize>, usize) = par_scan(&mapped,
                                             &|elt1: &usize, elt2: &usize| -> usize { *elt1 + *elt2 },
                                             &0);
    let mut ret: Vec<T> = Vec::with_capacity(tot);
    unsafe { ret.set_len(tot) }

    par_filter_util_v2(seq, &mut ret, &mapped, &x, 0);
    ret
}

#[allow(dead_code)]
pub fn par_filter_v3<T, U>(seq: &Vec<T>, _func: U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(usize, &T) -> bool
{
//    seq.par_iter().filter(|x| func(1, x)).collect()
    seq.clone()
}


fn par_filter_util_v1<T, U>(seq: &[T], mapped: &[i32], func: &U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(usize, &T) -> bool
{
    if seq.len() <= THRESHOLD {
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

pub fn par_filter_util_v2<T>(seq: &[T], ret: &mut [T], mapped: &[usize], x: &[usize], idx: usize)
    where T: Sync + Send + Copy
{
    if seq.len() <= THRESHOLD {
        for i in 0..seq.len() {
            if mapped[i] == 1 {
                println!("x[i]={}, idx={}", x[i], idx);
                ret[x[i]-idx] = seq[i];
            }
        }
    } else {
        let half: usize = seq.len()/2;
        // let ret_half = ret.len()/2;
        let (seq_l, seq_r) = seq.split_at(half);
        let (mapped_l, mapped_r) = mapped.split_at(half);
        let (x_l, x_r) = x.split_at(half);
        let x_half = x_r[0];
        let (ret_l, ret_r) = ret.split_at_mut(x_half-idx);

        rayon::join(
            || { par_filter_util_v2(seq_l, ret_l, mapped_l, x_l, idx); },
            || { par_filter_util_v2(seq_r, ret_r, mapped_r, x_r,  x_half); }
        );
    }
}

