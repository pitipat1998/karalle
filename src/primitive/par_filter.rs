extern crate rayon;

use crate::primitive::par_map::par_map_v3;
use crate::primitive::par_scan::par_scan;
use rayon::prelude::*;

const THRESHOLD: usize = 1000;

// TODO: more versions of filter

pub fn par_filter_v1<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(usize, &T) -> bool
{
    let mapped: Vec<i32> = par_map_v3(seq, &|i: usize, elt: &T| -> i32 { if func(i, elt) {1} else {0}});
    par_filter_util_v1(seq, &mapped, &func)
}

//pub fn par_filter_v2<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
//    where T: Sync + Send + Copy,
//          U: Sync + Send + Fn(usize, &T) -> bool
//{
//    let mapped: Vec<i32> = par_map_v3(seq, &|i: usize, elt: &T| -> i32 { if func(i, elt) {1} else {0}});
//    let (x, tot): (Vec<i32>, i32) = par_scan(&mapped,
//                                             &|elt1: &i32, elt2: &i32| -> i32 { *elt1 + *elt2 },
//                                             &0);
//    let mut ret: Vec<T> = Vec::with_capacity(tot as usize);
//    unsafe { ret.set_len(tot as usize) }
//
//    par_filter_util_v2(seq, &mut ret, &mapped, &x, &func);
//    ret
//}

pub fn par_filter_v3<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
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

//fn par_filter_util_v2<T, U>(seq: &[T], ret: &mut [T], mapped: &[i32], x: &[i32], func: &U)
//    where T: Sync + Send + Copy,
//          U: Sync + Send + Fn(usize, &T) -> bool
//{
//    rayon::scope(|s| {
//        for (i, chunk) in ret.chunks_mut(1).enumerate() {
//            s.spawn(move |_| {
//                if mapped[i] == 1 {
//                    chunk[0] = seq[i];
//                }
//            });
//        }
//    })
//}
//
