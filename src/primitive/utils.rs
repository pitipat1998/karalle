use std::cmp::min;
use serde::export::fmt::{Display, Debug};
use crate::primitive::{vec_init, vec_no_init, par_scan_inplace};
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

pub fn par_for<U>(s: usize, e: usize, f: &U, granularity: usize)
    where U: Sync + Send + Fn(usize)
{
    let n = e - s;
    if n <= granularity {
        for i in s..e {
            f(i);
        }
    } else {
        let m: usize = n / 2;
        rayon::join(
            || par_for(s, m, f, granularity),
            || par_for(m, e, f, granularity)
        );
    }
}

fn sliced_for_util<T, U>(seq1: &[T], seq2: &mut [T], block_size: usize, s: usize, e: usize, f: &U)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(usize, &[T], &mut [T])
{
    let n = e - s;
    if n <= 1 {
        let seq_chunks = seq1.chunks(block_size);
        let ret_chunks = seq2.chunks_mut(block_size);
        for (i, (seq_chunk, ret_chunk)) in seq_chunks.zip(ret_chunks).enumerate() {
            f(i+s, seq_chunk, ret_chunk);
        }
    } else {
        let m: usize = n / 2;
        let (seq_l, seq_r) = seq1.split_at(m * block_size);
        let (ret_l, ret_r) = seq2.split_at_mut(m * block_size);
        rayon::join(
            || sliced_for_util(seq_l, ret_l, block_size, s, s+m, f),
            || sliced_for_util(seq_r, ret_r, block_size, s+m, e, f),
        );
    }
}

pub fn sliced_for<T, U>(seq1: &[T], seq2: &mut [T], block_size: usize, f: &U)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(usize, &[T], &mut [T])
{
    let l = num_blocks(seq1.len(), block_size);
    sliced_for_util(seq1, seq2, block_size, 0, l, f);
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
    par_copy_util(to, from, 0, from.len());
}

fn split_three_util<T>(seq: &[T], fl: &[usize], block_size: usize, s: usize, e: usize) -> (Vec<T>, Vec<T>, Vec<T>)
    where T: Sync + Send + Copy + Display + Debug
{
    let n = e - s;
    if n <= 1 {
        let start = s * block_size;
        let end = min((s+1) * block_size, seq.len());
        let len = end - start;
        let mut c0: usize = 0;
        let mut c1: usize = 0;
        let mut c2: usize = 0;
        for j in start..end {
            if fl[j] == 0 {
                c0 += 1;
            }
            else if fl[j] == 1 {
                c1 += 1;
            }
            else {
                c2 += 1;
            }
        }
        let mut sp0: Vec<T> = vec_no_init(c0);
        let mut sp1: Vec<T> = vec_no_init(c1);
        let mut sp2: Vec<T> = vec_no_init(c2);
        let mut c0: usize = 0;
        let mut c1: usize = 0;
        let mut c2: usize = 0;
        for j in start..end {
            if fl[j] == 0 {
                sp0[c0] = seq[j];
                c0 += 1;
            }
            else if fl[j] == 1 {
                sp1[c1] = seq[j];
                c1 += 1;
            }
            else {
                sp2[c2] = seq[j];
                c2 += 1;
            }
        }
        (sp0, sp1, sp2)
    } else {
        let m: usize = n / 2;
        let ((mut l0, mut l1, mut l2), (r0, r1, r2)) = rayon::join(
            || split_three_util(seq, fl, block_size, s, s+m),
            || split_three_util(seq, fl, block_size, s+m, e),
        );
        l0.extend(r0);
        l1.extend(r1);
        l2.extend(r2);
        (l0, l1, l2)
    }
}

fn split_three<T>(seq: &[T], ret: &mut [T], fl: &[usize]) -> (usize, usize)
    where T: Sync + Send + Copy + Display + Debug
{
    let l = num_blocks(seq.len(), BLOCK_SIZE);
    let (sums0, m0): (Vec<usize>, usize) = {
        let mut tmp: Vec<usize> = vec_init(l, &|i, _| {
            let s = i * BLOCK_SIZE;
            let e = min((i + 1) * BLOCK_SIZE, seq.len());
            let mut c0: usize = 0;
            for j in s..e {
                if fl[j] == 0 {
                    c0 += 1;
                }
            }
            c0
        }, 1);
        let tot = par_scan_inplace(&mut tmp, &|a: &usize, b: &usize| { *a + *b }, &0);
        (tmp, tot)
    };
    let (sums1, m1): (Vec<usize>, usize) = {
        let mut tmp = vec_init(l, &|i, _| {
            let s = i * BLOCK_SIZE;
            let e = min((i + 1) * BLOCK_SIZE, seq.len());
            let mut c1: usize = 0;
            for j in s..e {
                if fl[j] == 1 {
                    c1 += 1;
                }
            }
            c1
        }, 1);
        let tot = par_scan_inplace(&mut tmp, &|a: &usize, b: &usize| { *a + *b }, &0);
        (tmp, tot)
    };
    let (sp0, sp1, sp2) = split_three_util(seq, fl, BLOCK_SIZE, 0, l);
    let (ret0, ret_rest) = ret.split_at_mut(m0);
    let (ret1, ret2) = ret_rest.split_at_mut(m1);
    rayon::join(
        || {
            rayon::join(
                || par_copy(ret0, &sp0),
                || par_copy(ret1, &sp1)
            )
        },
        || par_copy(ret2, &sp2)
    );
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