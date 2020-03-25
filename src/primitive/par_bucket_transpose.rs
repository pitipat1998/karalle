use serde::export::fmt::{Debug, Display};

use crate::primitive::*;
use crate::constant::*;

fn par_oblivious_buckets_transpose<T>(from: &mut [T], to: &mut [T], counts: &[usize],
                                          n: usize, _block_size: usize, num_blocks: usize,
                                          num_buckets: usize) -> Vec<usize>
    where T: Send + Sync + Copy + Display + Debug
{
    let m = num_buckets * num_blocks;
    let source_offsets: &mut [usize] = &mut {
        let mut tmp: Vec<usize> = vec_no_init(m+1);
        par_copy(&mut tmp, counts);
        tmp
    };
    let dest_offsets : &mut [usize] = &mut vec_no_init(m);
    par_transpose(counts, dest_offsets, num_blocks, num_buckets);

    let total = par_scan_inplace(dest_offsets, |a, b| { *a + *b }, &0);
    assert_eq!(total, n);
    let total = par_scan_inplace(source_offsets, |a, b| { *a + *b }, &0);
    assert_eq!(total, n);
    source_offsets[m] = n;

    par_block_transpose(from, to, source_offsets, dest_offsets, num_blocks, num_buckets);

    let mut bucket_offsets = vec_no_init(num_buckets + 1);
    for i in 0..num_buckets {
        bucket_offsets[i] = dest_offsets[i * num_blocks];
    }
    bucket_offsets[num_buckets] = n;
    bucket_offsets
}

fn par_non_oblivious_buckets_transpose<T>(from: &mut [T], to: &mut [T], counts: &[usize],
                                          n: usize, block_size: usize, num_blocks: usize,
                                          num_buckets: usize) -> Vec<usize>
    where T: Send + Sync + Copy + Display + Debug
{
    let m = num_blocks * num_buckets;
    let block_bits = ((num_blocks as f64).log2().ceil()) as usize;
    let block_mask = num_blocks - 1;
    assert_eq!(1 << block_bits, num_blocks);

    let dest_offsets: &mut [usize] = &mut vec_init(m, &|i, _| counts[(i >> block_bits) + num_buckets * (i & block_mask)], GRANULARITY);
    let sum = par_scan_inplace(dest_offsets, |a: &usize, b: &usize| { *a + *b }, &0);
    assert_eq!(sum, n);

    par_for(to, 0, num_blocks, &|ts, i| {
        let mut s_offset = i * block_size;
        for j in 0..num_buckets {
            let mut d_offset = dest_offsets[i + num_blocks * j];
            let len = counts[i * num_buckets + j];
            for _ in 0..len {
                ts[d_offset] = from[s_offset];
                d_offset += 1;
                s_offset += 1;
            }
        }
    }, 1);

    let mut bucket_offsets = vec_no_init(num_buckets + 1);
    for i in 0..num_buckets {
        bucket_offsets[i] = dest_offsets[i * num_blocks];
    }
    bucket_offsets[num_buckets] = n;
    bucket_offsets
}

pub fn par_buckets_transpose<T>(from: &mut [T], to: &mut [T], counts: &[usize],
                            n: usize, block_size: usize, num_blocks: usize,
                            num_buckets: usize) -> Vec<usize>
    where T: Send + Sync + Copy + Display + Debug
{
    if n < (1 << 22) || num_buckets <= 512 || num_blocks <= 512 {
        par_non_oblivious_buckets_transpose(from ,to, counts, n, block_size, num_blocks, num_buckets)
    } else {
        par_oblivious_buckets_transpose(from, to, counts, n, block_size, num_blocks, num_buckets)
    }
}
