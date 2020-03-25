use serde::export::fmt::{Debug, Display};
use crate::primitive::{no_split, par_for};
use crate::constant::*;

fn par_block_transpose_util<T>(from: &[T], to: &mut [T], source_offsets: &[usize], dest_offsets: &[usize], rs: usize, rc: usize, rl: usize, cs: usize, cc: usize, cl: usize)
    where T: Send + Sync + Copy + Display + Debug
{
    if cc * rc < TRANS_THRESHOLD * 16 {
        par_for(to, rs, rs + rc, &|ts, i| {
            for j in cs..(cs+cc) {
                let sa = source_offsets[i * rl + j];
                let sb = dest_offsets[j * cl + i];
                let l = source_offsets[i * rl + j + 1] - sa;
                for k in 0..l {
                    ts[k + sb] = from[k + sa];
                }
            }
        }, 1);
    } else if cc > rc {
        let l1 = cc / 2;
        let l2 = cc - l1;
        let (to1, to2) = no_split(to);

        rayon::join(
            || par_block_transpose_util(from, to1, source_offsets, dest_offsets, rs, rc, rl, cs, l1, cl),
            || par_block_transpose_util(from, to2, source_offsets, dest_offsets, rs, rc, rl, cs + l1, l2, cl)
        );
    } else {
        let l1 = cc / 2;
        let l2 = rc - l1;
        let (to1, to2) = no_split(to);
        rayon::join(
            || par_block_transpose_util(from, to1, source_offsets, dest_offsets, rs, l1, rl, cs, cc, cl),
            || par_block_transpose_util(from, to2, source_offsets, dest_offsets, rs + l1, l2, rl, cs, cc, cl)
        );
    }
}

pub fn par_block_transpose<T>(from: &[T], to: &mut[T], source_offsets: &[usize], dest_offsets: &[usize], rc: usize, cc: usize)
    where T: Send + Sync + Copy + Display + Debug
{
    par_block_transpose_util(from , to, source_offsets, dest_offsets, 0, rc, cc, 0, cc, rc);
}
