use std::cmp::min;
use serde::export::fmt::{Display, Debug};
use crate::primitive::{vec_init, vec_no_init, par_scan_inplace, no_split};
use rand::prelude::ThreadRng;
use crate::constant::*;

pub fn num_blocks(n: usize, block_size: usize) -> usize {
    if n == 0 {
        0
    }
    else {
        (1 + ((n)-1) / (block_size))
    }
}

pub fn par_for<T, U>(seq: &mut [T], s: usize, e: usize, f: &U, granularity: usize)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&mut [T], usize)
{
    let n = e - s;
    if n <= granularity {
        for i in s..e {
            f(seq, i);
        }
    } else {
        let m: usize = n / 2;
        let (seq1, seq2) = no_split(seq);
        rayon::join(
            || par_for(seq1, s, s+m, f, granularity),
            || par_for(seq2, s+m, e, f, granularity)
        );
    }
}

fn double_sliced_for_util<T, V, U>(seq1: &mut [T], seq2: &mut [V], len: usize, block_size: usize, s: usize, e: usize, f: &U)
    where T: Sync + Send + Copy + Display + Debug,
          V: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&mut [T], &mut [V], usize, usize, usize)
{
    let n = e - s;
    if n <= 1 {
        for i in s..e {
            let start = i * block_size;
            let end = min((i+1) * block_size, len);
            f(seq1, seq2, i, start, end);
        }
    } else {
        let m: usize = n / 2;
        let (seq1_1, seq1_2) = no_split(seq1);
        let (seq2_1, seq2_2) = no_split(seq2);
        rayon::join(
            || double_sliced_for_util(seq1_1, seq2_1, len, block_size, s, s+m, f),
            || double_sliced_for_util(seq1_2, seq2_2, len, block_size, s+m, e, f),
        );
    }
}

pub fn double_sliced_for<T, V, U>(seq1: &mut [T], seq2: &mut [V], len: usize, block_size: usize, f: &U)
    where T: Sync + Send + Copy + Display + Debug,
          V: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&mut [T], &mut [V], usize, usize, usize)
{
    let l = num_blocks(len, block_size);
    double_sliced_for_util(seq1, seq2, len, block_size, 0, l, f);
}

fn single_sliced_for_util<T, U>(seq: &mut [T], len: usize, block_size: usize, s: usize, e: usize, f: &U)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&mut [T], usize, usize, usize)
{
    let n = e - s;
    if n <= 1 {
        for i in s..e {
            let start = i * block_size;
            let end = min((i+1) * block_size, len);
            f(seq, i, start, end);
        }
    } else {
        let m: usize = n / 2;
        let (seq1, seq2) = no_split(seq);
        rayon::join(
            || single_sliced_for_util(seq1 , len, block_size, s, s+m, f),
            || single_sliced_for_util(seq2, len, block_size, s+m, e, f),
        );
    }
}

pub fn single_sliced_for<T, U>(seq: &mut [T], len: usize, block_size: usize, f: &U)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&mut [T], usize, usize, usize)
{
    let l = num_blocks(len, block_size);
    single_sliced_for_util(seq,  block_size, len, 0, l, f);
}

fn par_copy_util<T: Sync + Send + Copy + Display + Debug>(to: &mut [T], from: &[T], s: usize, e: usize) {
    let n = e - s;
    if n <= GRANULARITY {
        for i in s..e {
            let j = i - s;
            to[j] = from[i];
        }
    }
    else {
        let m: usize = n / 2;
        let (to_l, to_r) = to.split_at_mut(m);
        rayon::join(
            || par_copy_util(to_l, from, s, s+m),
            || par_copy_util(to_r, from, s+m, e)
        );
    }
}

pub fn par_copy<T: Sync + Send + Copy + Display + Debug>(to: &mut [T], from: &[T]) {
    if to.len() < from.len() {
        println!("par_copy: destination vector is less than source vector");
    }
    assert!(to.len() >= from.len());
    par_copy_util(to, from, 0, from.len());
}

fn split_three_util<T>(seq: &[T], ret: &mut[T], fl: &[usize], sums0: &[usize], sums1: &[usize],
                       m0: usize, m1: usize, block_size: usize, s: usize, e: usize)
    where T: Sync + Send + Copy + Display + Debug
{
    let n = e - s;
    if n <= 1 {
        let start = s * block_size;
        let end = min((s+1) * block_size, seq.len());
        let mut c0: usize = sums0[s];
        let mut c1: usize = m0 + sums1[s];
        let mut c2: usize = m0 + m1 + (start - sums0[s] - sums1[s]);
        for j in start..end {
            if fl[j] == 0 {
                ret[c0] = seq[j];
                c0 += 1;
            }
            else if fl[j] == 1 {
                ret[c1] = seq[j];
                c1 += 1;
            }
            else {
                ret[c2] = seq[j];
                c2 += 1;
            }
        }
    } else {
        let m: usize = n / 2;
        let (ret1, ret2) = no_split(ret);
        rayon::join(
            || split_three_util(seq, ret1, fl, sums0, sums1, m0, m1, block_size, s, s+m),
            || split_three_util(seq, ret2,fl, sums0, sums1, m0, m1, block_size, s+m, e),
        );
    }
}

fn split_three<T>(seq: &[T], ret: &mut [T], fl: &[usize]) -> (usize, usize)
    where T: Sync + Send + Copy + Display + Debug
{
    let l = num_blocks(seq.len(), BLOCK_SIZE);
    let sums0 : &mut [usize] = &mut vec_no_init(l);
    let sums1: &mut [usize] = &mut vec_no_init(l);
    double_sliced_for(sums0, sums1, seq.len(),BLOCK_SIZE, &|s0, s1, i, s, e, | {
        let mut c0: usize = 0;
        let mut c1: usize = 0;
        for j in s..e {
            if fl[j] == 0 {
                c0 += 1;
            } else if fl[j] == 1 {
                c1 += 1;
            }
        }
        s0[i] = c0;
        s1[i] = c1;
    });
    let m0: usize = par_scan_inplace(sums0, &|a: &usize, b: &usize| { *a + *b }, &0);
    let m1: usize = par_scan_inplace(sums1, &|a: &usize, b: &usize| { *a + *b }, &0);
    split_three_util(seq, ret, fl, &sums0, &sums1, m0, m1, BLOCK_SIZE, 0, l);
    (m0, m1)
}

fn sort5<T, U>(seq: &mut [T], f: &U)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let size : usize = 5;
    let m = seq.len() / (size + 1);
    for l in 0..size {
        seq.swap(l, m * (l + 1));
    }
    (&mut seq[0..size]).sort_by(|a, b| f(a, b).cmp(&0));
}

pub fn p_split3<T, U>(seq: &mut [T], ret: &mut [T], f: &U) -> (usize, usize, bool)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let n: usize = seq.len();
    sort5(seq, f);
    let mut p1 = seq[1];
    let mut p2 = seq[3];
    if f(&seq[0], &seq[1]) >= 0 {
        p1 = p2; // if few elements less than p1, then set to p2
    }
    if f(&seq[3], &seq[4]) >= 0 {
        p2 = p1;  // if few elements greater than p2, then set to p1
    }
    let flag = |i: usize, _: &mut ThreadRng| { if f(&seq[i], &p1) < 0 { 0 } else if f(&p2, &seq[i]) < 0 { 2 } else { 1 } };
    let (m0, m1) = split_three(seq, ret, &vec_init(n, &flag, GRANULARITY));
    (m0, m1, f(&p1, &p2) >= 0)
}

