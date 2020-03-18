extern crate rayon;

use num::PrimInt;
use rand::{distributions::Uniform, Rng};
use rayon::prelude::*;
use crate::primitive::{par_flatten, vec_zeroes, vec_init, vec_no_init};
use crate::sort::par_quick_sort_v2;
use crate::primitive::par_transpose_buckets;
use crate::primitive::par_copy;
use rand::prelude::ThreadRng;
use std::cmp::min;
use serde::export::fmt::{Display, Debug};

const QS_THRESHOLD: usize = 2000;
const GRANULARITY: usize = 2000;
const BUCKET_QUOTIENT: usize = 8;
const BLOCK_QUOTIENT: usize = 8;
const OVER_SAMPLE: usize = 8;

fn count<T, U>(seq: &mut [T], pivots: &[T], c: &mut [usize], block_size: usize, num_buckets: usize, s: usize, e: usize, func: &U)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let n = e - s;
    if n <= 64 {
        let seq_chunks = seq.chunks_mut(block_size);
        let c_chunks = c.chunks_mut(num_buckets);
        for (seq_chunk, c_chunk) in seq_chunks.zip(c_chunks) {
//            println!("seq_chunk={:?}, c_chunk={:?}", seq_chunk.len(), c_chunk.len());
            seq_chunk.sort_unstable_by(|a, b| func(a, b).cmp(&0));
            merge_seq(seq_chunk, pivots, c_chunk, func);
        }
    } else {
        let half = n / 2;
        let seq_mid = half * block_size;
        let c_mid = half * num_buckets;
//        println!("seq={}, seq_mid={}, c={}, c_mid={}, s={}, e={}", seq.len(), seq_mid, c.len(), c_mid, s, e);
        let (seq_l, seq_r) = seq.split_at_mut(seq_mid);
        let (c_l, c_r) = c.split_at_mut(c_mid);
        rayon::join(
            || count(seq_l, pivots, c_l, block_size, num_buckets, s, s+half, func),
            || count(seq_r, pivots, c_r, block_size, num_buckets, s+half, e, func)
        );
    }
}

fn merge_seq<T, U>(sa: &[T], sb: &[T], sc: &mut [usize], func: &U)
    where T: Sync + Send + Copy + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    if sa.len() == 0 || sb.len() == 0 { return; }
    let mut ia = 0;
    let mut ib = 0;
    let mut ic = 0;
    loop {
        while func(&sa[ia], &sb[ib]) < 0 {
            sc[ic] += 1;
            ia += 1;
            if ia == sa.len() { return; }
        }
        ib += 1;
        ic += 1;
        if ib == sb.len() { break; }
        if func(&sb[ib - 1], &sb[ib]) >= 0 {
            while func(&sb[ib], &sa[ia]) >= 0 {
                sc[ic] += 1;
                ia += 1;
                if ia == sa.len() { return; }
            }
            ib += 1;
            ic += 1;
            if ib == sb.len() { break; }
        }
    }
    sc[ic] = sa.len() - ia;
}

fn sort_within_bucket<T, U>(seq: &mut [T], pivots: &[T], bucket_offsets: &[usize], num_buckets: usize,
                            func: &U, s: usize, e: usize, seq_s: usize, seq_e: usize)
    where T: Copy + Sync + Send + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let n = e - s;
    if n <= 8 {
        for i in 0..n {
            let j = i+s;
            let start = bucket_offsets[j] - seq_s;
            let end = bucket_offsets[j + 1] - seq_s;

            if j == 0 || j == num_buckets - 1 || func(&pivots[j - 1], &pivots[j]) < 0 {
                (&mut seq[start..end]).sort_unstable_by(|a, b| func(a, b).cmp(&0));
            }
        }
    } else {
        let half = n / 2;
        let mid = bucket_offsets[half + s];
        let (l, r) = seq.split_at_mut(mid - seq_s);
        rayon::join(
            || sort_within_bucket(l, pivots, bucket_offsets, num_buckets, func, s, half+s, seq_s, mid),
            || sort_within_bucket(r, pivots, bucket_offsets, num_buckets, func, half+s, e, mid, seq_e)
        );
    }
}

fn par_sample_sort_util<T, U>(seq: &mut [T], aux: &mut [T], func: &U)
    where T: Copy + Sync + Send + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let n = seq.len();
    if n <= QS_THRESHOLD {
        seq.sort_unstable_by(|a, b| func(a, b).cmp(&0));
    } else {
        let sqrt = ((n as f64).sqrt().ceil()) as usize;
        let num_blocks = 1 << ((((sqrt / BLOCK_QUOTIENT) + 1) as f64).log2().ceil() as usize);
        let block_size = ((n - 1) / num_blocks) + 1;
        let num_buckets = (sqrt / BUCKET_QUOTIENT) + 1;
        let sample_set_size = num_buckets * OVER_SAMPLE;
        let m = num_blocks * num_buckets;
        let pivots = {
            let mut samples: Vec<T> = vec_init(sample_set_size, &|_, rng| seq[rng.gen_range(0, seq.len())]);
            par_quick_sort_v2(&mut samples, func);
            vec_init(num_buckets-1, &|i, _| samples[i * OVER_SAMPLE])
        };

        let mut counts: Vec<usize> = vec_zeroes(m+1);
        counts[m] = 0;
        count(seq, &pivots, &mut counts, block_size, num_buckets, 0, num_blocks, func);

        let bucket_offsets: Vec<usize> = par_transpose_buckets(seq, aux, &counts, seq.len(), block_size, num_blocks, num_buckets);
        sort_within_bucket(aux, &pivots, &bucket_offsets, num_buckets, func, 0, num_buckets, 0, aux.len());
        par_copy(seq, aux);
    }
}

#[allow(dead_code)]
pub fn par_sample_sort<T, U>(seq: &mut Vec<T>, func: &U)
    where T: Copy + Sync + Send + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
let mut aux: Vec<T> = Vec::with_capacity(seq.len());
    let n = seq.len();
    par_sample_sort_util(seq, &mut vec_no_init(n), func);
}