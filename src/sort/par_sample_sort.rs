extern crate rayon;

use rand::{Rng};
use crate::primitive::{vec_zeroes, vec_init, vec_no_init, par_for, double_sliced_for};
use crate::sort::{par_quick_sort_v2};
use crate::primitive::par_buckets_transpose;
use crate::primitive::par_copy;
use serde::export::fmt::{Display, Debug};
use crate::sort::par_quick_sort_slice;
use crate::constant::*;
use std::cmp::min;

fn merge_seq<T, U>(sa: &[T], sb: &[T], sc: &mut [usize], func: &U)
    where T: Sync + Send + Copy + Display + Debug + Display + Debug,
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
            let mut samples: Vec<T> = vec_init(sample_set_size, &|_, rng| seq[rng.gen_range(0, seq.len())], GRANULARITY);
            par_quick_sort_v2(&mut samples, func);
            vec_init(num_buckets - 1, &|i, _| samples[i * OVER_SAMPLE], GRANULARITY)
        };

        let counts: &mut [usize] = &mut vec_zeroes(m + 1);
        counts[m] = 0;
        double_sliced_for(seq, counts, n, block_size, &|ss, cc, i, s, e|{
            let cs = i * num_buckets;
            let ce = min((i+1)*num_buckets, cc.len());
            (&mut ss[s..e]).sort_unstable_by(|a, b| func(a, b).cmp(&0));
            merge_seq(&mut ss[s..e], &pivots, &mut cc[cs..ce], func);
        });

        let bucket_offsets: Vec<usize> = par_buckets_transpose(seq, aux, &counts, seq.len(), block_size, num_blocks, num_buckets);
        par_for(aux, 0, num_buckets, &|au, i|{
            let start = bucket_offsets[i];
            let end = bucket_offsets[i + 1];

            if i == 0 || i == num_buckets - 1 || func(&pivots[i - 1], &pivots[i]) < 0 {
                par_quick_sort_slice(&mut au[start..end], func);
            }
        }, 1);
        par_copy(seq, aux);
    }
}

#[allow(dead_code)]
pub fn par_sample_sort<T, U>(seq: &mut Vec<T>, func: &U)
    where T: Copy + Sync + Send + Display + Debug,
          U: Sync + Send + Fn(&T, &T) -> i32
{
    let n = seq.len();
    par_sample_sort_util(seq, &mut vec_no_init(n), func);
}