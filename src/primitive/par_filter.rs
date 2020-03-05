extern crate rayon;

use rayon::prelude::*;

use super::*;

const THRESHOLD: usize = 100;

fn par_filter_utils<T, U>(seq: &Vec<T>, ret: &mut [T], mapped: &Vec<i32>, x: &Vec<i32>, func: &U, s: usize, e: usize)
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(usize, &T) -> bool
{
    let n = e - s;
    if n <= THRESHOLD {
        for i in s..e {
            if mapped[i] == 1 {
                ret[x[i] as usize] = seq[i];
            }
        }
    } else {
        let sqrt: usize = (n as f64).sqrt().ceil() as usize;
        let num_chunks: usize = ((n as f64) / (sqrt as f64)).ceil() as usize;

        rayon::scope(|s| {
            for (i, chunk) in ret.chunks_mut(sqrt).enumerate() {
                if i < num_chunks-1 {
                    s.spawn(move |_| {
                        par_filter_utils(
                            seq,
                            chunk,
                            mapped,
                            x,
                            func,
                            i * sqrt,
                            (i + 1) * sqrt
                        );
                    });
                } else {
                    s.spawn(move |_| {
                        par_filter_utils(
                            seq,
                            chunk,
                            mapped,
                            x,
                            func,
                            i * sqrt,
                            seq.len()
                        );
                    });
                }
            }
        })
    }
}

pub fn par_filter<T, U>(seq: &Vec<T>, func: U) -> Vec<T>
    where T: Sync + Send + Copy,
          U: Sync + Send + Fn(usize, &T) -> bool
{
    let mapped: Vec<i32> = par_map_v1(seq, &|i: usize, elt: &T| -> i32 { if func(i, elt) {1} else {0}});
    let (x, tot): (Vec<i32>, i32) = par_scan(&mapped,
                                         &|elt1: &i32, elt2: &i32| -> i32 { *elt1 + *elt2 },
                                         &0);
    let mut ret: Vec<T> = Vec::with_capacity(tot as usize);
    unsafe { ret.set_len(tot as usize) }

    par_filter_utils(seq, &mut ret, &mapped, &x, &func, 0, seq.len());
    ret
}