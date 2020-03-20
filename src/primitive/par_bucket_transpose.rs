use serde::export::fmt::{Debug, Display};

use crate::primitive::*;

// const GRANULARITY: usize = 2048;
// const BLOCK_THRESHOLD: usize = 8;

fn transpose<T>(from: &[T], to: &mut [T], counts: &Vec<usize>, dest_offsets: &Vec<usize>, block_size: usize,
                num_blocks: usize, num_buckets: usize, s: usize, e: usize)
    where T: Send + Sync + Copy + Display + Debug
{
    for i in s..e {
        let mut s_offset = i * block_size;
        for j in 0..num_buckets {
            let mut d_offset = dest_offsets[i + num_blocks * j];
            let len = counts[i * num_buckets + j];
            for _ in 0..len {
                to[d_offset] = from[s_offset];
                d_offset += 1;
                s_offset += 1;
            }
        }
    }
}

pub fn par_transpose_buckets<T>(from: &mut [T], to: &mut [T], counts: &Vec<usize>,
                                n: usize, block_size: usize, num_blocks: usize, num_buckets: usize) -> Vec<usize>
    where T: Send + Sync + Copy + Display + Debug
{
    let m = num_blocks * num_buckets;
    let block_bits = ((num_blocks as f64).log2().ceil()) as usize;
    let block_mask = num_blocks - 1;
    assert_eq!(1 << block_bits, num_blocks);

    let dest_offsets: Vec<usize> = {
        let tmp: Vec<usize> = vec_init(m, &|i, _| counts[(i >> block_bits) + num_buckets * (i & block_mask)]);
        let (new_tmp, sum) = par_scan(&tmp, |a: &usize, b: &usize| { *a + *b }, &0);
        assert_eq!(sum, n);
        new_tmp
    };

    transpose(from, to, counts, &dest_offsets, block_size, num_blocks, num_buckets, 0, num_blocks);

    let mut bucket_offsets = vec_no_init(num_buckets + 1);
    for i in 0..num_buckets {
        bucket_offsets[i] = dest_offsets[i * num_blocks];
    }
    // last element is the total size n
    bucket_offsets[num_buckets] = n;
    return bucket_offsets;
}
